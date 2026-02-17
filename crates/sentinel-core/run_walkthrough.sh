#!/bin/bash
# run_walkthrough.sh: Orchestrates the ESNODE Sentinel Walkthrough

# 1. Start Server
echo "Starting Sentinel Core..."
# Navigate to workspace root to run the binary package
cd ../..
SENTINEL_CONFIG=sentinel_test_config.toml cargo run -p sentinel-cli &
SERVER_PID=$!
# Return to this directory for the test script
cd crates/sentinel-core

# 2. Wait for initialization
echo "Waiting for server to start listening on port 9100..."
# Wait up to 60 seconds
for i in {1..60}; do
    if nc -z localhost 9100; then
        echo "Server is up!"
        break
    fi
    sleep 2
    if [ $i -eq 60 ]; then
        echo "Timeout waiting for server."
        kill $SERVER_PID
        exit 1
    fi
done

# 3. Execute Tests
echo "Running Security Tests..."
chmod +x test_security.sh
./test_security.sh

# 4. Cleanup
echo "Stopping Sentinel Core (PID: $SERVER_PID)..."
kill $SERVER_PID
echo "Walkthrough Complete."
