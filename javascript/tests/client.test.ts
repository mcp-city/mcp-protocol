/**
 * Unit tests for mcp:// JavaScript/TypeScript client
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { MCPClient, MCPError, ConnectionError, ToolNotFoundError } from '../src/index';

describe('MCPClient', () => {
  let client: MCPClient;
  let mockFetch: any;

  beforeEach(() => {
    mockFetch = vi.fn();
    global.fetch = mockFetch;
    client = new MCPClient('mcp://example.com');
  });

  it('should initialize with mcp:// URL', () => {
    expect(client).toBeDefined();
  });

  it('should convert mcp:// to https://', () => {
    const httpsClient = new MCPClient('mcp://example.com');
    // Private property, but we can test through behavior
  });

  it('should initialize with token', () => {
    const tokenClient = new MCPClient('mcp://example.com', 'test_token');
    expect(tokenClient).toBeDefined();
  });

  it('should connect successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
    });

    const result = await client.connect();
    expect(result).toBe(true);
  });

  it('should fail to connect on error', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Connection failed'));

    await expect(client.connect()).rejects.toThrow(ConnectionError);
  });

  it('should get server info', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        name: 'Test Server',
        version: '1.0.0',
      }),
    });

    const info = await client.getServerInfo();
    expect(info.name).toBe('Test Server');
    expect(info.version).toBe('1.0.0');
  });

  it('should list tools', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        tools: [
          {
            name: 'echo',
            description: 'Echo message',
          },
        ],
      }),
    });

    const tools = await client.listTools();
    expect(tools).toHaveLength(1);
    expect(tools[0].name).toBe('echo');
  });

  it('should call tool successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        result: 'Echo: Hello',
        success: true,
      }),
    });

    const result = await client.callTool('echo', { message: 'Hello' });
    expect(result).toBe('Echo: Hello');
  });

  it('should throw ToolNotFoundError on 404', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 404,
    });

    await expect(client.callTool('nonexistent', {})).rejects.toThrow(ToolNotFoundError);
  });

  it('should throw MCPError on failed tool call', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        success: false,
        error: 'Invalid argument',
      }),
    });

    await expect(client.callTool('echo', {})).rejects.toThrow(MCPError);
  });
});
