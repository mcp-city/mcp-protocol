/**
 * mcp:// Protocol - Exceptions
 */

export class MCPError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'MCPError';
  }
}

export class ConnectionError extends MCPError {
  constructor(message: string) {
    super(message);
    this.name = 'ConnectionError';
  }
}

export class ToolNotFoundError extends MCPError {
  constructor(message: string) {
    super(message);
    this.name = 'ToolNotFoundError';
  }
}

export class AuthenticationError extends MCPError {
  constructor(message: string) {
    super(message);
    this.name = 'AuthenticationError';
  }
}

export class RateLimitError extends MCPError {
  constructor(message: string) {
    super(message);
    this.name = 'RateLimitError';
  }
}

export class InvalidArgumentError extends MCPError {
  constructor(message: string) {
    super(message);
    this.name = 'InvalidArgumentError';
  }
}
