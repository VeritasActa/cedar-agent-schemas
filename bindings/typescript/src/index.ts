// Canonical Cedar agent action verbs with typed helpers for
// constructing Cedar authorization requests.
//
// The underlying schema is agent-actions.cedarschema.json. These types
// mirror that schema so downstream systems can construct requests in a
// type-safe way without hand-assembling JSON.

export const SCHEMA_NAMESPACE = "Agent";

export type ActionVerb = "exec" | "open" | "connect" | "request_tool";

/** Canonical Agent::Principal::Agent attributes. */
export interface AgentPrincipal {
  agent_id: string;
  trust_score: string; // Decimal, as string for Cedar precision
  ring: number;
  session_id?: string;
}

/** Canonical Agent::File attributes. */
export interface AgentFile {
  path: string;
  owner_uid?: number;
}

/** Canonical Agent::Endpoint attributes. */
export interface AgentEndpoint {
  host: string;
  port: number;
  protocol: string;
}

/** Canonical Agent::Tool attributes. */
export interface AgentTool {
  name: string;
  server?: string;
}

/** Canonical Agent::Executable attributes. */
export interface AgentExecutable {
  path: string;
  trusted?: boolean;
}

/** Context for Agent::Action::"exec" requests. */
export interface ExecContext {
  command: string;
  argv: string[];
  cwd?: string;
  uid?: number;
}

/** Context for Agent::Action::"open" requests. */
export interface OpenContext {
  mode: "read" | "write" | "append" | string;
  size_bytes?: number;
}

/** Context for Agent::Action::"connect" requests. */
export interface ConnectContext {
  tls: boolean;
  method?: string;
  url?: string;
}

/** Context for Agent::Action::"request_tool" requests. */
export interface RequestToolContext {
  args_hash: string;
  transport?: string;
}

/** Canonical Cedar authorization request shape. */
export interface CedarAuthorizationRequest {
  principal: string;
  action: string;
  resource: string;
  context: Record<string, unknown>;
  entities: unknown[];
  schema?: unknown;
}

/** Build a fully-qualified Cedar entity UID string. */
export function entityUid(type: string, id: string): string {
  return `${SCHEMA_NAMESPACE}::${type}::"${id}"`;
}

/** Build a fully-qualified Cedar action UID string. */
export function actionUid(verb: ActionVerb): string {
  return `${SCHEMA_NAMESPACE}::Action::"${verb}"`;
}

/** Build a Cedar authorization request for an exec action. */
export function buildExecRequest(args: {
  principal: AgentPrincipal;
  executable: AgentExecutable & { id: string };
  context: ExecContext;
  entities?: unknown[];
}): CedarAuthorizationRequest {
  return {
    principal: entityUid("Principal", args.principal.agent_id),
    action: actionUid("exec"),
    resource: entityUid("Executable", args.executable.id),
    context: args.context as unknown as Record<string, unknown>,
    entities: args.entities ?? [],
  };
}

/** Build a Cedar authorization request for an open action. */
export function buildOpenRequest(args: {
  principal: AgentPrincipal;
  file: AgentFile & { id: string };
  context: OpenContext;
  entities?: unknown[];
}): CedarAuthorizationRequest {
  return {
    principal: entityUid("Principal", args.principal.agent_id),
    action: actionUid("open"),
    resource: entityUid("File", args.file.id),
    context: args.context as unknown as Record<string, unknown>,
    entities: args.entities ?? [],
  };
}

/** Build a Cedar authorization request for a connect action. */
export function buildConnectRequest(args: {
  principal: AgentPrincipal;
  endpoint: AgentEndpoint & { id: string };
  context: ConnectContext;
  entities?: unknown[];
}): CedarAuthorizationRequest {
  return {
    principal: entityUid("Principal", args.principal.agent_id),
    action: actionUid("connect"),
    resource: entityUid("Endpoint", args.endpoint.id),
    context: args.context as unknown as Record<string, unknown>,
    entities: args.entities ?? [],
  };
}

/** Build a Cedar authorization request for a request_tool action. */
export function buildRequestToolRequest(args: {
  principal: AgentPrincipal;
  tool: AgentTool & { id: string };
  context: RequestToolContext;
  entities?: unknown[];
}): CedarAuthorizationRequest {
  return {
    principal: entityUid("Principal", args.principal.agent_id),
    action: actionUid("request_tool"),
    resource: entityUid("Tool", args.tool.id),
    context: args.context as unknown as Record<string, unknown>,
    entities: args.entities ?? [],
  };
}
