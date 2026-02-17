#!/bin/bash
# test_security.sh: Integration tests for Sentinel Security & Config API

BASE_URL="http://localhost:9100"
ADMIN_TOKEN="sentinel-core-token" # Update if different in config

echo "--- Starting Security Walkthrough ---"

# 1. Health Check (Assuming Auth might be enabled/disabled)
echo "[1] Check Service Status..."
curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/status"
echo " (If 200: Auth Disabled/Public; If 403/401: Auth Enabled)"
echo ""

# 2. Get Initial Config (As Admin)
echo "[2] Fetching Initial Orchestrator Config (as Admin)..."
INITIAL_CONFIG=$(curl -s -H "Authorization: Bearer $ADMIN_TOKEN" "$BASE_URL/orchestrator/config")
echo "Raw Response: $INITIAL_CONFIG"
echo "$INITIAL_CONFIG" | jq .
echo ""

# 3. Enable Zombie Reaper (Simulate Dashboard Toggle)
echo "[3] Enabling Zombie Reaper via API..."
# Create JSON payload to enable zombie reaper
# Note: we need to ensure the response was valid JSON first
if [[ ! "$INITIAL_CONFIG" == {* ]]; then
    echo "ERROR: Initial config fetch failed or returned non-JSON"
    exit 1
fi

UPDATED_CONFIG=$(echo "$INITIAL_CONFIG" | jq '.enable_zombie_reaper = true')
RESPONSE=$(curl -s -X POST -H "Content-Type: application/json" -H "Authorization: Bearer $ADMIN_TOKEN" -d "$UPDATED_CONFIG" "$BASE_URL/orchestrator/config")
echo "Response: $RESPONSE"
echo ""

# 4. Verify Config Update
echo "[4] Verifying Config Update..."
NEW_CONFIG=$(curl -s -H "Authorization: Bearer $ADMIN_TOKEN" "$BASE_URL/orchestrator/config")
IS_ENABLED=$(echo "$NEW_CONFIG" | jq '.enable_zombie_reaper')
if [ "$IS_ENABLED" == "true" ]; then
    echo "SUCCESS: Zombie Reaper is ENABLED."
else
    echo "FAILURE: Config update failed. (Value: $IS_ENABLED)"
fi
echo ""

# 5. RBAC Failure Test (Simulate Unauthorized User with Wrong Token)
echo "[5] Testing Unauthorized Access (Wrong Token)..."
FAIL_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST -H "Content-Type: application/json" -H "Authorization: Bearer wrong-token" -d "$INITIAL_CONFIG" "$BASE_URL/orchestrator/config")
echo "HTTP Code: $FAIL_CODE"
if [ "$FAIL_CODE" == "401" ] || [ "$FAIL_CODE" == "403" ]; then
    echo "SUCCESS: Unauthorized access blocked."
else
    echo "WARNING: Check RBAC implementation (Code: $FAIL_CODE)."
fi

echo "--- Walkthrough Complete ---"
