use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use url::Url;

use crate::{SpiderConfig, DiscoveredUrl, progress::ProgressManager};
use super::url_extractor::UrlExtractor;
use crate::sources::ExternalSource;

/// Helper function for safe mutex locking
fn safe_mutex_lock<T>(mutex: &std::sync::Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>>
where
    T: std::fmt::Debug,
{
    mutex.lock().map_err(|e| anyhow::anyhow!("Mutex poisoned: {:?}", e))
}

pub struct Crawler {
    client: Client,
    config: Arc<SpiderConfig>,
    progress_manager: ProgressManager,
    url_extractor: UrlExtractor,
    wayback: crate::sources::wayback::WaybackMachine,
    commoncrawl: crate::sources::commoncrawl::CommonCrawl,
    virustotal: crate::sources::virustotal::VirusTotal,
    discovered_urls: Arc<std::sync::Mutex<Vec<DiscoveredUrl>>>,
    url_relationships: Arc<std::sync::Mutex<Vec<(String, String)>>>,
}

#[derive(Debug)]
pub struct CrawlResult {
    pub base_domain: String,
    pub discovered_urls: Vec<DiscoveredUrl>,
    pub subdomains: Vec<String>,
    pub s3_buckets: Vec<String>,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
}

impl Crawler {
    pub fn new(config: &SpiderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent("CyberSpider/7.0.0")
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;

        let config = Arc::new(config.clone());
        let progress_manager = ProgressManager::new(&config.progress_theme);
        let url_extractor = UrlExtractor::new(config.clone());
        
        // Initialize external sources
        let wayback = crate::sources::wayback::WaybackMachine::new();
        let commoncrawl = crate::sources::commoncrawl::CommonCrawl::new();
        let virustotal = crate::sources::virustotal::VirusTotal::new(None);

        Ok(Self {
            client,
            config,
            progress_manager,
            url_extractor,
            wayback,
            commoncrawl,
            virustotal,
            discovered_urls: Arc::new(std::sync::Mutex::new(Vec::new())),
            url_relationships: Arc::new(std::sync::Mutex::new(Vec::new())),
        })
    }

    pub async fn crawl_targets(&mut self, targets: Vec<String>) -> Result<CrawlResult> {
        let mut all_urls = HashSet::new();
        let mut processed_urls = HashSet::new();
        let mut subdomains = HashSet::new();
        let mut s3_buckets = HashSet::new();
        let mut total_requests = 0;
        let mut successful_requests = 0;
        let mut failed_requests = 0;

        for target in targets {
            let base_domain = self.extract_base_domain(&target)?;
            
            // Add initial target
            all_urls.insert(target.clone());
            let total_discovered = all_urls.len();
            
            // Create shared counter for total discovered URLs
            let total_discovered_arc = Arc::new(std::sync::Mutex::new(total_discovered));

            // Crawl with depth control
            for depth in 0..=self.config.depth {
                if depth == 0 {
                    if self.config.verbose {
                        println!("🎯 Starting crawl for: {}", target);
                    }
                    continue;
                }

                // Get URLs to process (only unprocessed ones)
                let urls_to_process: Vec<String> = all_urls.iter()
                    .filter(|url| !processed_urls.contains(*url))
                    .cloned()
                    .collect();
                
                if self.config.verbose {
                    println!("Depth {}: Processing {} new URLs ({} total discovered, {} already processed)", 
                        depth, urls_to_process.len(), total_discovered, processed_urls.len());
                }
                
                if urls_to_process.is_empty() {
                    if self.config.verbose {
                        println!("No new URLs to process at depth {}", depth);
                    }
                    continue;
                }
                
                let semaphore = Arc::new(Semaphore::new(self.config.concurrent));
                let processed_count = Arc::new(std::sync::Mutex::new(processed_urls.len()));
                
                // Create progress bar with dynamic total showing overall progress
                let total_discovered_count = safe_mutex_lock(&total_discovered_arc)?;
                let bar = self.progress_manager.create_crawl_progress_bar(*total_discovered_count as u64, depth);
                drop(total_discovered_count);
                
                let processed_count_value = safe_mutex_lock(&processed_count)?;
                let total_discovered_count2 = safe_mutex_lock(&total_discovered_arc)?;
                
                // Set initial progress
                bar.set_position(*processed_count_value as u64);
                bar.set_message(format!("Crawling depth {} - {}/{} URLs ({} discovered)", 
                    depth, *processed_count_value, *total_discovered_count2, *total_discovered_count2));
                
                drop(processed_count_value);
                drop(total_discovered_count2);

                let tasks: Vec<_> = urls_to_process.clone().into_iter().map(|url| {
                    let client = self.client.clone();
                    let config = self.config.clone();
                    let semaphore = semaphore.clone();
                    let bar = bar.clone();
                    let url_extractor = self.url_extractor.clone();
                    let discovered_urls = self.discovered_urls.clone();
                    let url_relationships = self.url_relationships.clone();
                    let total_discovered_arc = total_discovered_arc.clone();
                    let processed_count = processed_count.clone();

                    async move {
                        let _permit = semaphore.acquire().await
                            .map_err(|e| anyhow::anyhow!("Failed to acquire semaphore permit: {}", e))?;
                        
                        if config.delay > 0 {
                            tokio::time::sleep(Duration::from_secs(config.delay)).await;
                        }

                        let result = Self::crawl_url(&client, &url, &url_extractor, &config, discovered_urls, url_relationships).await;
                        
                        // Update progress bar to show current progress
                        {
                            let mut processed = safe_mutex_lock(&processed_count)?;
                            *processed += 1;
                            let total_discovered_guard = safe_mutex_lock(&total_discovered_arc)?;
                            
                            // Update bar with realistic progress
                            bar.set_length(*total_discovered_guard as u64);
                            bar.set_position(*processed as u64);
                            bar.set_message(format!("Crawling depth {} - {}/{} URLs ({} discovered)", 
                                depth, *processed, *total_discovered_guard, *total_discovered_guard));
                            drop(total_discovered_guard);
                        }
                        
                        result
                    }
                }).collect();

                let results = join_all(tasks).await;
                bar.finish();

                for (url, result) in urls_to_process.iter().zip(results) {
                    // Mark URL as processed
                    processed_urls.insert(url.clone());
                    
                    total_requests += 1;
                    match result {
                        Ok((discovered, new_subdomains, new_s3_buckets)) => {
                            successful_requests += 1;
                            
                            // Filter out already discovered URLs and add new ones
                            let initial_len = all_urls.len();
                            for url in discovered.iter() {
                                if !all_urls.contains(url) {
                                    all_urls.insert(url.clone());
                                }
                            }
                            let new_urls_count = all_urls.len() - initial_len;
                            
                            // Update total discovered count
                            {
                                let mut total = safe_mutex_lock(&total_discovered_arc)?;
                                *total += new_urls_count;
                            }
                            
                            subdomains.extend(new_subdomains);
                            s3_buckets.extend(new_s3_buckets);
                            
                            if self.config.verbose && new_urls_count > 0 {
                                self.progress_manager.display_discovery_alert(new_urls_count, "crawling");
                                println!("Found {} new unique URLs (total: {})", new_urls_count, all_urls.len());
                            }
                        }
                        Err(e) => {
                            failed_requests += 1;
                            if self.config.verbose {
                                self.progress_manager.display_error_alert(&e.to_string(), url);
                            }
                        }
                    }
                }
            }

            // Process external sources if enabled
            if self.config.other_sources_enabled {
                if self.config.verbose {
                    println!("Fetching URLs from external sources for domain: {}", base_domain);
                }
                
                // Real external sources implementation
                let domain = url::Url::parse(&base_domain)
                    .map(|u| u.domain().unwrap_or(&base_domain).to_string())
                    .unwrap_or_else(|_| base_domain.clone());
                
                // Fetch from Wayback Machine
                if self.config.verbose {
                    println!("Checking Wayback Machine...");
                }
                if let Ok(wayback_urls) = self.wayback.fetch_urls(&domain).await {
                    let initial_len = all_urls.len();
                    for url in &wayback_urls {
                        if !all_urls.contains(url) {
                            all_urls.insert(url.clone());
                        }
                    }
                    let new_urls_count = all_urls.len() - initial_len;
                    if self.config.verbose {
                        println!("Found {} URLs from Wayback Machine ({} new unique)", wayback_urls.len(), new_urls_count);
                    }
                    // Update total discovered count
                    {
                        let mut total = safe_mutex_lock(&total_discovered_arc)?;
                        *total += new_urls_count;
                    }
                } else if self.config.verbose {
                    println!("Failed to fetch from Wayback Machine");
                }
                
                // Fetch from Common Crawl
                if self.config.verbose {
                    println!("Checking Common Crawl...");
                }
                if let Ok(commoncrawl_urls) = self.commoncrawl.fetch_urls(&domain).await {
                    let initial_len = all_urls.len();
                    for url in &commoncrawl_urls {
                        if !all_urls.contains(url) {
                            all_urls.insert(url.clone());
                        }
                    }
                    let new_urls_count = all_urls.len() - initial_len;
                    if self.config.verbose {
                        println!("Found {} URLs from Common Crawl ({} new unique)", commoncrawl_urls.len(), new_urls_count);
                    }
                    // Update total discovered count
                    {
                        let mut total = safe_mutex_lock(&total_discovered_arc)?;
                        *total += new_urls_count;
                    }
                } else if self.config.verbose {
                    println!("Failed to fetch from Common Crawl");
                }
                
                // Fetch from VirusTotal (if API key is available)
                if self.config.verbose {
                    println!("Checking VirusTotal...");
                }
                if let Ok(virustotal_urls) = self.virustotal.fetch_urls(&domain).await {
                    let initial_len = all_urls.len();
                    for url in &virustotal_urls {
                        if !all_urls.contains(url) {
                            all_urls.insert(url.clone());
                        }
                    }
                    let new_urls_count = all_urls.len() - initial_len;
                    if self.config.verbose {
                        println!("Found {} URLs from VirusTotal ({} new unique)", virustotal_urls.len(), new_urls_count);
                    }
                    // Update total discovered count
                    {
                        let mut total = safe_mutex_lock(&total_discovered_arc)?;
                        *total += new_urls_count;
                    }
                } else if self.config.verbose {
                    println!("Failed to fetch from VirusTotal (no API key?)");
                }
                
                if self.config.verbose {
                    println!("Total unique URLs after external sources: {}", all_urls.len());
                }
            }

            // Extract final total_discovered value
            let final_total_discovered = *safe_mutex_lock(&total_discovered_arc)?;

            // Use real discovered URLs from the crawler
            let discovered_urls_final: Vec<DiscoveredUrl> = {
                let discovered = safe_mutex_lock(&self.discovered_urls)?;
                discovered.clone()
            };

            if self.config.verbose {
                println!("Crawl completed! Discovered {} unique URLs, processed {} URLs", 
                    final_total_discovered, processed_urls.len());
            }

            return Ok(CrawlResult {
                base_domain,
                discovered_urls: discovered_urls_final,
                subdomains: subdomains.into_iter().collect(),
                s3_buckets: s3_buckets.into_iter().collect(),
                total_requests,
                successful_requests,
                failed_requests,
            });
        }

        Err(anyhow::anyhow!("No targets to crawl"))
    }

    async fn crawl_url(
        client: &Client,
        url: &str,
        url_extractor: &UrlExtractor,
        config: &SpiderConfig,
        discovered_urls: Arc<std::sync::Mutex<Vec<DiscoveredUrl>>>,
        url_relationships: Arc<std::sync::Mutex<Vec<(String, String)>>>,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
        if config.verbose {
            println!("Crawling: {}", url);
        }
        
        let start_time = std::time::Instant::now();
        
        let response = match client.get(url).send().await {
            Ok(resp) => {
                if config.verbose {
                    println!("Response: {} - {} ({})", 
                        url, 
                        resp.status(), 
                        resp.status().canonical_reason().unwrap_or("Unknown"));
                }
                resp
            }
            Err(e) => {
                if config.verbose {
                    println!("Failed to fetch {}: {}", url, e);
                }
                return Err(e.into());
            }
        };
        
        let status_code = response.status().as_u16();
        let content_type = response.headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .map(|ct| ct.to_string());
        
        let content = match response.text().await {
            Ok(text) => {
                if config.verbose {
                    println!("Retrieved {} bytes from {}", text.len(), url);
                }
                text
            }
            Err(e) => {
                if config.verbose {
                    println!("Failed to read response body from {}: {}", url, e);
                }
                return Err(e.into());
            }
        };
        
        // Extract title from HTML content
        let title = if content_type.as_ref().map_or(false, |ct| ct.contains("text/html")) {
            extract_title_from_html(&content)
        } else {
            None
        };
        
        let discovered_urls_list = url_extractor.extract_urls(&content, url)?;
        let subdomains = url_extractor.extract_subdomains(&content)?;
        let s3_buckets = url_extractor.extract_s3_buckets(&content)?;
        
        // Store the discovered URL with real response data
        {
            let mut discovered = safe_mutex_lock(&discovered_urls)?;
            discovered.push(DiscoveredUrl {
                url: url.to_string(),
                source: "crawl".to_string(),
                status_code: Some(status_code),
                content_type: content_type.clone(),
                title,
                method: "GET".to_string(),
            });
        }
        
        // Store relationships (parent URL -> discovered URLs)
        {
            let mut relationships = safe_mutex_lock(&url_relationships)?;
            for discovered_url in &discovered_urls_list {
                relationships.push((url.to_string(), discovered_url.clone()));
            }
        }
        
        if config.verbose {
            let duration = start_time.elapsed();
            println!("⚡ Extracted {} URLs, {} subdomains, {} S3 buckets from {} in {:?}", 
                discovered_urls_list.len(), 
                subdomains.len(), 
                s3_buckets.len(), 
                url, 
                duration);
            
            if !discovered_urls_list.is_empty() && config.verbose {
                println!("   📍 Found URLs:");
                for (i, discovered_url) in discovered_urls_list.iter().take(5).enumerate() {
                    println!("     {}. {}", i + 1, discovered_url);
                }
                if discovered_urls_list.len() > 5 {
                    println!("     ... and {} more", discovered_urls_list.len() - 5);
                }
            }
        }

        Ok((discovered_urls_list, subdomains, s3_buckets))
    }

    fn extract_base_domain(&self, url: &str) -> Result<String> {
        let parsed_url = Url::parse(url)?;
        Ok(parsed_url.host_str().unwrap_or("unknown").to_string())
    }
}

// Helper function to extract title from HTML
fn extract_title_from_html(html: &str) -> Option<String> {
    use regex::Regex;
    
    let title_regex = Regex::new(r"<title[^>]*>(.*?)</title>").ok()?;
    
    if let Some(captures) = title_regex.captures(html) {
        if let Some(title_match) = captures.get(1) {
            let title = title_match.as_str().trim();
            if !title.is_empty() {
                // Simple HTML entity decoding for common entities
                let decoded = title
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"")
                    .replace("&#39;", "'");
                return Some(decoded);
            }
        }
    }
    
    None
}
