# StaffMon Developer Guide

## Table of Contents
- [Architecture Overview](#architecture-overview)
- [Extending StaffMon](#extending-staffmon)
- [Creating Custom Collectors](#creating-custom-collectors)
- [API Integration](#api-integration)
- [Plugin Development](#plugin-development)
- [Testing](#testing)
- [Contributing](#contributing)

## Architecture Overview

StaffMon follows a modular architecture with clear separation of concerns:

```
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│  Data Collectors│────▶│ SystemInfo   │────▶│ API Client  │
└─────────────────┘     └──────────────┘     └─────────────┘
         │                      │                     │
         │                      ▼                     ▼
         │              ┌──────────────┐      ┌─────────────┐
         └─────────────▶│ JSON Storage │      │ Remote API  │
                        └──────────────┘      └─────────────┘
```

### Core Components

1. **Data Collectors**: Functions that gather system metrics
2. **Data Structures**: Serializable structs representing system state
3. **API Client**: HTTP client for remote communication
4. **Configuration**: Environment and file-based configuration
5. **Logging**: Structured logging system

## Extending StaffMon

### Adding New Metrics

To add new system metrics, follow these steps:

1. **Define the data structure**:

```rust
// In src/main.rs
#[derive(Debug, Serialize)]
struct ContainerInfo {
    container_id: String,
    name: String,
    status: String,
    cpu_usage: f32,
    memory_usage: u64,
}
```

2. **Add to SystemInfo**:

```rust
#[derive(Debug, Serialize)]
struct SystemInfo {
    // ... existing fields ...
    containers: Vec<ContainerInfo>, // New field
}
```

3. **Create collector function**:

```rust
fn get_container_info() -> Vec<ContainerInfo> {
    let mut containers = Vec::new();
    
    // Docker example
    if let Ok(output) = Command::new("docker")
        .args(["ps", "--format", "{{json .}}"])
        .output() {
        // Parse output and populate containers
    }
    
    containers
}
```

4. **Integrate into get_system_info()**:

```rust
fn get_system_info() -> SystemInfo {
    // ... existing code ...
    
    SystemInfo {
        // ... existing fields ...
        containers: get_container_info(),
        timestamp: chrono::Local::now().to_rfc3339(),
    }
}
```

### Creating Custom Monitors

Create specialized monitors for specific use cases:

```rust
// src/monitors/database.rs
use crate::{SystemInfo, ApiClient};
use std::time::Duration;

pub struct DatabaseMonitor {
    client: ApiClient,
    check_interval: Duration,
}

impl DatabaseMonitor {
    pub fn new(client: ApiClient) -> Self {
        Self {
            client,
            check_interval: Duration::from_secs(60),
        }
    }
    
    pub fn check_database_health(&self) -> Result<DatabaseHealth> {
        // Check connection pools
        let pool_status = self.check_connection_pools()?;
        
        // Check query performance
        let query_metrics = self.check_query_performance()?;
        
        // Check replication lag
        let replication_status = self.check_replication()?;
        
        Ok(DatabaseHealth {
            pool_status,
            query_metrics,
            replication_status,
        })
    }
    
    pub async fn run(&self) -> Result<()> {
        loop {
            let health = self.check_database_health()?;
            self.client.send_custom_metrics("database", &health)?;
            tokio::time::sleep(self.check_interval).await;
        }
    }
}
```

## Creating Custom Collectors

### Collector Template

Use this template for creating new collectors:

```rust
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CustomMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}

pub trait MetricCollector {
    type Output: Serialize;
    
    fn collect(&self) -> Result<Self::Output>;
    fn name(&self) -> &str;
}

pub struct MyCustomCollector {
    name: String,
}

impl MyCustomCollector {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}

impl MetricCollector for MyCustomCollector {
    type Output = Vec<CustomMetric>;
    
    fn collect(&self) -> Result<Self::Output> {
        let mut metrics = Vec::new();
        
        // Collect your metrics here
        metrics.push(CustomMetric {
            name: "custom_metric".to_string(),
            value: 42.0,
            unit: "count".to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
        });
        
        Ok(metrics)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
```

### Async Collectors

For collectors that need async operations:

```rust
use tokio::process::Command;

pub struct AsyncCollector;

impl AsyncCollector {
    pub async fn collect_async_metrics(&self) -> Result<Vec<AsyncMetric>> {
        let output = Command::new("some-command")
            .output()
            .await?;
        
        // Process output asynchronously
        let metrics = self.process_output(output).await?;
        Ok(metrics)
    }
    
    async fn process_output(&self, output: std::process::Output) -> Result<Vec<AsyncMetric>> {
        // Async processing
        tokio::task::spawn_blocking(move || {
            // CPU-intensive parsing
        }).await?
    }
}
```

## API Integration

### Creating a Custom API Client

Extend the API client for specific endpoints:

```rust
use crate::api::ApiClient;
use serde::{Serialize, Deserialize};

pub struct ExtendedApiClient {
    base_client: ApiClient,
}

impl ExtendedApiClient {
    pub fn new(config: ApiConfig) -> Result<Self> {
        Ok(Self {
            base_client: ApiClient::new(config)?,
        })
    }
    
    // Custom endpoint for alerts
    pub fn send_alert(&self, alert: &Alert) -> Result<()> {
        let url = format!("{}/api/v1/alerts", self.base_client.config.base_url);
        
        let response = self.base_client.client
            .post(&url)
            .header("X-API-Key", &self.base_client.config.api_key)
            .json(alert)
            .send()?;
            
        if !response.status().is_success() {
            anyhow::bail!("Alert API failed: {}", response.status());
        }
        
        Ok(())
    }
    
    // Batch endpoint for metrics
    pub fn send_metrics_batch(&self, metrics: &[SystemInfo]) -> Result<()> {
        let url = format!("{}/api/v1/metrics/batch", self.base_client.config.base_url);
        
        let response = self.base_client.client
            .post(&url)
            .header("X-API-Key", &self.base_client.config.api_key)
            .json(metrics)
            .send()?;
            
        if !response.status().is_success() {
            anyhow::bail!("Batch API failed: {}", response.status());
        }
        
        Ok(())
    }
}
```

### Webhook Integration

Add webhook support for real-time notifications:

```rust
#[derive(Debug, Serialize)]
struct WebhookPayload {
    event_type: String,
    severity: String,
    message: String,
    data: serde_json::Value,
    timestamp: String,
}

pub struct WebhookClient {
    endpoints: Vec<String>,
    client: reqwest::Client,
}

impl WebhookClient {
    pub fn new(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn send_event(&self, event_type: &str, data: impl Serialize) -> Result<()> {
        let payload = WebhookPayload {
            event_type: event_type.to_string(),
            severity: self.determine_severity(&data),
            message: self.generate_message(&data),
            data: serde_json::to_value(data)?,
            timestamp: chrono::Local::now().to_rfc3339(),
        };
        
        // Send to all endpoints concurrently
        let futures: Vec<_> = self.endpoints.iter()
            .map(|endpoint| self.send_to_endpoint(endpoint, &payload))
            .collect();
            
        futures::future::join_all(futures).await;
        Ok(())
    }
}
```

## Plugin Development

### Plugin Interface

Create a plugin system for extending functionality:

```rust
use std::any::Any;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> Result<()>;
    fn collect(&self) -> Result<Box<dyn Any>>;
    fn cleanup(&mut self) -> Result<()>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        log::info!("Registering plugin: {} v{}", plugin.name(), plugin.version());
        self.plugins.push(plugin);
        Ok(())
    }
    
    pub fn initialize_all(&mut self) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.initialize()?;
        }
        Ok(())
    }
    
    pub fn collect_all(&self) -> Vec<(&str, Box<dyn Any>)> {
        self.plugins.iter()
            .filter_map(|plugin| {
                match plugin.collect() {
                    Ok(data) => Some((plugin.name(), data)),
                    Err(e) => {
                        log::error!("Plugin {} failed: {}", plugin.name(), e);
                        None
                    }
                }
            })
            .collect()
    }
}
```

### Example Plugin

```rust
pub struct RedisMonitorPlugin {
    redis_url: String,
}

impl Plugin for RedisMonitorPlugin {
    fn name(&self) -> &str {
        "redis-monitor"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn initialize(&mut self) -> Result<()> {
        // Test Redis connection
        self.test_connection()?;
        Ok(())
    }
    
    fn collect(&self) -> Result<Box<dyn Any>> {
        let metrics = self.collect_redis_metrics()?;
        Ok(Box::new(metrics))
    }
    
    fn cleanup(&mut self) -> Result<()> {
        // Close connections
        Ok(())
    }
}
```

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_info_collection() {
        let sys = System::new_all();
        let cpu_info = get_cpu_info(&sys);
        
        assert!(cpu_info.usage_percent >= 0.0);
        assert!(cpu_info.usage_percent <= 100.0);
        assert!(cpu_info.frequency_mhz > 0.0);
    }
    
    #[test]
    fn test_memory_info_calculation() {
        let sys = System::new_all();
        let mem_info = get_memory_info(&sys);
        
        assert_eq!(
            mem_info.total_mb,
            mem_info.used_mb + mem_info.free_mb
        );
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use mockito::{mock, Matcher};
    
    #[tokio::test]
    async fn test_api_client_send() {
        let _m = mock("POST", "/api/v1/system-info")
            .match_header("X-API-Key", "test-key")
            .match_body(Matcher::JsonString(r#"{"cpu":{"usage_percent":50.0}}"#.to_string()))
            .with_status(200)
            .create();
            
        let config = ApiConfig {
            base_url: mockito::server_url(),
            api_key: "test-key".to_string(),
            ..Default::default()
        };
        
        let client = ApiClient::new(config).unwrap();
        let info = create_test_system_info();
        
        assert!(client.send_system_info(&info).is_ok());
    }
}
```

### Benchmark Testing

```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_system_info_collection(c: &mut Criterion) {
        c.bench_function("collect_system_info", |b| {
            b.iter(|| {
                black_box(get_system_info())
            });
        });
    }
    
    fn benchmark_json_serialization(c: &mut Criterion) {
        let info = get_system_info();
        
        c.bench_function("serialize_to_json", |b| {
            b.iter(|| {
                black_box(serde_json::to_string(&info).unwrap())
            });
        });
    }
    
    criterion_group!(benches, benchmark_system_info_collection, benchmark_json_serialization);
    criterion_main!(benches);
}
```

## Contributing

### Code Style Guidelines

1. **Naming Conventions**:
   - Use snake_case for functions and variables
   - Use PascalCase for types and structs
   - Use SCREAMING_SNAKE_CASE for constants

2. **Error Handling**:
   - Use `Result<T>` for fallible operations
   - Use `anyhow::Result` for application errors
   - Provide context with `.context("operation failed")?`

3. **Documentation**:
   - Document all public APIs
   - Include examples in doc comments
   - Use `cargo doc` to generate documentation

### Example Contribution

```rust
/// Collects Docker container metrics
///
/// # Examples
///
/// ```
/// let containers = get_container_metrics()?;
/// for container in containers {
///     println!("{}: CPU {}%", container.name, container.cpu_usage);
/// }
/// ```
///
/// # Errors
///
/// Returns an error if Docker daemon is not accessible
pub fn get_container_metrics() -> Result<Vec<ContainerMetrics>> {
    // Implementation
}
```

### Submitting Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Run clippy: `cargo clippy -- -D warnings`
6. Format code: `cargo fmt`
7. Submit a pull request

### Performance Guidelines

1. **Avoid Blocking Operations**:
   ```rust
   // Bad
   let output = Command::new("slow-command").output()?;
   
   // Good
   let output = tokio::process::Command::new("slow-command")
       .output()
       .await?;
   ```

2. **Cache Expensive Operations**:
   ```rust
   lazy_static! {
       static ref HARDWARE_INFO: Mutex<Option<HardwareInfo>> = Mutex::new(None);
   }
   
   fn get_cached_hardware_info() -> HardwareInfo {
       let mut cache = HARDWARE_INFO.lock().unwrap();
       if cache.is_none() {
           *cache = Some(get_hardware_info());
       }
       cache.as_ref().unwrap().clone()
   }
   ```

3. **Use Efficient Data Structures**:
   ```rust
   // Use HashMap for lookups
   let services: HashMap<String, ServiceInfo> = services.into_iter()
       .map(|s| (s.name.clone(), s))
       .collect();
   ```

## Advanced Topics

### Custom Serialization

```rust
use serde::{Serializer, Serialize};

#[derive(Debug)]
struct SensitiveData {
    api_key: String,
    password: String,
}

impl Serialize for SensitiveData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SensitiveData", 2)?;
        state.serialize_field("api_key", "***REDACTED***")?;
        state.serialize_field("password", "***REDACTED***")?;
        state.end()
    }
}
```

### Custom Metrics Aggregation

```rust
pub struct MetricsAggregator {
    buffer: Vec<SystemInfo>,
    max_size: usize,
}

impl MetricsAggregator {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn add(&mut self, info: SystemInfo) -> Option<AggregatedMetrics> {
        self.buffer.push(info);
        
        if self.buffer.len() >= self.max_size {
            Some(self.aggregate())
        } else {
            None
        }
    }
    
    fn aggregate(&mut self) -> AggregatedMetrics {
        let metrics = std::mem::take(&mut self.buffer);
        
        // Calculate aggregates
        let avg_cpu = metrics.iter()
            .map(|m| m.cpu.usage_percent)
            .sum::<f32>() / metrics.len() as f32;
            
        AggregatedMetrics {
            avg_cpu,
            sample_count: metrics.len(),
            time_range: (
                metrics.first().unwrap().timestamp.clone(),
                metrics.last().unwrap().timestamp.clone(),
            ),
        }
    }
}
```