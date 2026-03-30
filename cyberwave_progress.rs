use std::time::Duration;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use console::Term;
use crate::r#box::DiagonalBox;

pub struct CyberWaveProgress {
    theme: ProgressTheme,
    term: Term,
}

#[derive(Debug, Clone)]
pub enum ProgressTheme {
    CyberWave,
    Matrix,
    Neon,
    Terminal,
}

impl CyberWaveProgress {
    pub fn new(theme: ProgressTheme) -> Self {
        Self {
            theme,
            term: Term::stdout(),
        }
    }

    pub fn display_cyberwave_logo(&self) {
        // Load banner from file
        let banner_content = if std::path::Path::new("banner.txt").exists() {
            match std::fs::read_to_string("banner.txt") {
                Ok(content) => content.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
                Err(_) => {
                    // Fallback to default banner if file can't be read
                    self.get_default_banner().into_iter().map(|s| s.to_string()).collect()
                }
            }
        } else {
            // Fallback to default banner if file doesn't exist
            self.get_default_banner().into_iter().map(|s| s.to_string()).collect()
        };

        // Add CyberSpider branding below the banner
        let mut full_banner = banner_content.clone();
        
        // Add gap before branding
        full_banner.push("".to_string());
        full_banner.push("                    CYBERSPIDER v7.0 - HYPERSECURITY".to_string());
        full_banner.push("                    Advanced Security Reconnaissance".to_string());
        full_banner.push("                    Author: Khaninkali".to_string());

        for (i, line) in full_banner.iter().enumerate() {
            let original_len = self.get_default_banner().len();
            match self.theme {
                ProgressTheme::CyberWave => {
                    if i >= original_len {
                        // HYPERSECURITY branding lines - use red colors
                        println!("{}", line.bright_red().bold());
                    } else {
                        // Banner ASCII art - use enhanced blue colors
                        println!("{}", line.cyan().bold());
                    }
                }
                ProgressTheme::Matrix => {
                    if i >= original_len {
                        println!("{}", line.bright_green().bold());
                    } else {
                        println!("{}", line.green());
                    }
                }
                ProgressTheme::Neon => {
                    if i >= original_len {
                        println!("{}", line.bright_magenta().bold());
                    } else {
                        println!("{}", line.magenta().bold());
                    }
                }
                ProgressTheme::Terminal => {
                    if i >= original_len {
                        println!("{}", line.bright_white().bold());
                    } else {
                        println!("{}", line.white());
                    }
                }
            }
        }
        
        println!(); // Add one line break after banner
    }

    pub fn display_scanning_status(&self, current_url: &str, total_urls: usize, processed_urls: usize, depth: usize) {
        let percentage = if total_urls > 0 {
            (processed_urls as f64 / total_urls as f64) * 100.0
        } else {
            0.0
        };

        let status = match self.theme {
            ProgressTheme::CyberWave => {
                format!("⟦ CYBERSPIDER ⟧ Scanning: {} [{}/{} URLs] {:.1}% - Depth {}",
                    current_url.bright_white(),
                    processed_urls.to_string().bright_green(),
                    total_urls.to_string().bright_cyan(),
                    percentage,
                    depth.to_string().bright_yellow()
                )
            }
            ProgressTheme::Matrix => {
                format!("◈ Scanning: {} [{}/{}] {:.1}% - Depth {}",
                    current_url.white(),
                    processed_urls,
                    total_urls,
                    percentage,
                    depth
                )
            }
            ProgressTheme::Neon => {
                format!("◆ Scanning: {} [{}/{}] {:.1}% - Depth {}",
                    current_url.bright_white(),
                    processed_urls.to_string().magenta(),
                    total_urls.to_string().bright_magenta(),
                    percentage,
                    depth.to_string().yellow()
                )
            }
            ProgressTheme::Terminal => {
                format!("✓ Scanning: {} [{}/{}] {:.1}% - Depth {}",
                    current_url.white(),
                    processed_urls,
                    total_urls,
                    percentage,
                    depth
                )
            }
        };

        println!("{}", status);
    }

