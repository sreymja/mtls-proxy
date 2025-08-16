#!/bin/bash

# mTLS Proxy Configuration UI Test Script
# This script tests all the working configuration management features

set -e

echo "🧪 Testing mTLS Proxy Configuration Management Features"
echo "========================================================"

# Configuration
BASE_URL="http://localhost:8443"
AUTH_HEADER="Authorization: Basic $(echo -n 'admin:admin123' | base64)"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Helper function to run tests
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_status="$3"
    
    echo -e "\n${BLUE}Testing: ${test_name}${NC}"
    
    if eval "$command" > /tmp/test_output.json 2>&1; then
        local status=$(tail -1 /tmp/test_output.json | grep -o '[0-9]*$')
        if [ "$status" = "$expected_status" ]; then
            echo -e "  ${GREEN}✅ PASSED${NC}"
            ((TESTS_PASSED++))
        else
            echo -e "  ${RED}❌ FAILED (Expected: $expected_status, Got: $status)${NC}"
            ((TESTS_FAILED++))
        fi
    else
        echo -e "  ${RED}❌ FAILED (Command error)${NC}"
        ((TESTS_FAILED++))
    fi
}

# Helper function to check JSON response
check_json_response() {
    local test_name="$1"
    local command="$2"
    local expected_field="$3"
    local expected_value="$4"
    
    echo -e "\n${BLUE}Testing: ${test_name}${NC}"
    
    if eval "$command" > /tmp/test_output.json 2>/dev/null; then
        local actual_value=$(cat /tmp/test_output.json | jq -r "$expected_field" 2>/dev/null)
        if [ "$actual_value" = "$expected_value" ]; then
            echo -e "  ${GREEN}✅ PASSED${NC}"
            ((TESTS_PASSED++))
        else
            echo -e "  ${RED}❌ FAILED (Expected: $expected_value, Got: $actual_value)${NC}"
            ((TESTS_FAILED++))
        fi
    else
        echo -e "  ${RED}❌ FAILED (Command error)${NC}"
        ((TESTS_FAILED++))
    fi
}

echo -e "\n${YELLOW}1. Testing Health Endpoint${NC}"
run_test "Health Check" "curl -s -w 'Status: %{http_code}' $BASE_URL/health" "200"

echo -e "\n${YELLOW}2. Testing Authentication${NC}"
run_test "Unauthorized Access to UI" "curl -s -w 'Status: %{http_code}' $BASE_URL/ui/config" "401"
run_test "Authorized Access to UI" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui/config" "200"

echo -e "\n${YELLOW}3. Testing Configuration API${NC}"
check_json_response "Get Current Configuration" "curl -s -H '$AUTH_HEADER' $BASE_URL/ui/api/config/current" ".target.base_url" "https://test-target.example.com"

run_test "Validate Configuration" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' -H 'Content-Type: application/json' -X POST -d '{}' $BASE_URL/ui/api/config/validate" "200"

echo -e "\n${YELLOW}4. Testing Configuration Update${NC}"
run_test "Update Configuration" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' -H 'Content-Type: application/json' -X POST -d '{\"target_url\":\"https://updated-target.example.com\",\"timeout_secs\":120,\"max_connections\":2000,\"auth_enabled\":true,\"admin_username\":\"admin\",\"admin_password\":null}' $BASE_URL/ui/api/config/update" "200"

check_json_response "Verify Configuration Update" "curl -s -H '$AUTH_HEADER' $BASE_URL/ui/api/config/current" ".target.base_url" "https://updated-target.example.com"

echo -e "\n${YELLOW}5. Testing Certificate Management${NC}"
check_json_response "List Certificates" "curl -s -H '$AUTH_HEADER' $BASE_URL/ui/api/certificates/list" ".certificates | length" "7"

run_test "Delete Certificate (non-existent)" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' -X DELETE $BASE_URL/ui/api/certificates/delete/non-existent.crt" "200"

echo -e "\n${YELLOW}6. Testing Metrics Endpoint${NC}"
run_test "Metrics Endpoint" "curl -s -w 'Status: %{http_code}' $BASE_URL/metrics" "200"

echo -e "\n${YELLOW}7. Testing UI Navigation${NC}"
run_test "Dashboard Page" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui" "200"
run_test "Logs Page" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui/logs" "200"
run_test "Config Page" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui/config" "200"

echo -e "\n${YELLOW}8. Testing API Endpoints${NC}"
run_test "API Logs Endpoint" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui/api/logs" "200"
run_test "API Stats Endpoint" "curl -s -w 'Status: %{http_code}' -H '$AUTH_HEADER' $BASE_URL/ui/api/stats" "200"

echo -e "\n${YELLOW}9. Testing Configuration Persistence${NC}"
echo -e "\n${BLUE}Testing: Configuration Persistence${NC}"
if [ -f "./config/config.toml" ]; then
    echo -e "  ${GREEN}✅ PASSED - Configuration file exists${NC}"
    ((TESTS_PASSED++))
else
    echo -e "  ${RED}❌ FAILED - Configuration file not found${NC}"
    ((TESTS_FAILED++))
fi

echo -e "\n${YELLOW}10. Testing Certificate Directory${NC}"
echo -e "\n${BLUE}Testing: Certificate Directory${NC}"
if [ -d "./certs" ]; then
    echo -e "  ${GREEN}✅ PASSED - Certificate directory exists${NC}"
    ((TESTS_PASSED++))
else
    echo -e "  ${RED}❌ FAILED - Certificate directory not found${NC}"
    ((TESTS_FAILED++))
fi

# Summary
echo -e "\n${YELLOW}========================================================"
echo "Test Summary"
echo "========================================================"
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
echo -e "Total Tests: $((TESTS_PASSED + TESTS_FAILED))${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All tests passed! Configuration management is working perfectly.${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed. Please check the output above.${NC}"
    exit 1
fi
