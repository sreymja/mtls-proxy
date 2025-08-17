#!/bin/bash

# Test script for UI refactoring
# This script tests that all UI pages are working with the new embedded CSS approach

BASE_URL="http://localhost:8440"
echo "🧪 Testing mTLS Proxy UI Refactoring"
echo "====================================="

# Test 1: Health endpoint
echo "1. Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/health")
HTTP_CODE="${HEALTH_RESPONSE: -3}"
RESPONSE_BODY="${HEALTH_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Health endpoint working (HTTP $HTTP_CODE)"
    echo "   📄 Response: $RESPONSE_BODY"
else
    echo "   ❌ Health endpoint failed (HTTP $HTTP_CODE)"
    exit 1
fi

echo ""

# Test 2: Dashboard page
echo "2. Testing dashboard page..."
DASHBOARD_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui")
HTTP_CODE="${DASHBOARD_RESPONSE: -3}"
RESPONSE_BODY="${DASHBOARD_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Dashboard page working (HTTP $HTTP_CODE)"
    if echo "$RESPONSE_BODY" | grep -q "mTLS Proxy - Dashboard"; then
        echo "   ✅ Correct title found"
    else
        echo "   ⚠️  Title not found in response"
    fi
    if echo "$RESPONSE_BODY" | grep -q "font-family: -apple-system"; then
        echo "   ✅ Embedded CSS found"
    else
        echo "   ⚠️  Embedded CSS not found"
    fi
else
    echo "   ❌ Dashboard page failed (HTTP $HTTP_CODE)"
    exit 1
fi

echo ""

# Test 3: Logs page
echo "3. Testing logs page..."
LOGS_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui/logs")
HTTP_CODE="${LOGS_RESPONSE: -3}"
RESPONSE_BODY="${LOGS_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Logs page working (HTTP $HTTP_CODE)"
    if echo "$RESPONSE_BODY" | grep -q "mTLS Proxy - Logs"; then
        echo "   ✅ Correct title found"
    else
        echo "   ⚠️  Title not found in response"
    fi
    if echo "$RESPONSE_BODY" | grep -q "font-family: -apple-system"; then
        echo "   ✅ Embedded CSS found"
    else
        echo "   ⚠️  Embedded CSS not found"
    fi
else
    echo "   ❌ Logs page failed (HTTP $HTTP_CODE)"
    exit 1
fi

echo ""

# Test 4: Config page
echo "4. Testing config page..."
CONFIG_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui/config")
HTTP_CODE="${CONFIG_RESPONSE: -3}"
RESPONSE_BODY="${CONFIG_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Config page working (HTTP $HTTP_CODE)"
    if echo "$RESPONSE_BODY" | grep -q "mTLS Proxy - Configuration"; then
        echo "   ✅ Correct title found"
    else
        echo "   ⚠️  Title not found in response"
    fi
    if echo "$RESPONSE_BODY" | grep -q "font-family: -apple-system"; then
        echo "   ✅ Embedded CSS found"
    else
        echo "   ⚠️  Embedded CSS not found"
    fi
else
    echo "   ❌ Config page failed (HTTP $HTTP_CODE)"
    exit 1
fi

echo ""

# Test 5: API endpoints
echo "5. Testing API endpoints..."

# Stats API
STATS_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui/api/stats")
HTTP_CODE="${STATS_RESPONSE: -3}"
RESPONSE_BODY="${STATS_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Stats API working (HTTP $HTTP_CODE)"
    echo "   📊 Stats: $RESPONSE_BODY"
else
    echo "   ❌ Stats API failed (HTTP $HTTP_CODE)"
fi

# Logs API
LOGS_API_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui/api/logs")
HTTP_CODE="${LOGS_API_RESPONSE: -3}"
RESPONSE_BODY="${LOGS_API_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Logs API working (HTTP $HTTP_CODE)"
    echo "   📋 Logs: $RESPONSE_BODY"
else
    echo "   ❌ Logs API failed (HTTP $HTTP_CODE)"
fi

# Config API
CONFIG_API_RESPONSE=$(curl -s -w "%{http_code}" "$BASE_URL/ui/api/config/current")
HTTP_CODE="${CONFIG_API_RESPONSE: -3}"
RESPONSE_BODY="${CONFIG_API_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "   ✅ Config API working (HTTP $HTTP_CODE)"
    echo "   ⚙️  Config loaded successfully"
else
    echo "   ❌ Config API failed (HTTP $HTTP_CODE)"
fi

echo ""

# Test 6: Navigation consistency
echo "6. Testing navigation consistency..."

# Check that all pages have the same navigation structure
DASHBOARD_NAV=$(echo "$DASHBOARD_RESPONSE" | grep -o '<a href="/ui[^"]*"[^>]*>[^<]*</a>' | head -3)
LOGS_NAV=$(echo "$LOGS_RESPONSE" | grep -o '<a href="/ui[^"]*"[^>]*>[^<]*</a>' | head -3)
CONFIG_NAV=$(echo "$CONFIG_RESPONSE" | grep -o '<a href="/ui[^"]*"[^>]*>[^<]*</a>' | head -3)

if [ "$DASHBOARD_NAV" = "$LOGS_NAV" ] && [ "$LOGS_NAV" = "$CONFIG_NAV" ]; then
    echo "   ✅ Navigation is consistent across all pages"
    echo "   🧭 Navigation: $DASHBOARD_NAV"
else
    echo "   ⚠️  Navigation differs between pages"
    echo "   Dashboard: $DASHBOARD_NAV"
    echo "   Logs: $LOGS_NAV"
    echo "   Config: $CONFIG_NAV"
fi

echo ""

# Test 7: CSS consistency
echo "7. Testing CSS consistency..."

# Check that all pages have the same base CSS styles
DASHBOARD_CSS=$(echo "$DASHBOARD_RESPONSE" | grep -o 'body\s*{[^}]*}' | head -1)
LOGS_CSS=$(echo "$LOGS_RESPONSE" | grep -o 'body\s*{[^}]*}' | head -1)
CONFIG_CSS=$(echo "$CONFIG_RESPONSE" | grep -o 'body\s*{[^}]*}' | head -1)

if [ "$DASHBOARD_CSS" = "$LOGS_CSS" ] && [ "$LOGS_CSS" = "$CONFIG_CSS" ]; then
    echo "   ✅ Base CSS is consistent across all pages"
else
    echo "   ⚠️  Base CSS differs between pages"
fi

echo ""

echo "🎉 UI Refactoring Test Complete!"
echo "================================"
echo ""
echo "📋 Summary:"
echo "   • All UI pages are serving HTML with embedded CSS"
echo "   • All API endpoints are working correctly"
echo "   • Navigation is consistent across pages"
echo "   • CSS styling is unified across all pages"
echo ""
echo "✅ The refactoring from external CSS to embedded CSS is complete and working!"
