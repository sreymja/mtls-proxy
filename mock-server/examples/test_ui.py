#!/usr/bin/env python3
"""
Test script for the Mock GPT-4o-mini Server UI.
This script tests the web interface endpoints.
"""

import requests
import json
import time
from typing import Dict, Any

class MockServerUITester:
    def __init__(self, base_url: str = "https://localhost:8444"):
        self.base_url = base_url.rstrip('/')
        self.session = requests.Session()
        # Disable SSL verification for testing
        self.session.verify = False
        import urllib3
        urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)
    
    def test_dashboard(self) -> Dict[str, Any]:
        """Test the dashboard endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/dashboard", timeout=10)
            response.raise_for_status()
            return {"status": "success", "status_code": response.status_code, "content_type": response.headers.get('content-type')}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_requests(self) -> Dict[str, Any]:
        """Test the requests endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/requests", timeout=10)
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
    
    def test_api_requests(self) -> Dict[str, Any]:
        """Test the API requests endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/ui/api/requests?limit=10", timeout=10)
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

def test_mock_server_ui():
    """Test all UI endpoints."""
    print("=== Mock GPT-4o-mini Server UI Test ===\n")
    
    tester = MockServerUITester()
    
    # Test 1: Dashboard
    print("1. Testing dashboard...")
    result = tester.test_dashboard()
    if result["status"] == "success":
        print(f"   ✓ Dashboard accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Dashboard failed: {result['error']}")
    print()
    
    # Test 2: Requests page
    print("2. Testing requests page...")
    result = tester.test_requests()
    if result["status"] == "success":
        print(f"   ✓ Requests page accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Requests page failed: {result['error']}")
    print()
    
    # Test 3: Health page
    print("3. Testing health page...")
    result = tester.test_health()
    if result["status"] == "success":
        print(f"   ✓ Health page accessible (Status: {result['status_code']})")
    else:
        print(f"   ✗ Health page failed: {result['error']}")
    print()
    
    # Test 4: API requests
    print("4. Testing API requests endpoint...")
    result = tester.test_api_requests()
    if result["status"] == "success":
        print(f"   ✓ API requests working (Status: {result['status_code']}, Data length: {result['data_length']})")
    else:
        print(f"   ✗ API requests failed: {result['error']}")
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
    print("  https://localhost:8443/ui/dashboard")
    print("\nNote: You may need to accept the self-signed certificate warning.")

if __name__ == "__main__":
    test_mock_server_ui()
