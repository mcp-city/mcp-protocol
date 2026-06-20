/**
 * mcp:// Protocol - JavaScript/TypeScript Client
 * Minimal HTTP/JSON implementation
 */

import { MCPError, ConnectionError, ToolNotFoundError } from './exceptions';

export interface ServerInfo {
  name: string;
  version: string;
  protocol: string;
  capabilities: string[];
}

export interface Tool {
  name: string;
  description: string;
  inputSchema: any;
}

export interface ToolCallResult {
  result: any;
  success: boolean;
  error?: string;
}

export class MCPClient {
  private baseUrl: string;
  private token?: string;
  private headers: Record<string, string>;

  constructor(serverUrl: string, token?: string) {
    // Convert mcp:// to https://
    if (serverUrl.startsWith('mcp://')) {
      this.baseUrl = serverUrl.replace('mcp://', 'https://');
    } else {
      this.baseUrl = serverUrl;
    }

    this.token = token;
    this.headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json',
    };

    if (this.token) {
      this.headers['Authorization'] = `Bearer ${this.token}`;
    }
  }

  async connect(): Promise<boolean> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'GET',
        headers: this.headers,
      });

      if (!response.ok) {
        throw new ConnectionError(`Failed to connect: ${response.statusText}`);
      }

      return true;
    } catch (error) {
      throw new ConnectionError(`Failed to connect: ${error}`);
    }
  }

  async getServerInfo(): Promise<ServerInfo> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'GET',
        headers: this.headers,
      });

      if (!response.ok) {
        throw new MCPError(`Failed to get server info: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      throw new MCPError(`Failed to get server info: ${error}`);
    }
  }

  async listTools(): Promise<Tool[]> {
    try {
      const response = await fetch(`${this.baseUrl}/tools/list`, {
        method: 'GET',
        headers: this.headers,
      });

      if (!response.ok) {
        throw new MCPError(`Failed to list tools: ${response.statusText}`);
      }

      const data = await response.json();
      return data.tools || [];
    } catch (error) {
      throw new MCPError(`Failed to list tools: ${error}`);
    }
  }

  async callTool(toolName: string, arguments_: Record<string, any>): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/tools/call`, {
        method: 'POST',
        headers: this.headers,
        body: JSON.stringify({
          tool: toolName,
          arguments: arguments_,
        }),
      });

      if (response.status === 404) {
        throw new ToolNotFoundError(`Tool '${toolName}' not found`);
      }

      if (!response.ok) {
        throw new MCPError(`Failed to call tool: ${response.statusText}`);
      }

      const data: ToolCallResult = await response.json();

      if (!data.success) {
        throw new MCPError(`Tool call failed: ${data.error || 'Unknown error'}`);
      }

      return data.result;
    } catch (error) {
      if (error instanceof MCPError) {
        throw error;
      }
      throw new MCPError(`Failed to call tool: ${error}`);
    }
  }

  close(): void {
    // No-op for fetch-based client
    // For WebSocket or persistent connections, cleanup would go here
  }
}
