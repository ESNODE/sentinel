# ESNODE Sentinel | Mission Control Plane (MCP) & Orchestration

The Sentinel Orchestrator and Mission Control Plane (MCP) enable global coordination of high-performance AI infrastructure.

## 🦅 Mission Control Plane (MCP)
The MCP allows a master node to aggregate and control a global fleet of Sentinel-managed servers.

### Master Aggregation
When `enable_mcp = true` is set in `sentinel.toml`, the master node initiates a background scraping loop for all cluster endpoints.

- **Cross-DC Visibility**: Telemetry from remote clusters is merged into the local state.
- **Node Topology**: Remote GPUs are identified by their cluster name prefixed to their ID (e.g., `EMEA-DC-01:GPU-08`).

### Configuration
```toml
enable_mcp = true

[[orchestrator.clusters]]
name = "US-EAST-01"
endpoints = ["node-a.dc1.esnode.io:9100", "node-b.dc1.esnode.io:9100"]
```

## 🦾 Autonomous Orchestrator Features
The Sentinel Orchestrator (embedded in the agent) continuously optimizes hardware state via its autonomous tick cycle.

| Feature | Description | Config Key |
| :--- | :--- | :--- |
| **Zombie Reaper** | Identifies and kills orphaned AI tasks to reclaim VRAM. | `enable_zombie_reaper` |
| **Flash Preemption** | Immediately pauses low-priority tasks when high-priority requests arrive. | `enable_flash_preemption` |
| **Thermal Management** | Automatically throttles power or offloads tasks if temps exceed 85°C. | `enable_thermal_management` |
| **Bin Packing** | Optimizes task placement to use the minimum number of GPUs, lowering PUE. | `enable_bin_packing` |
| **Dataset Prefetch** | Predicts next workloads and pre-fetches data to local NVME cache. | `enable_dataset_prefetch` |

## 🧪 Testing & Verification
The orchestrator includes robust unit tests to verify its scheduling and autonomous logic:

```bash
# Run orchestrator tests
cargo test -p sentinel-orchestrator
```

## 🛡️ Enterprise Security in MCP
MCP communication and skill deployment are protected by **ED25519 signatures** and **JWT Auth**. 
- Leaf nodes verify the Master's signature before accepting fleet-wide commands.
- All telemetry is encrypted via **mutual TLS (mTLS)** in production environments.

---
*ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB*
