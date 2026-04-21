# Policy catalog

Reference policies that downstream systems can import verbatim or use as templates. Each policy operates on the four canonical action verbs in [schemas/agent-actions.cedarschema.json](../schemas/agent-actions.cedarschema.json).

## Baseline policies

| Policy | Verb(s) | OWASP mapping | Purpose |
|---|---|---|---|
| [allow-read-workspace.cedar](allow-read-workspace.cedar) | `open` | OAT-05 (scope) | Permit read-only file access within the workspace for ring-2+ agents |
| [deny-metadata-endpoints.cedar](deny-metadata-endpoints.cedar) | `connect` | OAT-02 (privilege escalation) | Block cloud instance metadata endpoints |
| [deny-credential-exfiltration.cedar](deny-credential-exfiltration.cedar) | `open` | OAT-05 (tool misuse) | Block common credential paths regardless of trust |

## Composition

Policies compose cleanly. A typical deployment layers a permissive allow (e.g., workspace read) with one or more explicit forbids (e.g., credentials, metadata). Cedar's policy evaluator handles the combining; the `forbid` rules take precedence over any `permit`.

Example composition:

```cedar
// From allow-read-workspace.cedar: allow reads in /workspace
permit (
    principal is Agent::Principal,
    action == Agent::Action::"open",
    resource is Agent::File
) when {
    principal.ring >= 2 &&
    resource.path like "/workspace/*" &&
    context.mode == "read"
};

// From deny-credential-exfiltration.cedar: override and deny credentials
forbid (
    principal,
    action == Agent::Action::"open",
    resource is Agent::File
) when {
    resource.path like "*/.ssh/*" ||
    resource.path like "*/.aws/credentials"
    // ... full list in the policy file
};
```

A request to open `/workspace/.ssh/id_rsa` is denied because `forbid` beats `permit` in Cedar semantics.

## Adding policies

Open an issue describing the policy's purpose, the canonical verbs it operates on, and at least one concrete use case with two independent implementations. Discussion first, PR after.
