# Versioning

This library follows [SemVer](https://semver.org/) with additive-only rules between minor versions.

## Policy

- **Patch (0.1.x)**: no schema changes, docs and bindings only.
- **Minor (0.X.0)**: additive-only changes. New optional fields, new optional entity attributes, new action verbs may be added. Existing policies continue to work unchanged against the updated schema.
- **Major (X.0.0)**: breaking changes. Required-field additions, renamed verbs, removed entities, changed attribute types. Consumers pin to a major version and migrate deliberately.

## Schema URI

The raw schema file is versioned by git tag. Consumers pin to either a major version or a specific commit.

```
# Latest within major version 0 (tracks 0.x additive updates)
https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/v0/schemas/agent-actions.cedarschema.json

# Specific commit (reproducible)
https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/<commit-sha>/schemas/agent-actions.cedarschema.json

# Specific release tag
https://raw.githubusercontent.com/VeritasActa/cedar-agent-schemas/v0.1.0/schemas/agent-actions.cedarschema.json
```

## Backward-compat posture

Additive-only within a major version means:

- A policy that evaluates against `agent-actions.cedarschema.json` v0.1 continues to evaluate against v0.2, v0.3, etc.
- A Cedar request constructed against v0.1 remains valid against v0.2.
- A new optional field in v0.2 that a v0.1 policy does not reference has no effect on the policy's behavior.

Mirrors the posture IETF drafts take for protocol evolution: consumers can safely move forward within a major version without rewriting policies.

## Deprecation

A field is deprecated (but not removed) across at least one minor version before any major-version removal. Deprecated fields are documented in the schema changelog and in the field's JSON comment (`$comment`).
