# StaffMon – API Reference & Usage Guide

> **Version:** 1.0.2  
> **Crate:** `staffmon`  
> **Edition:** Rust 2021

This document provides an end-to-end reference for every **public** type, function, and module exposed by the `staffmon` code-base together with ready-to-run examples.

---

## 1. Module `config`

### 1.1 `ApiConfig`
A serialisable structure that holds all run-time settings required to communicate with the remote StaffMon backend.

| Field | Type | Purpose | Default |
|-------|------|---------|---------|
| `base_url` | `String` | Root URL of the backend (e.g. `https://api.staffmon.io`) | `"http://localhost:8080"` |
| `api_key` | `String` | Authentication token/secret | empty string |
| `timeout_seconds` | `u64` | Maximum HTTP request latency in seconds | `30` |
| `retry_count` | `u32` | How many times a failed request is retried | `3` |
| `rate_limit` | `u32` | Maximum requests per minute accepted by the backend | `100` |

#### Associated Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `load` | `pub fn load() -> Result<ApiConfig>` | Merges `config.toml` + `STAFFMON_*` environment overrides. Falls back to [`Default`] if the file is absent. |
| `default` | `fn default() -> ApiConfig` | Supplies the values shown in the table above. (Provided automatically by `impl Default`). |

#### Quick-Start Example
```rust
use staffmon::config::ApiConfig;

// Load from `config.toml` + environment variables.
let cfg = ApiConfig::load()?;
println!("Backend → {} (timeout = {} s)", cfg.base_url, cfg.timeout_seconds);
```

---

## 2. Module `api`

### 2.1 `ApiClient`
A thin synchronous wrapper around `reqwest::blocking::Client` that is pre-configured with time-outs and authentication headers.

| Function | Signature | Notes |
|----------|-----------|-------|
| `new` | `pub fn new(cfg: ApiConfig) -> Result<ApiClient>` | Builds an underlying HTTP client using `cfg.timeout_seconds`. |
| `send_system_info` | `pub fn send_system_info(&self, info: &SystemInfo) -> Result<()>` | `POST /api/v1/system-info`.
Errors if the server does not respond with a 2xx status code. |
| `get_system_info` | `pub fn get_system_info(&self) -> Result<SystemInfo>` | `GET /api/v1/system-info` and deserialises the JSON payload. |

#### Usage Example
```rust
use staffmon::config::ApiConfig;
use staffmon::api::ApiClient;
use staffmon::SystemInfo; // re-exported from the binary crate root

let cfg = ApiConfig::load()?;
let client = ApiClient::new(cfg)?;

// Gather local metrics however you wish …
let metrics: SystemInfo = collect_metrics();

// Upload them
client.send_system_info(&metrics)?;

// Or retrieve the last snapshot recorded by the backend
let latest = client.get_system_info()?;
println!("Remote CPU usage: {} %", latest.cpu.usage_percent);
```

---

## 3. Module `log_config`

### 3.1 `init_logger`
`pub fn init_logger() -> Result<(), Box<dyn std::error::Error>>`

Initialises a rotating file logger (`logs/staffmon.log`) using **log4rs** with the following pattern:
```
{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}{n}
```
The logger is automatically picked up by every crate that relies on the `log` facade (`info!`, `warn!`, `error!`, …).

#### Example
```rust
use staffmon::log_config;

log_config::init_logger()?;
log::info!("Hello logging 🌍");
```

---

## 4. Data Model – `SystemInfo` & Friends
Although these structures are **not public** outside the crate, understanding their shape helps when consuming the REST API or the generated JSON snapshots.

```text
SystemInfo
├── cpu: CpuInfo
│   ├── usage_percent: f32
│   ├── temperature_celsius: Option<f32>
│   └── frequency_mhz: f32
├── memory: MemoryInfo        (total/used/free in MiB)
├── load_avg: LoadAverage     (1 / 5 / 15-minute)
├── disks: Vec<DiskInfo>
├── network: NetworkInfo
│   └── interfaces: Vec<NetworkInterface>
├── user_access: UserAccess
├── services: Vec<ServiceInfo>
├── security: SecurityInfo
├── hardware: HardwareInfo
├── system_uptime: UptimeInfo
├── hostname / kernel_version / os_version: String
├── process_list: Vec<ProcessInfo>
└── timestamp: RFC-3339 String
```

A prettified JSON sample is produced every two seconds at the project root when running the monitor binary:
```bash
$ cat system_info_20250101_120000.json | jq .
```

---

## 5. Running StaffMon as a Service
The CLI entry-point lives in `main.rs`. Typical workflows:

### 5.1 Manual Run (Foreground)
```bash
cargo run --release --quiet
```

### 5.2 As a Daemon
`staffmon` automatically daemonises itself via **daemonize** and writes its PID to `/tmp/staffmon.pid`. Logs land in `logs/staffmon.log`.

---

## 6. Contributing & Extending

1. **Add new metrics** – write a helper that returns a plain Rust structure and plug it into `get_system_info()`.
2. **Tune log verbosity** – edit `log_config::init_logger()` or provide your own `log4rs.yaml` and initialise at start-up.
3. **Generate HTML docs** – simply run `cargo doc --open` to explore the code-level documentation generated from this file along with inline comments.

---

## 7. Support & Troubleshooting
See `docs/TROUBLESHOOTING_EN.md` for common runtime issues or open an [issue](https://github.com/forniya/StaffLinuxMonitor/issues).

---

© 2025 StaffMon contributors – MIT License.