    pub fn display_discovery_alert(&self, url_count: usize, source: &str) {
        let alert = match self.theme {
            ProgressTheme::CyberWave => {
                format!("⟦ CYBERSPIDER ⟧ Discovered {} new URLs from {}",
                    url_count.to_string().bright_green().bold(),
                    source.bright_cyan()
                )
            }
            ProgressTheme::Matrix => {
                format!("◈ Discovered {} URLs from {}",
                    url_count,
                    source
                )
            }
            ProgressTheme::Neon => {
                format!("◆ Discovered {} URLs from {}",
                    url_count.to_string().magenta(),
                    source.bright_magenta()
                )
            }
            ProgressTheme::Terminal => {
                format!("✓ Discovered {} URLs from {}",
                    url_count,
                    source
                )
            }
        };

        println!("{}", alert);
    }

    pub fn display_error_alert(&self, error: &str, url: &str) {
        let alert = match self.theme {
            ProgressTheme::CyberWave => {
                format!("⟦ CYBERSPIDER ⟧ Failed to crawl {}: {}",
                    url.bright_red(),
                    error.red()
                )
            }
            ProgressTheme::Matrix => {
                format!("☠ Failed to crawl {}: {}",
                    url.red(),
                    error.red()
                )
            }
            ProgressTheme::Neon => {
                format!("◉ Failed to crawl {}: {}",
                    url.bright_red(),
                    error.red()
                )
            }
            ProgressTheme::Terminal => {
                format!("✗ Failed to crawl {}: {}",
                    url.red(),
                    error.red()
                )
            }
        };

        println!("{}", alert);
    }

