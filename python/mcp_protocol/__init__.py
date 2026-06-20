"""
mcp:// Protocol - Python Client Library
Minimal implementation for open source compatibility
"""

from .client import MCPClient
from .exceptions import MCPError, ConnectionError, ToolNotFoundError

__version__ = "0.1.0"
__all__ = ["MCPClient", "MCPError", "ConnectionError", "ToolNotFoundError"]
