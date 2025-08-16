#!/usr/bin/env python3
"""
Test script for the mTLS proxy server.
This script demonstrates how to use the proxy to communicate with a private GPT-4o-mini instance.
"""

import requests
import json
import time
from typing import Dict, Any

class MTLSProxyClient:
    def __init__(self, proxy_url: str = "http://localhost:8080"):
        self.proxy_url = proxy_url.rstrip('/')
        self.session = requests.Session()
        
    def chat_completion(self, messages: list, model: str = "gpt-4o-mini", **kwargs) -> Dict[str, Any]:
        """
        Send a chat completion request through the mTLS proxy.
        
        Args:
            messages: List of message dictionaries with 'role' and 'content'
            model: Model name to use
            **kwargs: Additional parameters for the API call
            
        Returns:
            API response as dictionary
        """
        url = f"{self.proxy_url}/v1/chat/completions"
        
        payload = {
            "model": model,
            "messages": messages,
            **kwargs
        }
        
        headers = {
            "Content-Type": "application/json",
            "Authorization": "Bearer your-api-key-here"  # Replace with actual API key
        }
        
        try:
            response = self.session.post(url, json=payload, headers=headers, timeout=60)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Request failed: {e}")
            return {"error": str(e)}

def test_basic_chat():
    """Test basic chat completion functionality."""
    client = MTLSProxyClient()
    
    messages = [
        {"role": "user", "content": "Hello! Can you tell me a short joke?"}
    ]
    
    print("Sending chat completion request...")
    start_time = time.time()
    
    response = client.chat_completion(messages, max_tokens=100)
    
    duration = time.time() - start_time
    print(f"Request completed in {duration:.2f} seconds")
    
    if "error" in response:
        print(f"Error: {response['error']}")
    else:
        print("Response:")
        print(json.dumps(response, indent=2))
        
        if "choices" in response and response["choices"]:
            content = response["choices"][0]["message"]["content"]
            print(f"\nAI Response: {content}")

def test_streaming_chat():
    """Test streaming chat completion."""
    client = MTLSProxyClient()
    
    messages = [
        {"role": "user", "content": "Write a short story about a robot learning to paint."}
    ]
    
    print("Sending streaming chat completion request...")
    
    url = f"{client.proxy_url}/v1/chat/completions"
    payload = {
        "model": "gpt-4o-mini",
        "messages": messages,
        "stream": True,
        "max_tokens": 200
    }
    
    headers = {
        "Content-Type": "application/json",
        "Authorization": "Bearer your-api-key-here"
    }
    
    try:
        response = client.session.post(url, json=payload, headers=headers, stream=True, timeout=60)
        response.raise_for_status()
        
        print("Streaming response:")
        for line in response.iter_lines():
            if line:
                line = line.decode('utf-8')
                if line.startswith('data: '):
                    data = line[6:]  # Remove 'data: ' prefix
                    if data == '[DONE]':
                        break
                    try:
                        chunk = json.loads(data)
                        if 'choices' in chunk and chunk['choices']:
                            delta = chunk['choices'][0].get('delta', {})
                            if 'content' in delta:
                                print(delta['content'], end='', flush=True)
                    except json.JSONDecodeError:
                        continue
        print()  # New line at the end
        
    except requests.exceptions.RequestException as e:
        print(f"Streaming request failed: {e}")

def test_error_handling():
    """Test error handling with invalid requests."""
    client = MTLSProxyClient()
    
    # Test with invalid model
    messages = [{"role": "user", "content": "Hello"}]
    
    print("Testing error handling with invalid model...")
    response = client.chat_completion(messages, model="invalid-model")
    
    if "error" in response:
        print(f"Expected error: {response['error']}")
    else:
        print("Response (should contain error):")
        print(json.dumps(response, indent=2))

def test_connection():
    """Test basic connection to the proxy."""
    client = MTLSProxyClient()
    
    try:
        # Try to make a simple request to see if proxy is reachable
        response = client.session.get(f"{client.proxy_url}/health", timeout=5)
        print(f"Proxy health check: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"Proxy connection test failed: {e}")
        print("Make sure the proxy server is running on localhost:8080")

if __name__ == "__main__":
    print("=== mTLS Proxy Test Script ===\n")
    
    # Test 1: Connection check
    print("1. Testing proxy connection...")
    test_connection()
    print()
    
    # Test 2: Basic chat completion
    print("2. Testing basic chat completion...")
    test_basic_chat()
    print()
    
    # Test 3: Streaming chat completion
    print("3. Testing streaming chat completion...")
    test_streaming_chat()
    print()
    
    # Test 4: Error handling
    print("4. Testing error handling...")
    test_error_handling()
    print()
    
    print("=== Test completed ===")
