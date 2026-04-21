# cedar-agent-schemas

Canonical community-maintained Cedar schema library for the four universal AI-agent action verbs: `exec`, `open`, `connect`, `request_tool`.

A single schema file that every Cedar-based agent-governance system can import as the shared vocabulary for agent-action policy authoring. Operator policies become portable across AGT, sb-runtime, protect-mcp, Signet, bindu, nono, APS, and future implementers.

## Status

v0.1.0 alpha. Schema shape is settling; additive changes likely. Versioned by SemVer with additive-only minor revisions (see [docs/versioning.md](docs/versioning.md)).

## What this is

An external, community-maintained Cedar schema library. Not part of `cedar-policy/*`. Created in response to the scope redirect on [cedar-policy/cedar-for-agents#76](https://github.com/cedar-policy/cedar-for-agents/issues/76), which established that domain-specific schemas are distributed outside Cedar core. Follows the design vocabulary proposed in Cedar [RFC #58](https://github.com/cedar-policy/rfcs/pull/58) (Cedar standard library) and [RFC #69](https://github.com/cedar-policy/rfcs/pull/69) (Schema libraries).

## What this is not

- Not an alternative to Cedar. All schemas here parse against unmodified `cedar-policy/cedar`.
- Not a runtime. No policy evaluation happens here; downstream systems bring their own Cedar engine.
- Not opinionated about governance framework. Any Cedar consumer can import these schemas; what you do with them is your system's concern.

## Install

Drop the schema file into your Cedar workspace:

```bash
curl -O https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/main/schemas/agent-actions.cedarschema.json
```

Or reference by commit hash for reproducibility:

```bash
curl -O https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/<commit-sha>/schemas/agent-actions.cedarschema.json
```

TypeScript helpers (optional):

```bash
npm install @veritasacta/cedar-agent-schemas
```

Python helpers planned for v0.2.

## The four action verbs

| Verb | Resource | What it authorizes |
|---|---|---|
| `exec` | `Agent::Executable` | Shell command / process spawn |
| `open` | `Agent::File` | File read / write |
| `connect` | `Agent::Endpoint` | Network socket / HTTP request |
| `request_tool` | `Agent::Tool` | MCP or agent-SDK tool invocation |

Every agent-governance system maps its own internal actions to these four primitives. Cross-ecosystem policies become portable: an operator writes `permit action == Agent::Action::"open"` once, and any conformant governance system enforces it.

## Example

```cedar
// Allow read-only access inside /workspace for any agent at ring >= 2
permit (
    principal is Agent::Principal,
    action == Agent::Action::"open",
    resource is Agent::File
) when {
    principal.ring >= 2 &&
    resource.path like "/workspace/*" &&
    context.mode == "read"
};

// Deny network to cloud metadata regardless of trust
forbid (
    principal,
    action == Agent::Action::"connect",
    resource is Agent::Endpoint
) when {
    resource.host == "169.254.169.254" ||
    resource.host == "metadata.google.internal"
};
```

More examples in [policies/](policies/).

## Design principles

1. **Narrow surface.** Four action verbs. Not 40. Downstream systems layer their own tool-specific actions on top.
2. **Context-bearing attributes.** Action-specific attributes (`command`, `argv`, `mode`, `url`) live in Cedar `context`, not as action attributes. Matches Cedar's recommended shape.
3. **Additive versioning.** Minor revisions (0.1 → 0.2) add optional fields only. Breaking changes require a major bump and a new schema URI.
4. **No vendor opinion.** Schema does not encode any specific governance framework, receipt format, signing scheme, or runtime. It is a naming layer only.
5. **OWASP-mapped.** Each verb maps to an OWASP Agentic Top 10 risk. See [docs/owasp-mapping.md](docs/owasp-mapping.md).

## Implementations

Systems that import this schema (as the canonical base, then extend):

- [protect-mcp](https://www.npmjs.com/package/protect-mcp) and [protect-mcp-adk](https://pypi.org/project/protect-mcp-adk/) (Cedar + signed receipts for MCP tool calls)
- [sb-runtime](https://github.com/ScopeBlind/sb-runtime) (Rust runtime for Cedar + receipts)
- [agent-governance-toolkit](https://github.com/microsoft/agent-governance-toolkit) (Microsoft AGT) via the sb-runtime provider shim
- APS ([Agent Passport System](https://github.com/aeoess/APS)) as a verifiable-delegation layer above the schema
- [Signet](https://github.com/prismer-ai/signet) (governance adapter)
- Your system here; PRs welcome

## Versioning

SemVer. Additive-only between minor versions. See [docs/versioning.md](docs/versioning.md).

The schema URI includes the major version:

```
https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/v0/schemas/agent-actions.cedarschema.json
```

Consumers pin to a major version; additive updates within that major version do not break existing policies.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). New action verbs are added conservatively; propose via an issue first with at least two independent implementations that need the verb.

## Prior art

- [Cedar RFC #58: Standard library](https://github.com/cedar-policy/rfcs/pull/58) by @cdisselkoen (Cedar team, March 2024). Proposed bundling common domain schemas into Cedar. Redirected to external distribution.
- [Cedar RFC #69: Schema libraries](https://github.com/cedar-policy/rfcs/pull/69) by @cdisselkoen (June 2024). Mechanism for external schema imports.
- [cedar-policy/cedar-for-agents#76](https://github.com/cedar-policy/cedar-for-agents/issues/76) (April 2026). Confirmed scope: domain schemas live in community repos, not in Cedar core.

This repo implements the community-venue pattern those RFCs established.

## License

Apache-2.0. Same license as Cedar itself.

## Contact

Issues and PRs welcome. For standards-body coordination (IETF, CNCF, OpenSSF), cc @tomjwxf.
