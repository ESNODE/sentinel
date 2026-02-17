# Testing & TDD workflow

This repo now includes a minimal test suite and CI gates to enforce it before packaging.

Commands (run from repo root):
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

What is covered today:
- Config overrides (sentinel-core/tests/config_overrides.rs)
- Client URL normalization and a tiny in-process mock server for `/status` (agent-bin/src/client.rs tests)
- Console helpers (NodeSummary/MetricToggleState) for data-to-string formatting (agent-bin/src/console.rs tests)
- CLI parsing for status/metrics/enable-metric-set (agent-bin/src/main.rs tests)

CI:
- `.github/workflows/tests.yml` runs fmt/clippy/test on PRs/pushes.
- On tags, a packaging smoke installs `fpm`/rpm tooling and runs `scripts/dist/build-agent.sh` (best-effort; skips missing toolchains).

Notes:
- The integration tests use a `TcpListener` mock server and add no non-std runtime deps beyond dev-only crates (`assert_cmd`, `predicates`, `tempfile`).
- Cross-toolchains are still required for cross builds; the release script skips targets if the toolchain is missing.

## Enterprise Fleet & Security Integration Testing
For validating the Mission Control Plane (MCP) and Cryptographic Skill Verification:

### 1. Cryptographic Signature Test
Verifies that the ED25519 signature engine correctly validates or rejects signed WASM skills.
```bash
cargo test --package sentinel-core --lib auth::tests::test_skill_signature_verification
```

### 2. Fleet Aggregation Test
Simulates a multi-node datacenter on the local machine to verify the MCP Master can scrape and aggregate leaf nodes.
```bash
# Run the automated integration script
./scripts/test_mcp.sh
```

This script:
1. Spawns an authenticated Worker node on port 9101.
2. Spawns an MCP Master node on port 9100.
3. Verifies Master successfully aggregates telemetry from the Worker.
4. Cleans up background processes.
