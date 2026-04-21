# cedar-agent-schemas (Rust)

Rust helpers for constructing Cedar authorization requests against the canonical agent action schema at [`schemas/agent-actions.cedarschema.json`](../../schemas/agent-actions.cedarschema.json).

## Install

```toml
[dependencies]
cedar-agent-schemas = "0.1"
```

## Usage

```rust
use cedar_agent_schemas::{
    AgentPrincipal, RequestToolContext, build_request_tool_request,
};

let principal = AgentPrincipal {
    agent_id: "claude-code-3a2f".into(),
    trust_score: "0.85".into(),
    ring: 2,
    session_id: Some("sess-9af21".into()),
};

let ctx = RequestToolContext {
    args_hash: "sha256:e4d61f7a...".into(),
    transport: Some("mcp_stdio".into()),
};

let req = build_request_tool_request(&principal, "Bash", &ctx, None);

// serde_json::to_string(&req).unwrap() produces:
// {
//   "principal": "Agent::Principal::\"claude-code-3a2f\"",
//   "action":    "Agent::Action::\"request_tool\"",
//   "resource":  "Agent::Tool::\"Bash\"",
//   "context":   {"args_hash":"sha256:e4d61f7a...","transport":"mcp_stdio"},
//   "entities":  []
// }
```

Pass the serialized request to any Cedar evaluator (cedar-policy crate, cedar-wasm, or a remote evaluator).

## What this crate is

A typed-builder layer on top of the canonical agent action schema. It does NOT:

- evaluate Cedar policies (use the `cedar-policy` crate for that)
- sign receipts (use `@veritasacta/verify` downstream)
- include an embedded schema (the `.cedarschema.json` lives in the repo root, not bundled into the crate)

What it DOES:

- Canonical Cedar entity UID construction (`entity_uid`, `action_uid`)
- Typed structs for the five canonical entity types (Principal, File, Endpoint, Tool, Executable)
- Typed context structs for each of the four action verbs
- Request builders that handle UID formatting and context serialization
- Serde (de)serialization throughout

## Schema version

This crate version tracks the schema version. `cedar-agent-schemas` 0.1.x uses `agent-actions.cedarschema.json` at v0.1 (additive-only between minor versions).

## License

Apache-2.0. Same license as Cedar itself.
