# CyberSpider7.0release
 A powerful, multi-threaded web crawler with real-time visualization, distributed processing, and comprehensive security analysis capabilities.

## 🕷️ Overview

CyberSpider is a sophisticated reconnaissance tool designed for security professionals and penetration testers. It combines advanced crawling capabilities with real-time visualization, distributed processing, and comprehensive security analysis in a single, powerful package.

### 🔥 Key Features

- **🚀 High-Performance Crawling**: Multi-threaded, concurrent URL discovery with depth control
- **🎯 Real-Time Visualization**: Interactive graph visualization with .Dot export formats (DOT,) 
- **🌐 External Sources Integration**: Wayback Machine, Common Crawl, VirusTotal data aggregation
- **🔍 Security Analysis**: Built-in vulnerability scanning, API detection, and technology fingerprinting
- **📊 Advanced Metrics**: Real-time performance monitoring with percentile-based analysis
- **🔌 Plugin System**: Extensible architecture for custom detectors and processors
- **🌐 Distributed Processing**: Coordinator-worker architecture for large-scale operations
- **💾 Database Support**: SQLite and Redis integration for persistent storage
- **🎨 Multiple Themes**: CyberWave, Matrix, Neon, Terminal progress indicators
- **📱 Browser Integration**: Headless Chrome support for JavaScript-heavy sites

## 🛠️ Installation

### Prerequisites

- Rust 1.70+ 
- Redis (optional, for distributed mode)
- SQLite (included with Rust)

### Build from Source

```bash
git clone https://github.com/HyperSecurityLabs/CyberSpider7.0release
cd CyberSpider7.0release
cargo build --release
```


## 🚀 Quick Start

### Basic Crawling

```bash
# Simple crawl with verbose output
./target/release/cyberspider --site https://example.com --verbose

# Advanced crawl with visualization
./target/release/cyberspider --site https://example.com --depth 3 --concurrent 10 --visualize --theme cyberwave

# Crawl with external sources
./target/release/cyberspider --site https://example.com --other-sources --verbose
```


### Distributed Mode

```bash
# Start coordinator node
./target/release/cyberspider --distributed --node-type coordinator --port 8080

# Start worker nodes
./target/release/cyserspider --distributed --node-type worker --coordinator http://localhost:8080


## 📖 Usage Examples

### Command Line Options

```bash
# Basic usage
./cyberspider --site <URL> [OPTIONS]

# Common options
--site <URL>                    Target website to crawl
--depth <N>                      Maximum crawl depth (default: 1)
--concurrent <N>                  Number of concurrent threads (default: 5)
--verbose                        Enable verbose logging
--visualize                     Generate visualization graphs
--theme <THEME>                Progress theme (cyberwave|matrix|neon|terminal)
--other-sources                  Include external data sources
--security                       Enable security scanning
--browser                        Use headless browser
--database                       Enable database storage
--output <DIR>                   Output directory
--json                          JSON output format
```


### Configuration File (Already Included)

Create `cyberspider.toml`:

```toml
[spider]
site = "https://example.com"
depth = 3
concurrent = 10
delay = 1
timeout = 10
threads = 4
verbose = true
progress_theme = "cyberwave"
other_sources_enabled = true

[database]
enabled = true
sqlite_path = "cyberspider.db"
redis_url = "redis://localhost:6379"

[browser]
enabled = true
headless = true
timeout = 30

[security]
enabled = true
api_detection = true
vulnerability_scan = true
technology_fingerprinting = true

[webhooks]
enabled = false
discord_webhook = "https://discord.com/api/webhooks/..."
slack_webhook = "https://hooks.slack.com/services/..."
```
``

## 🎨 Themes

### CyberWave (Default)
- Scanning wave progress indicator
- Cyan and blue color scheme
- Professional security aesthetic

### Matrix
- Green terminal-style progress
- Classic security tool look

### Neon
- Magenta and purple accents
- Modern, vibrant appearance

### Terminal
- Clean white and gray
- Minimal distraction design

## 📊 Visualization

CyberSpider generates multiple visualization formats:

### DOT Graph (Graphviz)
```bash
./cyberspider --site https://example.com --visualize
dot -Tpng cyberspider_graph.dot -o output.png
```
## 🔌 Plugin Development

Create custom plugins for specialized detection:

```rust
use cyberspider::plugins::{Plugin, PluginResult, PluginContext};

pub struct CustomDetector {
    initialized: bool,
}

