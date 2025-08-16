#!/usr/bin/env python3
"""
Test script for the mTLS Proxy UI.
This script tests the web interface endpoints.
"""

import requests
import json
import time
from typing import Dict, Any

class UI tester:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url.rstrip('/')
        self.session = requests.Session()
    
    def test_dashboard(self) -> Dict[str, Any]:
        """Test the dashboard endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/dashboard", timeout=10)
            response.raise_for_status()
            return {"status": "success", "status_code": response.status_code, "content_type": response.headers.get('content-type')}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_logs(self) -> Dict[str, Any]:
        """Test the logs endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/logs", timeout=10)
            response.raise_for_status()
            return {"status": "success", "status_code": response.status_code, "content_type": response.headers.get('content-type')}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_health(self) -> Dict[str, Any]:
        """Test the health endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/health", timeout=10)
            response.raise_for_status()
            return {"status": "success", "status_code": response.status_code, "content_type": response.headers.get('content-type')}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_api_logs(self) -> Dict[str, Any]:
        """Test the API logs endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/api/logs?limit=10", timeout=10)
            response.raise_for_status()
            data = response.json()
            return {"status": "success", "status_code": response.status_code, "data_length": len(data)}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_api_stats(self) -> Dict[str, Any]:
        """Test the API stats endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/api/stats", timeout=10)
            response.raise_for_status()
            data = response.json()
            return {"status": "success", "status_code": response.status_code, "stats": data}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_static_files(self) -> Dict[str, Any]:
        """Test static file serving."""
        files_to_test = [
            "/ui/static/style.css",
            "/ui/static/script.js",
            "/ui/static/favicon.ico"
        ]
        
        results = {}
        for file_path in files_to_test:
            try:
                response = self.session.get(f"{self.base_url}{file_path}", timeout=10)
                response.raise_for_status()
                results[file_path] = {
                    "status": "success",
                    "status_code": response.status_code,
                    "content_type": response.headers.get('content-type'),
                    "content_length": len(response.content)
                }
            except requests.exceptions.RequestException as e:
                results[file_path] = {"status": "error", "error": str(e)}
        
        return results

def test_ui():
    """Test all UI endpoints."""
    print("=== mTLS Proxy UI Test ===\n")
    
    tester = UI tester()
    
    # Test 1: Dashboard
    print("1. Testing dashboard...")
    result = tester.test_dashboard()
    if result["status"] == "success":
        print(f"   ✓ Dashboard accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Dashboard failed: {result['error']}")
    print()
    
    # Test 2: Logs page
    print("2. Testing logs page...")
    result = tester.test_logs()
    if result["status"] == "success":
        print(f"   ✓ Logs page accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Logs page failed: {result['error']}")
    print()
    
    # Test 3: Health page
    print("3. Testing health page...")
    result = tester.test_health()
    if result["status"] == "success":
        print(f"   ✓ Health page accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Health page failed: {result['error']}")
    print()
    
    # Test 4: API logs
    print("4. Testing API logs endpoint...")
    result = tester.test_api_logs()
    if result["status"] == "success":
        print(f"   ✓ API logs working (Status: {result['status_code']}, Data length: {result['data_length']})")
    else:
        print(f"   ✗ API logs failed: {result['error']}")
    print()
    
    # Test 5: API stats
    print("5. Testing API stats endpoint...")
    result = tester.test_api_stats()
    if result["status"] == "success":
        stats = result["stats"]
        print(f"   ✓ API stats working (Status: {result['status_code']})")
        print(f"   - Total requests: {stats.get('total_requests', 0)}")
        print(f"   - Success rate: {stats.get('success_rate', 0):.1f}%")
        print(f"   - Avg response time: {stats.get('avg_response_time', 0):.1f}ms")
    else:
        print(f"   ✗ API stats failed: {result['error']}")
    print()
    
    # Test 6: Static files
    print("6. Testing static files...")
    results = tester.test_static_files()
    for file_path, result in results.items():
        if result["status"] == "success":
            print(f"   ✓ {file_path} accessible ({result['content_length']} bytes)")
        else:
            print(f"   ✗ {file_path} failed: {result['error']}")
    print()
    
    print("=== UI Test completed ===")
    print("\nTo access the UI, open your browser and go to:")
    print("  http://localhost:8080/ui/dashboard")

if __name__ == "__main__":
    test_ui()
