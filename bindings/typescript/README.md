# @veritasacta/cedar-agent-schemas

TypeScript helpers for constructing Cedar authorization requests against the canonical agent action schema at [schemas/agent-actions.cedarschema.json](../../schemas/agent-actions.cedarschema.json).

## Install

```bash
npm install @veritasacta/cedar-agent-schemas
```

## Usage

```typescript
import { buildRequestToolRequest } from "@veritasacta/cedar-agent-schemas";

const req = buildRequestToolRequest({
  principal: {
    agent_id: "claude-code-3a2f",
    trust_score: "0.85",
    ring: 2,
    session_id: "sess-9af21",
  },
  tool: { id: "Bash", name: "Bash", server: "mcp-filesystem" },
  context: {
    args_hash: "sha256:e4d61f7a...",
    transport: "mcp_stdio",
  },
});

// req is a Cedar-ready authorization request:
// {
//   principal: "Agent::Principal::\"claude-code-3a2f\"",
//   action:    "Agent::Action::\"request_tool\"",
//   resource:  "Agent::Tool::\"Bash\"",
//   context:   { args_hash: "sha256:...", transport: "mcp_stdio" },
//   entities:  []
// }
```

Pass `req` to any Cedar evaluator (e.g., cedar-wasm, cedar-for-agents WASM bindings, cedar-policy Rust API) along with your policy set.

## Exported

### Types

- `AgentPrincipal`, `AgentFile`, `AgentEndpoint`, `AgentTool`, `AgentExecutable` — canonical entity attribute shapes
- `ExecContext`, `OpenContext`, `ConnectContext`, `RequestToolContext` — per-action context shapes
- `CedarAuthorizationRequest` — the request shape expected by Cedar evaluators
- `ActionVerb` — `"exec" | "open" | "connect" | "request_tool"`

### Builders

- `buildExecRequest`, `buildOpenRequest`, `buildConnectRequest`, `buildRequestToolRequest` — typed request constructors
- `entityUid(type, id)`, `actionUid(verb)` — low-level string builders for Cedar UIDs

### Constants

- `SCHEMA_NAMESPACE` = `"Agent"`

## Schema version

This package version tracks the schema version. `@veritasacta/cedar-agent-schemas@0.1.x` uses `agent-actions.cedarschema.json` at v0.1.

## License

Apache-2.0. See root [LICENSE](../../LICENSE).
