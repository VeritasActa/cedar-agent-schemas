"""Canonical Cedar agent action verbs with typed helpers.

Python equivalents of the TypeScript builders in the sibling npm package
(``@veritasacta/cedar-agent-schemas``). The underlying schema is the
same file; these helpers produce Cedar authorization request dicts that
any Cedar evaluator (cedar-py, cedar-wasm via PyO3 bindings, or a
remote evaluator) can consume.

See the canonical schema at
``schemas/agent-actions.cedarschema.json`` in the repository root.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Literal, Optional

SCHEMA_NAMESPACE = "Agent"

ActionVerb = Literal["exec", "open", "connect", "request_tool"]


@dataclass
class AgentPrincipal:
    """Canonical ``Agent::Principal`` attributes."""

    agent_id: str
    trust_score: str  # Decimal, serialized as string for Cedar precision
    ring: int
    session_id: Optional[str] = None


@dataclass
class AgentFile:
    path: str
    owner_uid: Optional[int] = None


@dataclass
class AgentEndpoint:
    host: str
    port: int
    protocol: str


@dataclass
class AgentTool:
    name: str
    server: Optional[str] = None


@dataclass
class AgentExecutable:
    path: str
    trusted: Optional[bool] = None


@dataclass
class ExecContext:
    command: str
    argv: list[str]
    cwd: Optional[str] = None
    uid: Optional[int] = None


@dataclass
class OpenContext:
    mode: str  # "read" | "write" | "append" | etc.
    size_bytes: Optional[int] = None


@dataclass
class ConnectContext:
    tls: bool
    method: Optional[str] = None
    url: Optional[str] = None


@dataclass
class RequestToolContext:
    args_hash: str
    transport: Optional[str] = None


@dataclass
class CedarAuthorizationRequest:
    """Canonical Cedar authorization request shape."""

    principal: str
    action: str
    resource: str
    context: dict[str, Any]
    entities: list[Any] = field(default_factory=list)
    schema: Optional[Any] = None

    def to_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {
            "principal": self.principal,
            "action": self.action,
            "resource": self.resource,
            "context": self.context,
            "entities": self.entities,
        }
        if self.schema is not None:
            out["schema"] = self.schema
        return out


def entity_uid(type_: str, id_: str) -> str:
    """Build a fully-qualified Cedar entity UID string."""

    return f'{SCHEMA_NAMESPACE}::{type_}::"{id_}"'


def action_uid(verb: ActionVerb) -> str:
    """Build a fully-qualified Cedar action UID string."""

    return f'{SCHEMA_NAMESPACE}::Action::"{verb}"'


def _context_to_dict(ctx: Any) -> dict[str, Any]:
    """Drop None fields so the Cedar evaluator sees only set attrs."""

    if hasattr(ctx, "__dict__"):
        return {k: v for k, v in ctx.__dict__.items() if v is not None}
    if isinstance(ctx, dict):
        return {k: v for k, v in ctx.items() if v is not None}
    raise TypeError(f"Unsupported context type: {type(ctx)!r}")


def build_exec_request(
    principal: AgentPrincipal,
    executable_id: str,
    context: ExecContext,
    entities: Optional[list[Any]] = None,
) -> CedarAuthorizationRequest:
    """Build a Cedar authorization request for an ``exec`` action."""

    return CedarAuthorizationRequest(
        principal=entity_uid("Principal", principal.agent_id),
        action=action_uid("exec"),
        resource=entity_uid("Executable", executable_id),
        context=_context_to_dict(context),
        entities=entities or [],
    )


def build_open_request(
    principal: AgentPrincipal,
    file_id: str,
    context: OpenContext,
    entities: Optional[list[Any]] = None,
) -> CedarAuthorizationRequest:
    """Build a Cedar authorization request for an ``open`` action."""

    return CedarAuthorizationRequest(
        principal=entity_uid("Principal", principal.agent_id),
        action=action_uid("open"),
        resource=entity_uid("File", file_id),
        context=_context_to_dict(context),
        entities=entities or [],
    )


def build_connect_request(
    principal: AgentPrincipal,
    endpoint_id: str,
    context: ConnectContext,
    entities: Optional[list[Any]] = None,
) -> CedarAuthorizationRequest:
    """Build a Cedar authorization request for a ``connect`` action."""

    return CedarAuthorizationRequest(
        principal=entity_uid("Principal", principal.agent_id),
        action=action_uid("connect"),
        resource=entity_uid("Endpoint", endpoint_id),
        context=_context_to_dict(context),
        entities=entities or [],
    )


def build_request_tool_request(
    principal: AgentPrincipal,
    tool_id: str,
    context: RequestToolContext,
    entities: Optional[list[Any]] = None,
) -> CedarAuthorizationRequest:
    """Build a Cedar authorization request for a ``request_tool`` action."""

    return CedarAuthorizationRequest(
        principal=entity_uid("Principal", principal.agent_id),
        action=action_uid("request_tool"),
        resource=entity_uid("Tool", tool_id),
        context=_context_to_dict(context),
        entities=entities or [],
    )


__all__ = [
    "SCHEMA_NAMESPACE",
    "ActionVerb",
    "AgentPrincipal",
    "AgentFile",
    "AgentEndpoint",
    "AgentTool",
    "AgentExecutable",
    "ExecContext",
    "OpenContext",
    "ConnectContext",
    "RequestToolContext",
    "CedarAuthorizationRequest",
    "entity_uid",
    "action_uid",
    "build_exec_request",
    "build_open_request",
    "build_connect_request",
    "build_request_tool_request",
]
