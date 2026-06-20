"""
mcp:// Protocol - Python Client
Minimal HTTP/JSON implementation
"""

import requests
from typing import Dict, List, Any, Optional
from .exceptions import MCPError, ConnectionError, ToolNotFoundError


class MCPClient:
    """Minimal MCP client for mcp:// protocol"""
    
    def __init__(self, server_url: str, token: Optional[str] = None):
        """
        Initialize MCP client
        
        Args:
            server_url: mcp:// URL or HTTPS URL
            token: Optional authentication token
        """
        # Convert mcp:// to https://
        if server_url.startswith("mcp://"):
            self.base_url = server_url.replace("mcp://", "https://")
        else:
            self.base_url = server_url
            
        self.token = token
        self.session = requests.Session()
        
        if self.token:
            self.session.headers.update({
                "Authorization": f"Bearer {self.token}"
            })
    
    def connect(self) -> bool:
        """
        Connect to MCP server
        
        Returns:
            True if connection successful
        """
        try:
            response = self.session.get(
                self.base_url,
                headers={"Accept": "application/json"},
                timeout=10
            )
            response.raise_for_status()
            return True
        except requests.RequestException as e:
            raise ConnectionError(f"Failed to connect: {e}")
    
    def get_server_info(self) -> Dict[str, Any]:
        """
        Get server information
        
        Returns:
            Server info dictionary
        """
        try:
            response = self.session.get(
                self.base_url,
                headers={"Accept": "application/json"},
                timeout=10
            )
            response.raise_for_status()
            return response.json()
        except requests.RequestException as e:
            raise MCPError(f"Failed to get server info: {e}")
    
    def list_tools(self) -> List[Dict[str, Any]]:
        """
        List available tools
        
        Returns:
            List of tool definitions
        """
        try:
            response = self.session.get(
                f"{self.base_url}/tools/list",
                headers={"Accept": "application/json"},
                timeout=10
            )
            response.raise_for_status()
            data = response.json()
            return data.get("tools", [])
        except requests.RequestException as e:
            raise MCPError(f"Failed to list tools: {e}")
    
    def call_tool(self, tool_name: str, arguments: Dict[str, Any]) -> Any:
        """
        Call a tool
        
        Args:
            tool_name: Name of the tool to call
            arguments: Tool arguments
            
        Returns:
            Tool result
        """
        try:
            response = self.session.post(
                f"{self.base_url}/tools/call",
                json={
                    "tool": tool_name,
                    "arguments": arguments
                },
                headers={"Content-Type": "application/json"},
                timeout=30
            )
            
            if response.status_code == 404:
                raise ToolNotFoundError(f"Tool '{tool_name}' not found")
            
            response.raise_for_status()
            data = response.json()
            
            if not data.get("success", False):
                raise MCPError(f"Tool call failed: {data.get('error', 'Unknown error')}")
            
            return data.get("result")
        except requests.RequestException as e:
            raise MCPError(f"Failed to call tool: {e}")
    
    def close(self):
        """Close the session"""
        self.session.close()
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
