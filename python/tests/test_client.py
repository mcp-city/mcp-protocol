"""
Unit tests for mcp:// Python client
"""

import pytest
import requests
from unittest.mock import Mock, patch
from mcp_protocol import MCPClient, MCPError, ConnectionError, ToolNotFoundError


class TestMCPClient:
    """Test MCP client functionality"""
    
    def test_init_with_mcp_url(self):
        """Test initialization with mcp:// URL"""
        client = MCPClient("mcp://example.com")
        assert client.base_url == "https://example.com"
    
    def test_init_with_https_url(self):
        """Test initialization with HTTPS URL"""
        client = MCPClient("https://example.com")
        assert client.base_url == "https://example.com"
    
    def test_init_with_token(self):
        """Test initialization with token"""
        client = MCPClient("mcp://example.com", token="test_token")
        assert client.session.headers.get("Authorization") == "Bearer test_token"
    
    @patch('mcp_protocol.client.requests.Session.get')
    def test_connect_success(self, mock_get):
        """Test successful connection"""
        mock_response = Mock()
        mock_response.status_code = 200
        mock_get.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        result = client.connect()
        
        assert result is True
        mock_get.assert_called_once()
    
    @patch('mcp_protocol.client.requests.Session.get')
    def test_connect_failure(self, mock_get):
        """Test connection failure"""
        mock_get.side_effect = requests.RequestException("Connection failed")
        
        client = MCPClient("mcp://example.com")
        
        with pytest.raises(ConnectionError):
            client.connect()
    
    @patch('mcp_protocol.client.requests.Session.get')
    def test_get_server_info(self, mock_get):
        """Test getting server info"""
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "name": "Test Server",
            "version": "1.0.0"
        }
        mock_get.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        info = client.get_server_info()
        
        assert info["name"] == "Test Server"
        assert info["version"] == "1.0.0"
    
    @patch('mcp_protocol.client.requests.Session.get')
    def test_list_tools(self, mock_get):
        """Test listing tools"""
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "tools": [
                {
                    "name": "echo",
                    "description": "Echo message"
                }
            ]
        }
        mock_get.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        tools = client.list_tools()
        
        assert len(tools) == 1
        assert tools[0]["name"] == "echo"
    
    @patch('mcp_protocol.client.requests.Session.post')
    def test_call_tool_success(self, mock_post):
        """Test successful tool call"""
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "result": "Echo: Hello",
            "success": True
        }
        mock_post.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        result = client.call_tool("echo", {"message": "Hello"})
        
        assert result == "Echo: Hello"
    
    @patch('mcp_protocol.client.requests.Session.post')
    def test_call_tool_not_found(self, mock_post):
        """Test tool not found error"""
        mock_response = Mock()
        mock_response.status_code = 404
        mock_post.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        
        with pytest.raises(ToolNotFoundError):
            client.call_tool("nonexistent", {})
    
    @patch('mcp_protocol.client.requests.Session.post')
    def test_call_tool_failure(self, mock_post):
        """Test tool call failure"""
        mock_response = Mock()
        mock_response.status_code = 200
        mock_response.json.return_value = {
            "success": False,
            "error": "Invalid argument"
        }
        mock_post.return_value = mock_response
        
        client = MCPClient("mcp://example.com")
        
        with pytest.raises(MCPError):
            client.call_tool("echo", {})
    
    def test_context_manager(self):
        """Test using client as context manager"""
        with patch('mcp_protocol.client.requests.Session') as mock_session:
            client = MCPClient("mcp://example.com")
            with client:
                pass
            client.session.close.assert_called_once()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
