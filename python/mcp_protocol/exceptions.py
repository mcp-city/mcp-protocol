"""
mcp:// Protocol - Exceptions
"""


class MCPError(Exception):
    """Base MCP exception"""
    pass


class ConnectionError(MCPError):
    """Connection failed"""
    pass


class ToolNotFoundError(MCPError):
    """Tool not found on server"""
    pass


class AuthenticationError(MCPError):
    """Authentication failed"""
    pass


class RateLimitError(MCPError):
    """Rate limit exceeded"""
    pass


class InvalidArgumentError(MCPError):
    """Invalid argument provided"""
    pass