    fn get_default_banner(&self) -> Vec<&'static str> {
        vec![
            "   ______      __              _____       _     __         _    _______",
            "  / ____/_  __/ /_  ___  _____/ ___/____  (_)___/ /__  ____| |  / /__  /",
            " / /   / / / / __ \\/ _ \\/ ___/\\__ \\/ __ \\/ / __  / _ \\/ ___/ | / /  / / ",
            "/ /___/ /_/ / /_/ /  __/ /   ___/ / /_/ / / /_/ /  __/ /   | |/ /  / /  ",
            "\\____/\\__, /_.___/\\___/_/   /____/ .___/_/\\__,_/\\___/_/    |___/  /_/   ",
            "     /____/                     /_/                                      ",
        ]
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        
        let style = match self.theme {
            ProgressTheme::CyberWave => ProgressStyle::with_template(
                "⟦ CYBERSPIDER ⟧  {spinner}  {msg}"
            )
            .unwrap()
            .tick_strings(&[
                "▰▰▰▰▱",
                "▰▰▰▱▰",
                "▰▰▱▰▰",
                "▰▱▰▰▰",
                "▱▰▰▰▰",
                "▰▱▰▰▰",
                "▰▰▱▰▰",
                "▰▰▰▱▰",
            ]),
            ProgressTheme::Matrix => ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["◐", "◓", "◑", "◒"]),
            ProgressTheme::Neon => ProgressStyle::default_spinner()
                .template("{spinner:.magenta} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            ProgressTheme::Terminal => ProgressStyle::default_spinner()
                .template("{spinner:.white} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        };

        pb.set_style(style);
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message(message.to_string());
        pb
    }

    pub fn create_progress_bar(&self, total: u64, message: &str) -> ProgressBar {
        let pb = ProgressBar::new(total);
        
        let style = match self.theme {
            ProgressTheme::CyberWave => ProgressStyle::with_template(
                "⟦ CYBERSPIDER ⟧ [{elapsed_precise}] [{bar:40.bright_green/blue}] {pos}/{len} {msg}"
            )
            .unwrap()
            .progress_chars("▰▰▰▰>>"),
            ProgressTheme::Matrix => ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("▓▓░░"),
            ProgressTheme::Neon => ProgressStyle::default_bar()
                .template("{spinner:.magenta} [{elapsed_precise}] [{bar:40.magenta/white}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("■□▫░"),
            ProgressTheme::Terminal => ProgressStyle::default_bar()
                .template("{spinner:.white} [{elapsed_precise}] [{bar:40.white/black}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("=>- "),
        };
        
        pb.set_style(style);
        pb.set_message(message.to_string());
        pb
    }

    pub fn print_success(&self, message: &str) {
        match self.theme {
            ProgressTheme::CyberWave => {
                let box_lines = DiagonalBox::create_diagonal_success_box(message);
                for line in box_lines {
                    println!("{}", line);
                }
            },
            ProgressTheme::Matrix => println!("◈ {}", message.green()),
            ProgressTheme::Neon => println!("◆ {}", message.magenta().bold()),
            ProgressTheme::Terminal => println!("✓ {}", message.white()),
        }
    }

    pub fn print_error(&self, message: &str) {
        match self.theme {
            ProgressTheme::CyberWave => {
                let box_lines = DiagonalBox::create_diagonal_error_box(message);
                for line in box_lines {
                    println!("{}", line);
                }
            },
            ProgressTheme::Matrix => println!("☠ {}", message.red().bold()),
            ProgressTheme::Neon => println!("◉ {}", message.red().bold()),
            ProgressTheme::Terminal => println!("✗ {}", message.red()),
        }
    }

    pub fn print_warning(&self, message: &str) {
        match self.theme {
            ProgressTheme::CyberWave => {
                let box_lines = DiagonalBox::create_diagonal_warning_box(message);
                for line in box_lines {
                    println!("{}", line);
                }
            },
            ProgressTheme::Matrix => println!("⚡ {}", message.yellow()),
            ProgressTheme::Neon => println!("⚡ {}", message.yellow().bold()),
            ProgressTheme::Terminal => println!("⚠ {}", message.yellow()),
        }
    }

    pub fn print_info(&self, message: &str) {
        match self.theme {
            ProgressTheme::CyberWave => println!("ℹ {}", message.cyan().bold()),
            ProgressTheme::Matrix => println!("◉ {}", message.cyan()),
            ProgressTheme::Neon => println!("○ {}", message.cyan().bold()),
            ProgressTheme::Terminal => println!("ℹ {}", message.cyan()),
        }
    }

    pub fn display_stats(&self, stats: &CrawlStats) {
        let total_str = stats.total_requests.to_string();
        let success_str = stats.successful_requests.to_string();
        let failed_str = stats.failed_requests.to_string();
        let urls_str = stats.urls_discovered.to_string();
        let subdomains_str = stats.subdomains_found.to_string();
        let s3_str = stats.s3_buckets_found.to_string();
        let duration_str = stats.duration_ms.to_string();
        let rps_str = format!("{:.2}", stats.requests_per_second);
        
        let stats_data = vec![
            ("Total Requests", total_str.as_str()),
            ("Successful", success_str.as_str()),
            ("Failed", failed_str.as_str()),
            ("URLs Discovered", urls_str.as_str()),
            ("Subdomains Found", subdomains_str.as_str()),
            ("S3 Buckets", s3_str.as_str()),
            ("Duration (ms)", duration_str.as_str()),
            ("Requests/sec", rps_str.as_str()),
        ];

        let box_lines = DiagonalBox::create_diagonal_stats_box(&stats_data);
        for line in box_lines {
            println!("{}", line);
        }
    }

    /// Display a boxed completion message based on theme
    pub fn display_completion_box(&self, message: &str) {
        let box_lines = match self.theme {
            ProgressTheme::CyberWave => DiagonalBox::create_double_diagonal_box(60, 5, Some("CYBERSPIDER COMPLETE"), &[message.to_string()]),
            ProgressTheme::Matrix => DiagonalBox::create_zigzag_box(60, 5, Some("SCAN COMPLETE"), &[message.to_string()]),
            ProgressTheme::Neon => DiagonalBox::create_mixed_diagonal_box(60, 5, Some("NEON SCAN COMPLETE"), &[message.to_string()]),
            ProgressTheme::Terminal => DiagonalBox::create_diagonal_box(60, 5, Some("TERMINAL COMPLETE"), &[message.to_string()]),
        };
        
        for line in box_lines {
            println!("{}", line);
        }
    }

    /// Display a boxed error message based on theme
    pub fn display_error_box(&self, message: &str) {
        let box_lines = match self.theme {
            ProgressTheme::CyberWave => DiagonalBox::create_zigzag_box(60, 5, Some("CYBERSPIDER ERROR"), &[message.to_string()]),
            ProgressTheme::Matrix => DiagonalBox::create_diagonal_box(60, 5, Some("MATRIX ERROR"), &[message.to_string()]),
            ProgressTheme::Neon => DiagonalBox::create_double_diagonal_box(60, 5, Some("NEON ERROR"), &[message.to_string()]),
            ProgressTheme::Terminal => DiagonalBox::create_mixed_diagonal_box(60, 5, Some("TERMINAL ERROR"), &[message.to_string()]),
        };
        
        for line in box_lines {
            println!("{}", line);
        }
    }

    /// Display a boxed warning message based on theme
    pub fn display_warning_box(&self, message: &str) {
        let box_lines = match self.theme {
            ProgressTheme::CyberWave => DiagonalBox::create_mixed_diagonal_box(60, 5, Some("CYBERSPIDER WARNING"), &[message.to_string()]),
            ProgressTheme::Matrix => DiagonalBox::create_double_diagonal_box(60, 5, Some("MATRIX WARNING"), &[message.to_string()]),
            ProgressTheme::Neon => DiagonalBox::create_zigzag_box(60, 5, Some("NEON WARNING"), &[message.to_string()]),
            ProgressTheme::Terminal => DiagonalBox::create_diagonal_box(60, 5, Some("TERMINAL WARNING"), &[message.to_string()]),
        };
        
        for line in box_lines {
            println!("{}", line);
        }
    }

    /// Display a boxed info message based on theme
    pub fn display_info_box(&self, message: &str) {
        let box_lines = match self.theme {
            ProgressTheme::CyberWave => DiagonalBox::create_diagonal_box(60, 5, Some("CYBERSPIDER INFO"), &[message.to_string()]),
            ProgressTheme::Matrix => DiagonalBox::create_mixed_diagonal_box(60, 5, Some("MATRIX INFO"), &[message.to_string()]),
            ProgressTheme::Neon => DiagonalBox::create_zigzag_box(60, 5, Some("NEON INFO"), &[message.to_string()]),
            ProgressTheme::Terminal => DiagonalBox::create_double_diagonal_box(60, 5, Some("TERMINAL INFO"), &[message.to_string()]),
        };
        
        for line in box_lines {
            println!("{}", line);
        }
    }
}

#[derive(Debug, Default)]
pub struct CrawlStats {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub urls_discovered: usize,
    pub subdomains_found: usize,
    pub s3_buckets_found: usize,
    pub duration_ms: u64,
    pub requests_per_second: f64,
}

impl CrawlStats {
    pub fn calculate_rps(&mut self) {
        if self.duration_ms > 0 {
            self.requests_per_second = (self.total_requests as f64) / (self.duration_ms as f64 / 1000.0);
        }
    }

    /// Display stats with a themed progress display
    pub fn display_with_progress(&self, progress: &CyberWaveProgress) {
        progress.display_stats(self);
    }

    /// Create a summary message for completion boxes
    pub fn create_summary_message(&self) -> String {
        format!(
            "Scan completed: {} URLs discovered, {} successful, {} failed in {}ms",
            self.urls_discovered,
            self.successful_requests,
            self.failed_requests,
            self.duration_ms
        )
    }
}