#[async_trait::async_trait]
impl Plugin for CustomDetector {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "custom_detector".to_string(),
            version: "1.0.0".to_string(),
            description: "Custom security detector".to_string(),
            author: "Your Name".to_string(),
            plugin_type: PluginType::Detector,
            dependencies: vec![],
            permissions: vec!["read_content".to_string()],
        }
    }

    async fn execute(&mut self, context: &PluginContext) -> Result<PluginResult> {
        // Your custom detection logic here
        Ok(PluginResult {
            success: true,
            data: Some(serde_json::json!({"custom": "result"})),
            error: None,
            metadata: std::collections::HashMap::new(),
        })
    }
}
```


## 🌐 Distributed Architecture

### Coordinator Node
- Manages task distribution
- Monitors worker health
- Aggregates results
- Provides REST API

### Worker Nodes
- Execute crawl tasks
- Report progress and results
- Handle failures gracefully
- Auto-reconnect capability

### Communication
- HTTP REST API for coordination
- Message passing via channels
- Health check endpoints
- Graceful shutdown support

## 📈 Metrics and Monitoring

### Real-Time Metrics
- Request rate and success rate
- Response time percentiles (P95, P99)
- Memory and CPU usage
- Error tracking and alerting

### Performance Monitoring
```bash
# Enable detailed metrics
./cyberspider --site https://example.com --verbose 

# View performance statistics
tail -f cyberspider.log | grep "METRICS"
```

## 🔒 Security Features

### Built-in Scanners
- **API Endpoint Detection**: Automatically discovers REST/GraphQL APIs
- **Form Detection**: Identifies login forms and input fields
- **Technology Fingerprinting**: Detects frameworks, libraries, and technologies
- **Vulnerability Scanning**: Basic security issue detection
- **Subdomain Enumeration**: DNS and certificate-based discovery

### External Intelligence
- **Wayback Machine**: Historical URL discovery
- **Common Crawl**: Bulk URL datasets
- **VirusTotal**: Malware and reputation analysis

## 🗄️ Database Integration

## Advanced Support (Auto)
### SQLite Support
```bash
# Enable SQLite storage
./cyberspider --site https://example.com --database  custom.db
```

### Redis Support
```bash
# Enable Redis for distributed mode
./cyberspider --site https://example.com --database  redis://localhost:6379
```

## 🐳 Browser Integration

### Headless Chrome
```bash
# Enable browser for JavaScript-heavy sites
./cyberspider --site https://example.com --browser --timeout 30
```

### Browser Configuration
- Custom user agents
- Proxy support
- Cookie management
- Screenshot capture
- JavaScript execution control

## 🔧 Configuration

### Environment Variables ( iF needed)
```bash
export CYBERSPIDER_REDIS_URL="redis://localhost:6379"
export CYBERSPIDER_DB_PATH="/path/to/database.db"
export CYBERSPIDER_LOG_LEVEL="info"
export CYBERSPIDDER_PROXY_URL="http://proxy.example.com:8080"
```

### Configuration Files
- `cyberspider.toml` - Main configuration
- `sites.txt` - Bulk target sites
- `user-agents.txt` - Custom user agents
- `proxies.txt` - Proxy rotation list



## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Author

**Khaninkali**  
*Security Researcher  & Offensive Security Mindset Not expert*

**HyperSecurity Offensive Labs**  
*Advanced Security Research and Development*

### 📧 Contact
- **Telegram**: [@hypersecurity_offsec](https://t.me/hypersecurity_offsec)
- **GitHub**: [@khaninkali](https://github.com/hypersecurityLabs)

### 🌐 Links (More Links Will be added Later)
- [Telegram Channel](https://t.me/hypersecurity_offsec)

## 🙏 Acknowledgments

- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client library
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [clap](https://github.com/clap-rs/clap) - Command line parsing
- [indicatif](https://github.com/console/indicatif) - Progress bars
- [petgraph](https://github.com/petgraph/petgraph) - Graph algorithms
- [redis-rs](https://github.com/redis-rs/redis-rs) - Redis client

## ⚠️ Disclaimer

This tool is intended for authorized security testing and research purposes only. Users are responsible for ensuring they have proper authorization before scanning any targets. The authors are not responsible for any misuse of this software.

---

<div align="center">
  <p>Made with ❤️ by <a href="https://github.com/hypersecuritylabs">Khaninkali</a> and <a href="https://github.com/HyperSecurityLabs">HyperSecurity Offensive Labs</a></p>
</div>

**Note Since ITS a New Project** 
**USE ETHICALLY**
