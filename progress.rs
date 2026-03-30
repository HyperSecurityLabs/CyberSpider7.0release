pub use crate::cyberwave_progress::*;
use indicatif::ProgressBar;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub struct ProgressManager {
    pub theme: String,
    cyberwave: CyberWaveProgress,
    current_bar: Arc<AsyncMutex<Option<ProgressBar>>>,
}

impl ProgressManager {
    pub fn new(theme: &str) -> Self {
        let cyberwave_theme = match theme {
            "matrix" => ProgressTheme::Matrix,
            "neon" => ProgressTheme::Neon,
            "terminal" => ProgressTheme::Terminal,
            _ => ProgressTheme::CyberWave,
        };
        
        Self {
            theme: theme.to_string(),
            cyberwave: CyberWaveProgress::new(cyberwave_theme),
            current_bar: Arc::new(AsyncMutex::new(None)),
        }
    }

    pub fn display_rust_logo() {
        let cyberwave = CyberWaveProgress::new(ProgressTheme::CyberWave);
        cyberwave.display_cyberwave_logo();
    }

    pub fn create_spinner(&self) -> ProgressBar {
        self.cyberwave.create_spinner("Processing...")
    }

    pub fn create_bar(&self, length: u64) -> ProgressBar {
        self.cyberwave.create_progress_bar(length, "Processing...")
    }

    pub fn create_dynamic_bar(&self, initial_total: u64, message: &str) -> ProgressBar {
        let bar = self.cyberwave.create_progress_bar(initial_total, message);
        
        // Store reference for dynamic updates
        let current_bar = self.current_bar.clone();
        let bar_clone = bar.clone();
        tokio::spawn(async move {
            let mut guard = current_bar.lock().await;
            *guard = Some(bar_clone);
        });
        
        bar
    }

    pub async fn update_progress(&self, current: u64, total: u64, message: &str) {
        let guard = self.current_bar.lock().await;
        if let Some(bar) = guard.as_ref() {
            bar.set_length(total);
            bar.set_position(current);
            bar.set_message(message.to_string());
        }
    }

    pub async fn increment_progress(&self, increment: u64) {
        let guard = self.current_bar.lock().await;
        if let Some(bar) = guard.as_ref() {
            bar.inc(increment);
        }
    }

    pub async fn finish_current_bar(&self) {
        let guard = self.current_bar.lock().await;
        if let Some(bar) = guard.as_ref() {
            bar.finish();
        }
    }

    pub fn create_crawl_progress_bar(&self, total_urls: u64, current_depth: usize) -> ProgressBar {
        let message = format!("Crawling depth {} - 0/{} URLs", current_depth, total_urls);
        let bar = self.cyberwave.create_progress_bar(total_urls, &message);
        
        // Store reference for dynamic updates
        let current_bar = self.current_bar.clone();
        let bar_clone = bar.clone();
        tokio::spawn(async move {
            let mut guard = current_bar.lock().await;
            *guard = Some(bar_clone);
        });
        
        bar
    }

    pub async fn update_crawl_progress(&self, processed: u64, total: u64, discovered: u64, current_depth: usize) {
        let message = format!("Crawling depth {} - {}/{} URLs ({} discovered)", 
            current_depth, processed, total, discovered);
        let guard = self.current_bar.lock().await;
        if let Some(bar) = guard.as_ref() {
            bar.set_length(total);
            bar.set_position(processed);
            bar.set_message(message);
        }
    }

    // Public methods for cyberwave visualization
    pub fn display_discovery_alert(&self, url_count: usize, source: &str) {
        self.cyberwave.display_discovery_alert(url_count, source);
    }

    pub fn display_error_alert(&self, error: &str, url: &str) {
        self.cyberwave.display_error_alert(error, url);
    }

    pub fn display_scanning_status(&self, current_url: &str, total_urls: usize, processed_urls: usize, depth: usize) {
        self.cyberwave.display_scanning_status(current_url, total_urls, processed_urls, depth);
    }
}
