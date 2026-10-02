import type { BaseRequestOptions, YouVersionPlatformClient } from '@cameronapak/platform-sdk';

export interface Schema {
  type?: string;
  anyOf?: Schema[];
  items?: Schema;
  enum?: unknown[];
  properties?: Record<string, Schema>;
  required?: string[];
  [keyword: string]: unknown;
}

export interface Input {
  name: string;
  flag: string;
  location: 'path' | 'query' | 'header';
  required: boolean;
  description: string;
  schema: Schema;
  secretEnv?: string;
}

export interface Operation {
  operationId: string;
  command: string[];
  httpMethod: string;
  path: string;
  description: string;
  kind: 'read' | 'write' | 'excluded';
  auth: 'app-key' | 'oauth' | 'exchange-token' | 'approval';
  sensitive: boolean;
  requiresDisclosure: boolean;
  response: 'json' | 'text' | 'empty';
  redirect: boolean;
  inputs: Input[];
  body?: Schema;
  exclusionReason?: string;
  execute(client: YouVersionPlatformClient, request: Record<string, unknown>, options: BaseRequestOptions): Promise<{
    data: unknown;
    rawResponse: { status: number; headers: { get(name: string): string | null } };
  }>;
}
