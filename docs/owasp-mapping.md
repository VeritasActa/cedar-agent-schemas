# OWASP Agentic Top 10 Mapping

Each of the four action verbs maps to specific risks in the [OWASP Agentic Top 10](https://owasp.org/www-project-agentic-top-10/). A policy authored against these verbs addresses the listed risks at the authorization layer (complementing sandbox isolation, runtime monitoring, and other defense layers).

## Mapping table

| Verb | Primary OWASP risk | Secondary risks | Notes |
|---|---|---|---|
| `exec` | OAT-05 Tool Misuse | OAT-07 Goal Manipulation | Shell command execution is the highest-leverage action for a compromised agent. Policy at the `exec` boundary is the primary authorization gate; sandbox primitives (Landlock, Seatbelt, WASM) provide enforcement defense-in-depth. |
| `open` | OAT-05 Tool Misuse | OAT-02 Privilege Escalation | File reads can exfiltrate secrets (credentials, histories, env files); file writes can modify the trust boundary (CI config, instructions). Path-based allow/forbid policies are the primary control. |
| `connect` | OAT-02 Privilege Escalation | OAT-05 Tool Misuse | Network egress is the channel for credential exfiltration (via metadata endpoints) and command-and-control. Host allowlists combined with cloud-metadata denies are the baseline. |
| `request_tool` | OAT-07 Goal Manipulation | OAT-05 Tool Misuse | MCP and agent-SDK tool calls are how a prompt-injected agent activates downstream actions. Per-tool authorization (via `Tool.name`) with context-based constraints (via `args_hash`) is the primary surface. |

## Why this matters for governance frameworks

Governance frameworks that claim OWASP Agentic Top 10 coverage should have policies authored against specific verbs for specific risks. Using the canonical verb names here (rather than framework-specific names) makes the coverage claims portable across frameworks and audit tools.

## Scope boundary

This library does not attempt to cover all OWASP risks at the policy layer. Some risks (e.g., OAT-09 Insecure Output Handling) are better addressed at the output-validation layer downstream of policy evaluation. The four verbs cover the four highest-leverage authorization boundaries; compose with other mechanisms for risks outside that set.

## References

- [OWASP Agentic Top 10 (draft)](https://owasp.org/www-project-agentic-top-10/)
- [Cedar policy examples](../policies/)
