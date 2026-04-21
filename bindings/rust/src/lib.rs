//! Canonical Cedar agent action verbs with typed helpers.
//!
//! Rust equivalents of the TypeScript and Python builders in the sibling
//! packages. The underlying schema file lives at
//! `schemas/agent-actions.cedarschema.json` in the repository root.
//!
//! These helpers produce Cedar authorization request values that any
//! Cedar evaluator (cedar-policy, cedar-wasm, or a remote evaluator) can
//! consume. This crate does not itself evaluate policies; it is a
//! typed-builder layer on top of the canonical schema.

use serde::{Deserialize, Serialize};

/// The canonical schema namespace for agent action verbs.
pub const SCHEMA_NAMESPACE: &str = "Agent";

/// The four canonical AI-agent action verbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionVerb {
    Exec,
    Open,
    Connect,
    RequestTool,
}

impl ActionVerb {
    /// The string form used inside Cedar action UIDs.
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionVerb::Exec => "exec",
            ActionVerb::Open => "open",
            ActionVerb::Connect => "connect",
            ActionVerb::RequestTool => "request_tool",
        }
    }
}

/// Canonical `Agent::Principal` attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPrincipal {
    pub agent_id: String,
    /// Decimal as a string for Cedar precision (e.g. "0.85").
    pub trust_score: String,
    pub ring: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// Canonical `Agent::File` attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFile {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_uid: Option<i64>,
}

/// Canonical `Agent::Endpoint` attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEndpoint {
    pub host: String,
    pub port: i64,
    pub protocol: String,
}

/// Canonical `Agent::Tool` attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// Canonical `Agent::Executable` attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExecutable {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
}

/// Context for `Agent::Action::"exec"` requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecContext {
    pub command: String,
    pub argv: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

/// Context for `Agent::Action::"open"` requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenContext {
    pub mode: String, // "read" | "write" | "append" | etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

/// Context for `Agent::Action::"connect"` requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectContext {
    pub tls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Context for `Agent::Action::"request_tool"` requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestToolContext {
    pub args_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
}

/// Canonical Cedar authorization request shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CedarAuthorizationRequest {
    pub principal: String,
    pub action: String,
    pub resource: String,
    pub context: serde_json::Value,
    pub entities: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
}

/// Build a fully-qualified Cedar entity UID string.
pub fn entity_uid(type_: &str, id: &str) -> String {
    format!("{SCHEMA_NAMESPACE}::{type_}::\"{id}\"")
}

/// Build a fully-qualified Cedar action UID string.
pub fn action_uid(verb: ActionVerb) -> String {
    format!("{SCHEMA_NAMESPACE}::Action::\"{}\"", verb.as_str())
}

/// Build a Cedar authorization request for an `exec` action.
pub fn build_exec_request(
    principal: &AgentPrincipal,
    executable_id: &str,
    context: &ExecContext,
    entities: Option<Vec<serde_json::Value>>,
) -> CedarAuthorizationRequest {
    CedarAuthorizationRequest {
        principal: entity_uid("Principal", &principal.agent_id),
        action: action_uid(ActionVerb::Exec),
        resource: entity_uid("Executable", executable_id),
        context: serde_json::to_value(context).unwrap_or(serde_json::Value::Null),
        entities: entities.unwrap_or_default(),
        schema: None,
    }
}

/// Build a Cedar authorization request for an `open` action.
pub fn build_open_request(
    principal: &AgentPrincipal,
    file_id: &str,
    context: &OpenContext,
    entities: Option<Vec<serde_json::Value>>,
) -> CedarAuthorizationRequest {
    CedarAuthorizationRequest {
        principal: entity_uid("Principal", &principal.agent_id),
        action: action_uid(ActionVerb::Open),
        resource: entity_uid("File", file_id),
        context: serde_json::to_value(context).unwrap_or(serde_json::Value::Null),
        entities: entities.unwrap_or_default(),
        schema: None,
    }
}

/// Build a Cedar authorization request for a `connect` action.
pub fn build_connect_request(
    principal: &AgentPrincipal,
    endpoint_id: &str,
    context: &ConnectContext,
    entities: Option<Vec<serde_json::Value>>,
) -> CedarAuthorizationRequest {
    CedarAuthorizationRequest {
        principal: entity_uid("Principal", &principal.agent_id),
        action: action_uid(ActionVerb::Connect),
        resource: entity_uid("Endpoint", endpoint_id),
        context: serde_json::to_value(context).unwrap_or(serde_json::Value::Null),
        entities: entities.unwrap_or_default(),
        schema: None,
    }
}

/// Build a Cedar authorization request for a `request_tool` action.
pub fn build_request_tool_request(
    principal: &AgentPrincipal,
    tool_id: &str,
    context: &RequestToolContext,
    entities: Option<Vec<serde_json::Value>>,
) -> CedarAuthorizationRequest {
    CedarAuthorizationRequest {
        principal: entity_uid("Principal", &principal.agent_id),
        action: action_uid(ActionVerb::RequestTool),
        resource: entity_uid("Tool", tool_id),
        context: serde_json::to_value(context).unwrap_or(serde_json::Value::Null),
        entities: entities.unwrap_or_default(),
        schema: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_uid_formats() {
        assert_eq!(
            entity_uid("Principal", "alice"),
            r#"Agent::Principal::"alice""#
        );
        assert_eq!(entity_uid("Tool", "Bash"), r#"Agent::Tool::"Bash""#);
    }

    #[test]
    fn action_uid_for_each_verb() {
        assert_eq!(action_uid(ActionVerb::Exec), r#"Agent::Action::"exec""#);
        assert_eq!(action_uid(ActionVerb::Open), r#"Agent::Action::"open""#);
        assert_eq!(
            action_uid(ActionVerb::Connect),
            r#"Agent::Action::"connect""#
        );
        assert_eq!(
            action_uid(ActionVerb::RequestTool),
            r#"Agent::Action::"request_tool""#
        );
    }

    #[test]
    fn build_request_tool_request_shape() {
        let principal = AgentPrincipal {
            agent_id: "claude-code-3a2f".into(),
            trust_score: "0.85".into(),
            ring: 2,
            session_id: Some("sess-9af21".into()),
        };
        let ctx = RequestToolContext {
            args_hash: "sha256:e4d61f7a".into(),
            transport: Some("mcp_stdio".into()),
        };
        let req = build_request_tool_request(&principal, "Bash", &ctx, None);

        assert_eq!(req.principal, r#"Agent::Principal::"claude-code-3a2f""#);
        assert_eq!(req.action, r#"Agent::Action::"request_tool""#);
        assert_eq!(req.resource, r#"Agent::Tool::"Bash""#);
        assert!(req.entities.is_empty());
    }
}
