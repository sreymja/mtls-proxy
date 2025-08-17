#!/usr/bin/env python3
"""
Test script for the mock GPT-4o-mini API server.
This script demonstrates how to test the mock server directly and through the mTLS proxy.
"""

import requests
import json
import time
import ssl
from typing import Dict, Any

class MockServerTester:
    def __init__(self, base_url: str = "https://localhost:8444", verify_ssl: bool = False):
        self.base_url = base_url.rstrip('/')
        self.session = requests.Session()
        
        if not verify_ssl:
            # Disable SSL verification for testing
            self.session.verify = False
            # Suppress SSL warnings
            import urllib3
            urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)
    
    def test_health(self) -> Dict[str, Any]:
        """Test the health endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/health", timeout=10)
            response.raise_for_status()
            return {"status": "success", "data": response.json()}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_models(self) -> Dict[str, Any]:
        """Test the models endpoint."""
        try:
            response = self.session.get(f"{self.base_url}/v1/models", timeout=10)
            response.raise_for_status()
            return {"status": "success", "data": response.json()}
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_chat_completion(self, messages: list, stream: bool = False) -> Dict[str, Any]:
        """Test the chat completions endpoint."""
        url = f"{self.base_url}/v1/chat/completions"
        
        payload = {
            "model": "gpt-4o-mini",
            "messages": messages,
            "stream": stream,
            "max_tokens": 100,
            "temperature": 0.7
        }
        
        try:
            if stream:
                response = self.session.post(url, json=payload, stream=True, timeout=30)
                response.raise_for_status()
                
                # Process streaming response
                chunks = []
                for line in response.iter_lines():
                    if line:
                        line = line.decode('utf-8')
                        if line.startswith('data: '):
                            data = line[6:]  # Remove 'data: ' prefix
                            if data == '[DONE]':
                                break
                            try:
                                chunk = json.loads(data)
                                chunks.append(chunk)
                            except json.JSONDecodeError:
                                continue
                
                return {"status": "success", "data": chunks}
            else:
                response = self.session.post(url, json=payload, timeout=30)
                response.raise_for_status()
                return {"status": "success", "data": response.json()}
                
        except requests.exceptions.RequestException as e:
            return {"status": "error", "error": str(e)}
    
    def test_error_scenarios(self) -> Dict[str, Any]:
        """Test various error scenarios."""
        results = {}
        
        # Test invalid model
        try:
            response = self.session.post(
                f"{self.base_url}/v1/chat/completions",
                json={
                    "model": "invalid-model",
                    "messages": [{"role": "user", "content": "Hello"}]
                },
                timeout=10
            )
            results["invalid_model"] = {
                "status_code": response.status_code,
                "response": response.json() if response.status_code != 200 else None
            }
        except Exception as e:
            results["invalid_model"] = {"error": str(e)}
        
        # Test invalid JSON
        try:
            response = self.session.post(
                f"{self.base_url}/v1/chat/completions",
                data="invalid json",
                headers={"Content-Type": "application/json"},
                timeout=10
            )
            results["invalid_json"] = {
                "status_code": response.status_code,
                "response": response.json() if response.status_code != 200 else None
            }
        except Exception as e:
            results["invalid_json"] = {"error": str(e)}
        
        # Test non-existent endpoint
        try:
            response = self.session.get(f"{self.base_url}/v1/nonexistent", timeout=10)
            results["nonexistent_endpoint"] = {
                "status_code": response.status_code,
                "response": response.json() if response.status_code != 200 else None
            }
        except Exception as e:
            results["nonexistent_endpoint"] = {"error": str(e)}
        
        return results

def test_mock_server_directly():
    """Test the mock server directly."""
    print("=== Testing Mock Server Directly ===\n")
    
    tester = MockServerTester("https://localhost:8443")
    
    # Test 1: Health check
    print("1. Testing health endpoint...")
    result = tester.test_health()
    if result["status"] == "success":
        print(f"   ✓ Health check passed: {result['data']}")
    else:
        print(f"   ✗ Health check failed: {result['error']}")
    print()
    
    # Test 2: Models endpoint
    print("2. Testing models endpoint...")
    result = tester.test_models()
    if result["status"] == "success":
        models = result["data"]["data"]
        print(f"   ✓ Models endpoint passed: {len(models)} models available")
        for model in models:
            print(f"     - {model['id']}")
    else:
        print(f"   ✗ Models endpoint failed: {result['error']}")
    print()
    
    # Test 3: Chat completion (non-streaming)
    print("3. Testing chat completion (non-streaming)...")
    messages = [{"role": "user", "content": "Hello! Can you tell me a joke?"}]
    result = tester.test_chat_completion(messages, stream=False)
    if result["status"] == "success":
        response = result["data"]
        content = response["choices"][0]["message"]["content"]
        print(f"   ✓ Chat completion passed")
        print(f"   Response: {content[:100]}...")
    else:
        print(f"   ✗ Chat completion failed: {result['error']}")
    print()
    
    # Test 4: Chat completion (streaming)
    print("4. Testing chat completion (streaming)...")
    messages = [{"role": "user", "content": "Write a short story about a robot."}]
    result = tester.test_chat_completion(messages, stream=True)
    if result["status"] == "success":
        chunks = result["data"]
        print(f"   ✓ Streaming chat completion passed: {len(chunks)} chunks")
        # Show first chunk content
        if chunks and "choices" in chunks[0] and chunks[0]["choices"]:
            delta = chunks[0]["choices"][0].get("delta", {})
            if "content" in delta:
                print(f"   First chunk: {delta['content'][:50]}...")
    else:
        print(f"   ✗ Streaming chat completion failed: {result['error']}")
    print()
    
    # Test 5: Error scenarios
    print("5. Testing error scenarios...")
    results = tester.test_error_scenarios()
    for scenario, result in results.items():
        if "error" in result:
            print(f"   ✗ {scenario}: {result['error']}")
        else:
            status_code = result["status_code"]
            if status_code >= 400:
                print(f"   ✓ {scenario}: Expected error (status {status_code})")
            else:
                print(f"   ⚠ {scenario}: Unexpected success (status {status_code})")
    print()

def test_through_proxy():
    """Test the mock server through the mTLS proxy."""
    print("=== Testing Through mTLS Proxy ===\n")
    
    # Note: This requires the proxy to be configured to point to the mock server
    tester = MockServerTester("http://localhost:8080")
    
    print("1. Testing health through proxy...")
    result = tester.test_health()
    if result["status"] == "success":
        print(f"   ✓ Health check through proxy passed")
    else:
        print(f"   ✗ Health check through proxy failed: {result['error']}")
    print()
    
    print("2. Testing chat completion through proxy...")
    messages = [{"role": "user", "content": "Hello through proxy!"}]
    result = tester.test_chat_completion(messages, stream=False)
    if result["status"] == "success":
        response = result["data"]
        content = response["choices"][0]["message"]["content"]
        print(f"   ✓ Chat completion through proxy passed")
        print(f"   Response: {content[:100]}...")
    else:
        print(f"   ✗ Chat completion through proxy failed: {result['error']}")
    print()

def main():
    print("=== Mock GPT-4o-mini API Server Test Script ===\n")
    
    # Test 1: Direct connection to mock server
    test_mock_server_directly()
    
    # Test 2: Connection through proxy (if available)
    print("Note: Proxy testing requires the mTLS proxy to be running and configured")
    print("to point to the mock server at https://localhost:8443")
    print()
    
    response = input("Do you want to test through the proxy? (y/N): ")
    if response.lower() == 'y':
        test_through_proxy()
    
    print("=== Test completed ===")

if __name__ == "__main__":
    main()
