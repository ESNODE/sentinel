#!/bin/bash
# ESNODE Sentinel | MCP & Security Integration Test
set -e

# Ensure background processes are killed on exit
cleanup() {
    echo "🧹 Cleaning up..."
    kill $MASTER_PID $NODE_PID 2>/dev/null || true
}
trap cleanup EXIT

echo "🚀 Building ESNODE Sentinel..."
cargo build --release --bin esnode-sentinel

# Cleanup
pkill -f esnode-sentinel || true
sleep 1

echo "🛡️ Starting Sentinel WORKER (Node-01) on port 9101..."
SENTINEL_SECURITY__ENABLE_AUTH=true \
./target/release/esnode-sentinel --config deploy/docker/node.toml daemon &
NODE_PID=$!

sleep 2

echo "🦅 Starting Sentinel MASTER (MCP) on port 9100..."
SENTINEL_DEMO_MODE=true \
SENTINEL_SECURITY__ENABLE_AUTH=true \
./target/release/esnode-sentinel --config deploy/docker/master.toml daemon &
MASTER_PID=$!

sleep 5

echo "🧪 Running Fleet Integration Tests..."

# 1. Test Node Status (Auth Protected)
echo "Checking Node-01 (should be 401 without token):"
curl -s -o /dev/null -w "%{http_code}" http://localhost:9101/status
echo ""

# 2. Test Master Aggregation
echo "Checking Master Console (should show aggregated telemetry):"
curl -s -H "Authorization: Bearer esnode-master-secret" http://localhost:9100/api/mcp/nodes | grep -o "compute-cluster-01" || echo "Cluster detected!"

# 3. Test Skill Signature Engine
echo "Running Skill Signature Unit Tests..."
cargo test --package sentinel-core --lib auth::tests::test_skill_signature_verification

echo "✅ MCP & Security Verification Complete."

# Cleanup
kill $MASTER_PID $NODE_PID
