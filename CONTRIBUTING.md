# Contributing

Contributions welcome. This library is designed to be tight in scope; see below for what fits and what does not.

## Guiding principles

1. **Community-neutral**: the schema and policies do not encode any specific governance framework, receipt format, signing scheme, or runtime. If you propose a change that requires a specific vendor choice, it belongs in a downstream system, not here.
2. **Narrow surface**: four action verbs. Adding a fifth requires two independent implementations that need the verb and a demonstrated mapping to an OWASP Agentic Top 10 risk.
3. **Additive only between minor versions**: see [docs/versioning.md](docs/versioning.md).
4. **Policy precedence: discussion first, PR after**: open an issue before opening a PR that adds or changes schema entities, actions, or required fields.

## What fits

- New optional attributes on existing entities, with a concrete use case
- Additional reference policies under `policies/` that operate on the existing verbs
- Documentation improvements
- Bindings (TypeScript, Python, Go, Rust) that produce Cedar requests from typed inputs against this schema
- OWASP mapping refinements
- Cross-implementation examples showing how the same verb is used across downstream frameworks

## What does not fit

- Vendor-specific extensions (framework-specific actions, receipt-format opinions, sandbox-specific fields)
- Schemas for non-agent Cedar use cases
- Runtime code, evaluators, or enforcement surfaces (those belong in consumer projects)
- Domain-specific namespaces outside `Agent::*` (consumer projects can define their own namespaces that extend this one)

## How to propose a new verb

A new action verb is added conservatively. Open an issue with:

1. The proposed verb name and resource type
2. Two independent implementations (production or staged) that need the verb
3. The OWASP Agentic Top 10 risk the verb addresses
4. A worked example policy using the verb
5. Why the existing four verbs are insufficient (cannot be expressed as `exec`, `open`, `connect`, or `request_tool`)

Expect discussion. If the proposal matches the shape above, we accept with a schema bump to the next minor version.

## How to propose a new optional attribute

Simpler than a new verb. Open an issue or PR directly with:

1. The attribute and its Cedar type
2. The entity it belongs on
3. One concrete use case

Optional attributes are additive and ship without a major version bump.

## PR checklist

- [ ] Schema file validates: `cedar-policy-cli validate --schema schemas/agent-actions.cedarschema.json`
- [ ] Any new policies validate against the schema
- [ ] Documentation updated (README, versioning, OWASP mapping if relevant)
- [ ] Changelog entry added (CHANGELOG.md)
- [ ] Backward-compat confirmed (existing policies continue to evaluate)

## Code of Conduct

Contributors are expected to follow the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). Disagree technically; respect each other personally.

## License

By contributing, you agree that your contributions are licensed under Apache-2.0.

## Attribution

This library was created in response to the scope redirect on [cedar-policy/cedar-for-agents#76](https://github.com/cedar-policy/cedar-for-agents/issues/76). It implements the community-venue pattern established by Cedar RFCs [#58](https://github.com/cedar-policy/rfcs/pull/58) and [#69](https://github.com/cedar-policy/rfcs/pull/69).
