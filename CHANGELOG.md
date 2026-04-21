# Changelog

All notable changes follow [SemVer](https://semver.org/) and Keep-a-Changelog conventions.

## [0.1.0] - 2026-04-21

Initial release.

### Added

- Schema `agent-actions.cedarschema.json` with four canonical action verbs
  (`exec`, `open`, `connect`, `request_tool`) and five entity types
  (`Principal`, `File`, `Endpoint`, `Tool`, `Executable`)
- Three reference policies under `policies/`:
  - `allow-read-workspace.cedar`
  - `deny-metadata-endpoints.cedar`
  - `deny-credential-exfiltration.cedar`
- TypeScript bindings (`@veritasacta/cedar-agent-schemas`) with typed request
  builders for all four verbs
- Docs: versioning policy, OWASP Agentic Top 10 mapping, composition
  guide, prior art
- CI: schema validation, policy validation, TypeScript build

### Context

Created in response to the scope redirect on
[cedar-policy/cedar-for-agents#76](https://github.com/cedar-policy/cedar-for-agents/issues/76),
following the community-venue pattern established by Cedar RFCs
[#58](https://github.com/cedar-policy/rfcs/pull/58) and
[#69](https://github.com/cedar-policy/rfcs/pull/69).
