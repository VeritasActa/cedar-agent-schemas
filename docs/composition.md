# Composition

How downstream systems compose this schema with their own tool-specific extensions.

## Pattern

This library defines four canonical action verbs. Downstream systems typically want to authorize more specific actions (e.g., a particular MCP tool, a specific API endpoint, a domain-specific file type).

The recommended pattern: downstream systems define their own namespace and map their specific actions onto the canonical four.

## Example: AGT-governed agent

Microsoft Agent Governance Toolkit (AGT) defines its own governance schema. An AGT policy authoring team might write:

```cedar
// AGT-specific namespace, extends Agent:: with AGT conventions
namespace AgtGovernance {
  // AGT's GitHub MCP integration authorizes specific actions
  action "CreatePullRequest" in [Agent::Action::"request_tool"];
  action "MergePullRequest" in [Agent::Action::"request_tool"];
}

// Policy uses the canonical verb with AGT's refinement
permit (
    principal is Agent::Principal,
    action == AgtGovernance::Action::"CreatePullRequest",
    resource is Agent::Tool
)
when {
    resource.name == "mcp-github" &&
    principal.trust_score > 0.7
};
```

The underlying operation maps to `Agent::Action::"request_tool"`; AGT's specific tool-call action is a refinement. An auditor verifying the policy chain across AGT and another system sees the same canonical verb at the top level.

## Example: protect-mcp with tool-specific attributes

protect-mcp emits signed receipts that reference these canonical verbs:

```json
{
  "decision": "allow",
  "principal": "Agent::Principal::\"claude-code-3a2f\"",
  "action": "Agent::Action::\"request_tool\"",
  "resource": "Agent::Tool::\"Bash\"",
  "context": {
    "args_hash": "sha256:e4d61f7a...",
    "transport": "mcp_stdio"
  }
}
```

The receipt's `action` field uses the canonical namespace. An auditor running `npx @veritasacta/verify` validates against any receipt from any Cedar-based system using the same vocabulary.

## Example: physical-attestation sensor

A cold-chain sensor evaluating whether to permit a "release shipment" action uses a physical `open` with additional context attributes:

```cedar
// Physical attestation extension
namespace PhysicalAttestation {
  action "ReleaseShipment" in [Agent::Action::"open"];
}

permit (
    principal is Agent::Principal,
    action == PhysicalAttestation::Action::"ReleaseShipment",
    resource is Agent::File  // a shipment manifest
)
when {
    context.mode == "read" &&
    context.current_temperature_c <= 8.0 &&
    context.shock_g <= 2.0 &&
    context.chain_verified == true
};
```

The sensor's policy engine uses canonical `Agent::Action::"open"` with extended context attributes. A verifier does not need to know the physical-attestation extension; the canonical verb and the signed context are sufficient.

## Rules of thumb

1. **Do not add new verbs at the canonical `Agent::*` level for a single system's needs.** Extend in your own namespace.
2. **Map your actions to the canonical four.** Each downstream action should be expressible as one of `exec`, `open`, `connect`, `request_tool`.
3. **Extend context attributes additively.** Your downstream can require more context attributes than the canonical schema specifies; consumers that do not know about those attributes simply ignore them.
4. **Do not redefine canonical entity types.** Your extensions reference `Agent::File`, `Agent::Tool`, etc., rather than creating parallel `YourSystem::File`, etc. types.

Following this pattern keeps policies portable across systems and lets verifiers and auditors work against a stable canonical form.
