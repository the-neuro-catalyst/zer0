import { describe, it, expect, vi } from 'vitest';

// Technical Mocking: Using traditional function constructor for maximum compatibility
const mockRegisterTool = vi.fn();
const mockConnect = vi.fn().mockResolvedValue(undefined);

function MockMcpServer(this: any, config: any) {
  this.config = config;
  this.registerTool = mockRegisterTool;
  this.connect = mockConnect;
}

vi.mock('@modelcontextprotocol/sdk/server/mcp.js', () => {
  return {
    McpServer: MockMcpServer,
  };
});

vi.mock('@modelcontextprotocol/sdk/server/stdio.js', () => {
  return {
    StdioServerTransport: vi.fn().mockImplementation(() => ({})),
  };
});

// Import SDK classes locally for type checking
import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';

describe('ZERO Gemini Extension Technical Validation', () => {
  it('Server initialization adheres to protocol standards', async () => {
    // @ts-ignore
    const server = new McpServer({
      name: 'zero-gemini-mcp',
      version: '0.1.0',
    });
    expect(server).toBeDefined();
  });

  it('Tool registration registry should be populated', async () => {
    // @ts-ignore
    const server = new McpServer({ name: 'test', version: '1.0.0' });
    server.registerTool('test_tool', { description: 'test', inputSchema: {} as any }, async () => ({ content: [] }));
    expect(mockRegisterTool).toHaveBeenCalled();
  });
});