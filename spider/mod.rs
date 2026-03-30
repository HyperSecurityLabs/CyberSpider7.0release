pub mod crawler;
pub mod parser;
pub mod url_extractor;

use anyhow::Result;
use std::time::Instant;
use crate::{SpiderConfig, SpiderResult, progress::ProgressManager};

pub struct Spider {
    config: SpiderConfig,
    progress_manager: ProgressManager,
}

impl Spider {
    pub fn new(config: SpiderConfig) -> Self {
        let progress_manager = ProgressManager::new(&config.progress_theme);
        
        Self {
            config,
            progress_manager,
        }
    }

    pub async fn run(&mut self) -> Result<SpiderResult> {
        let start_time = Instant::now();
        
        // Get target URLs
        let targets = self.get_targets()?;
        
        if targets.is_empty() {
            return Err(anyhow::anyhow!("No valid targets specified"));
        }

        let spinner = self.progress_manager.create_spinner();
        spinner.set_message("Initializing spider...");
        
        // Create crawler
        let mut crawler = crawler::Crawler::new(&self.config)
            .map_err(|e| anyhow::anyhow!("Failed to initialize crawler: {}", e))?;
        
        // Start crawling directly (no extra message)
        let result = crawler.crawl_targets(targets).await?;
        
        let duration = start_time.elapsed();
        
        Ok(SpiderResult {
            base_domain: result.base_domain,
            discovered_urls: result.discovered_urls,
            subdomains: result.subdomains,
            s3_buckets: result.s3_buckets,
            total_requests: result.total_requests,
            successful_requests: result.successful_requests,
            failed_requests: result.failed_requests,
            duration_ms: duration.as_millis() as u64,
        })
    }

    fn get_targets(&self) -> Result<Vec<String>> {
        let mut targets = Vec::new();
        
        if let Some(site) = &self.config.site {
            targets.push(site.clone());
        }
        
        if let Some(sites_file) = &self.config.sites_file {
            let content = std::fs::read_to_string(sites_file)?;
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    targets.push(line.to_string());
                }
            }
        }
        
        Ok(targets)
    }
}
