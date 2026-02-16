# ESNODE Sentinel Installation Guide

<div align="center">
  <img src="docs/images/esnode-logo-dark.png" alt="ESNODE" width="500"/>
  
  **Power-Aware AI Infrastructure**  
  Version 0.2.0
</div>

---

## Table of Contents

- [System Requirements](#system-requirements)
- [Installation Methods](#installation-methods)
- [Quick Start](#quick-start)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Configuration](#configuration)
- [Service Management](#service-management)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

---

## System Requirements

### Supported Operating Systems

| OS | Architecture | Status |
|----|--------------| -------|
| Ubuntu Server 20.04+ | x86_64, ARM64 | ✅ Primary |
| RHEL / Rocky / AlmaLinux | x86_64, ARM64 | ✅ Supported |
| Debian 11+ | x86_64, ARM64 | ✅ Supported |
| SLES 15+ | x86_64 | ✅ Supported |
| macOS 12+ | ARM64 (M1/M2), x86_64 | ✅ Supported |
| NVIDIA DGX OS | x86_64 | ✅ Optimized |

### Hardware Requirements

**Minimum:**
- CPU: 2 cores
- RAM: 1 GB
- Disk: 500 MB

**Recommended:**
- CPU: 4+ cores
- RAM: 2 GB
- Disk: 2 GB (for local TSDB)
- GPU: NVIDIA with CUDA support (optional)

### Software Dependencies

**Required:**
- None (statically linked binary)

**Optional:**
- **NVIDIA GPU Drivers** (for GPU metrics)
- **CUDA Toolkit** 11.0+ (for advanced GPU features)
- **systemd** (Linux) or **launchd** (macOS) for service management

---

## Installation Methods

### Method 1: Automated Installation (Recommended)

Download and run the installation script:

```bash
# Download release
wget https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-linux-amd64-v0.2.0.tar.gz

# Extract
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz
cd esnode-sentinel-linux-amd64-v0.2.0

# Install (requires root)
sudo ./install.sh
```

**User Installation (no root required):**
```bash
INSTALL_DIR=$HOME/.local/bin ./install.sh
```

### Method 2: Manual Installation

```bash
# Extract binary
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz

# Copy to system
sudo cp esnode-sentinel-linux-amd64-v0.2.0/esnode-sentinel /usr/local/bin/
sudo chmod +x /usr/local/bin/esnode-sentinel

# Create directories
sudo mkdir -p /etc/esnode
sudo mkdir -p /var/lib/esnode/tsdb
sudo mkdir -p /var/log/esnode

# Create service user (Linux)
sudo useradd --system --no-create-home --shell /bin/false esnode

# Set permissions
sudo chown -R esnode:esnode /var/lib/esnode /var/log/esnode
```

### Method 3: Build from Source

```bash
# Clone repository
git clone https://github.com/ESNODE/ESNODE Sentinel.git
cd ESNODE Sentinel

# Build release
cargo build --release -p agent-bin

# Install
sudo cp target/release/esnode-sentinel /usr/local/bin/
```

---

## Quick Start

### 1. Start the Agent

**Foreground (testing):**
```bash
esnode-sentinel daemon
```

**Background (systemd):**
```bash
sudo systemctl start esnode-sentinel
sudo systemctl enable esnode-sentinel  # Enable at boot
```

### 2. Verify Operation

```bash
# Check health
curl http://localhost:9100/health

# View metrics
curl http://localhost:9100/metrics

# Get status
curl http://localhost:9100/status | jq
```

### 3. Launch TUI

```bash
esnode-sentinel cli
```

Use arrow keys to navigate between screens.

---

## Platform-Specific Instructions

### Ubuntu / Debian

```bash
# Download
wget https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-linux-amd64-v0.2.0.tar.gz

# Extract and install
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz
cd esnode-sentinel-linux-amd64-v0.2.0
sudo ./install.sh

# Start service
sudo systemctl start esnode-sentinel
sudo systemctl status esnode-sentinel

# View logs
sudo journalctl -u esnode-sentinel -f
```

### RHEL / Rocky / AlmaLinux

```bash
# Download
curl -LO https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-linux-amd64-v0.2.0.tar.gz

# Extract and install
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz
cd esnode-sentinel-linux-amd64-v0.2.0
sudo ./install.sh

# Configure firewall (if needed)
sudo firewall-cmd --add-port=9100/tcp --permanent
sudo firewall-cmd --reload

# Start service
sudo systemctl start esnode-sentinel
sudo systemctl enable esnode-sentinel
```

### macOS

```bash
# Download (ARM64 for M1/M2)
curl -LO https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-darwin-arm64-v0.2.0.tar.gz

# Extract and install
tar -xzf esnode-sentinel-darwin-arm64-v0.2.0.tar.gz
cd esnode-sentinel-darwin-arm64-v0.2.0
sudo ./install.sh

# Start service
sudo launchctl load /Library/LaunchDaemons/com.estimatedstocks.esnode-sentinel.plist
```

### NVIDIA DGX

```bash
# DGX OS is Ubuntu-based
wget https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-linux-amd64-v0.2.0.tar.gz
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz
cd esnode-sentinel-linux-amd64-v0.2.0
sudo ./install.sh

# Enable GPU metrics (already default)
# Verify GPU detection
esnode-sentinel status | jq '.gpus'
```

---

## Configuration

### Configuration File

Default location: `/etc/esnode/config.toml` (system) or `~/.config/esnode/config.toml` (user)

**Basic Configuration:**
```toml
# HTTP listener
listen_address = "0.0.0.0:9100"

# Scrape interval
scrape_interval = "5s"

# Enable collectors
enable_cpu = true
enable_memory = true
enable_disk = true
enable_network = true
enable_gpu = true
enable_power = true

# GPU settings
enable_gpu_mig = false
enable_gpu_events = false

# Local TSDB
enable_local_tsdb = true
local_tsdb_path = "/var/lib/esnode/tsdb"
local_tsdb_retention_hours = 48

# Logging
log_level = "info"
```

**Advanced GPU Configuration:**
```toml
# Filter visible GPUs (comma-separated UUIDs or indices)
gpu_visible_devices = "0,1,2,3"

# Enable MIG metrics (requires NVML FFI)
enable_gpu_mig = true
mig_config_devices = "GPU-abc123,GPU-def456"

# Enable GPU events (XID, ECC, throttle)
enable_gpu_events = true

# Kubernetes compatibility mode
k8s_mode = true
```

**Power Management:**
```toml
# Enable power collector
enable_power = true

# Set node power envelope (optional)
node_power_envelope_watts = 1500

# Enable orchestrator (autonomous power-aware scheduling)
enable_orchestrator = true
```

### Environment Variables

Override configuration via environment variables:

```bash
export ESNODE_LISTEN_ADDRESS="0.0.0.0:9100"
export ESNODE_SCRAPE_INTERVAL="10s"
export ESNODE_ENABLE_GPU=true
export ESNODE_LOG_LEVEL="debug"
export ESNODE_CONFIG="/custom/path/config.toml"
```

---

## Service Management

### systemd (Linux)

```bash
# Start
sudo systemctl start esnode-sentinel

# Stop
sudo systemctl stop esnode-sentinel

# Restart
sudo systemctl restart esnode-sentinel

# Status
sudo systemctl status esnode-sentinel

# Enable at boot
sudo systemctl enable esnode-sentinel

# Disable at boot
sudo systemctl disable esnode-sentinel

# View logs
sudo journalctl -u esnode-sentinel -f

# View recent errors
sudo journalctl -u esnode-sentinel -p err -n 50
```

### launchd (macOS)

```bash
# Load (start)
sudo launchctl load /Library/LaunchDaemons/com.estimatedstocks.esnode-sentinel.plist

# Unload (stop)
sudo launchctl unload /Library/LaunchDaemons/com.estimatedstocks.esnode-sentinel.plist

# View logs
tail -f /var/log/esnode/esnode-sentinel.log
```

### Manual Daemon

```bash
# Start in background
nohup esnode-sentinel daemon > /var/log/esnode/esnode-sentinel.log 2>&1 &

# Stop
pkill -f esnode-sentinel
```

---

## Verification

### Check Installation

```bash
# Version check
esnode-sentinel --version

# Help
esnode-sentinel --help

# Test configuration
esnode-sentinel daemon --config /etc/esnode/config.toml &
sleep 5
curl http://localhost:9100/health
pkill -f esnode-sentinel
```

### Verify GPU Detection

```bash
# Check GPU status
esnode-sentinel status | jq '.gpus'

# Or via metrics
curl http://localhost:9100/metrics | grep gpu_utilization

# TUI GPU screen
esnode-sentinel cli  # Navigate to "GPU & Power"
```

### Test Metrics Export

```bash
# Fetch all metrics
curl http://localhost:9100/metrics

# Test Prometheus scrape
curl -H "Accept: application/openmetrics-text" http://localhost:9100/metrics

# Specific metric
curl -s http://localhost:9100/metrics | grep esnode_cpu_percent
```

### Performance Check

```bash
# CPU usage
top -p $(pgrep esnode-sentinel)

# Memory usage
ps aux | grep esnode-sentinel

# Number of open files
lsof -p $(pgrep esnode-sentinel) | wc -l
```

---

## Troubleshooting

### Common Issues

#### 1. Binary Won't Run

**Symptom:** `Permission denied` or `command not found`

**Solution:**
```bash
# Make executable
chmod +x /usr/local/bin/esnode-sentinel

# Add to PATH
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### 2. GPU Metrics Show Zeros

**Symptom:** `gpu_utilization=0` for all GPUs

**Solutions:**
```bash
# Check NVIDIA driver
nvidia-smi

# Verify GPU access
ls -l /dev/nvidia*

# Check permissions (may need to run as root initially)
sudo esnode-sentinel daemon

# Add user to GPU group (Debian/Ubuntu)
sudo usermod -a -G video esnode
sudo usermod -a -G render esnode
```

#### 3. Port Already in Use

**Symptom:** `Address already in use` error

**Solutions:**
```bash
# Find process using port 9100
sudo lsof -i :9100

# Kill conflicting process
sudo kill $(sudo lsof -t -i:9100)

# Or change port in config
echo 'listen_address = "0.0.0.0:9101"' >> /etc/esnode/config.toml
```

#### 4. TUI Shows "CONNECTING..."

**Symptom:** TUI unable to connect to agent

**Solutions:**
```bash
# Verify daemon is running
ps aux | grep esnode-sentinel

# Check if metrics endpoint responds
curl http://localhost:9100/status

# Start daemon if not running
sudo systemctl start esnode-sentinel
```

#### 5. High Memory Usage

**Symptom:** Agent consuming excessive RAM

**Solutions:**
```toml
# Reduce TSDB retention in config
local_tsdb_retention_hours = 24
local_tsdb_max_disk_mb = 1024

# Disable TSDB if not needed
enable_local_tsdb = false

# Reduce scrape frequency
scrape_interval = "10s"
```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
# Via environment variable
ESNODE_LOG_LEVEL=debug esnode-sentinel daemon

# Via config
echo 'log_level = "debug"' >> /etc/esnode/config.toml
sudo systemctl restart esnode-sentinel

# View debug logs
sudo journalctl -u esnode-sentinel -f
```

### Getting Help

- **GitHub Issues:** [Report bugs](https://github.com/ESNODE/ESNODE Sentinel/issues)
- **Documentation:** [User guides](https://github.com/ESNODE/ESNODE Sentinel/tree/main/docs)
- **TUI Guide:** `docs/TUI_USER_GUIDE.md`

---

## Upgrade Instructions

### From v0.1.x to v0.2.0

```bash
# Stop service
sudo systemctl stop esnode-sentinel

# Backup configuration
sudo cp /etc/esnode/config.toml /etc/esnode/config.toml.backup

# Download new version
wget https://github.com/ESNODE/ESNODE Sentinel/releases/download/v0.2.0/esnode-sentinel-linux-amd64-v0.2.0.tar.gz
tar -xzf esnode-sentinel-linux-amd64-v0.2.0.tar.gz

# Replace binary
sudo cp esnode-sentinel-linux-amd64-v0.2.0/esnode-sentinel /usr/local/bin/
sudo chmod +x /usr/local/bin/esnode-sentinel

# Restart service
sudo systemctl start esnode-sentinel
sudo systemctl status esnode-sentinel

# Verify TUI
esnode-sentinel cli  # Should show new branding
```

**Breaking Changes:**
- None (v0.2.0 is backward compatible)

---

## Uninstallation

```bash
# Stop and disable service
sudo systemctl stop esnode-sentinel
sudo systemctl disable esnode-sentinel

# Remove binary
sudo rm /usr/local/bin/esnode-sentinel

# Remove service file
sudo rm /etc/systemd/system/esnode-sentinel.service
sudo systemctl daemon-reload

# Remove data (optional)
sudo rm -rf /var/lib/esnode
sudo rm -rf /var/log/esnode
sudo rm -rf /etc/esnode

# Remove service user (optional)
sudo userdel esnode
```

---

## Security Considerations

### Network Security

```bash
# Bind to localhost only (recommended for single-node)
listen_address = "127.0.0.1:9100"

# Use firewall to restrict access
sudo ufw allow from 192.168.1.0/24 to any port 9100 proto tcp
```

### File Permissions

```bash
# Secure configuration
sudo chmod 640 /etc/esnode/config.toml
sudo chown root:esnode /etc/esnode/config.toml

# Secure data directory
sudo chmod 750 /var/lib/esnode
sudo chown esnode:esnode /var/lib/esnode
```

### SELinux (RHEL/CentOS)

```bash
# If SELinux blocks operation
sudo semanage fcontext -a -t bin_t "/usr/local/bin/esnode-sentinel"
sudo restorecon -v /usr/local/bin/esnode-sentinel
```

---

## License

BUSL-1.1 (Business Source License)  
Copyright (c) 2024 Estimatedstocks AB

Source available at: https://github.com/ESNODE/ESNODE Sentinel

---

**Last Updated:** 2026-02-07  
**Version:** 0.2.0
