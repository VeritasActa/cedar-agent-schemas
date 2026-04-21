# cedar-agent-schemas (Python)

Python helpers for constructing Cedar authorization requests against the canonical agent action schema at [`schemas/agent-actions.cedarschema.json`](../../schemas/agent-actions.cedarschema.json).

## Install

```bash
pip install cedar-agent-schemas
```

## Usage

```python
from cedar_agent_schemas import (
    AgentPrincipal,
    RequestToolContext,
    build_request_tool_request,
)

req = build_request_tool_request(
    principal=AgentPrincipal(
        agent_id="claude-code-3a2f",
        trust_score="0.85",
        ring=2,
        session_id="sess-9af21",
    ),
    tool_id="Bash",
    context=RequestToolContext(
        args_hash="sha256:e4d61f7a...",
        transport="mcp_stdio",
    ),
)

# req.to_dict() is a Cedar-ready authorization request:
# {
#     "principal": "Agent::Principal::\"claude-code-3a2f\"",
#     "action":    "Agent::Action::\"request_tool\"",
#     "resource":  "Agent::Tool::\"Bash\"",
#     "context":   {"args_hash": "sha256:...", "transport": "mcp_stdio"},
#     "entities":  []
# }
```

Pass `req.to_dict()` to any Cedar evaluator (cedar-py, cedar-wasm via PyO3, or a remote evaluator).

## Exported

### Types

`AgentPrincipal`, `AgentFile`, `AgentEndpoint`, `AgentTool`, `AgentExecutable` — canonical entity attribute shapes (dataclasses).

`ExecContext`, `OpenContext`, `ConnectContext`, `RequestToolContext` — per-action context shapes.

`CedarAuthorizationRequest` — the request shape expected by Cedar evaluators (with `.to_dict()` serializer).

`ActionVerb` — `"exec" | "open" | "connect" | "request_tool"`.

### Builders

`build_exec_request`, `build_open_request`, `build_connect_request`, `build_request_tool_request`.

`entity_uid(type, id)`, `action_uid(verb)` — low-level UID builders.

### Constants

`SCHEMA_NAMESPACE = "Agent"`.

## Schema version

This package version tracks the schema version. `cedar-agent-schemas==0.1.x` uses `agent-actions.cedarschema.json` at v0.1 (additive-only between minor versions).

## License

Apache-2.0. Same license as Cedar itself.
