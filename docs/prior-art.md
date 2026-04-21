# Prior art and provenance

This library was created in response to the Cedar team's established scope boundary on domain schemas. The following three artifacts set the pattern:

## Cedar RFC #58: Standard library

[cedar-policy/rfcs#58](https://github.com/cedar-policy/rfcs/pull/58), filed March 2024 by @cdisselkoen (Cedar team member).

Proposed bundling common domain schemas (OIDC::User, etc.) into Cedar itself. After community discussion, the consensus that emerged (and was later re-confirmed on #76 below) was that domain-specific schemas should be distributed outside Cedar core. Users provide the schema file alongside their own policies.

Quoted from community member nynymike (March 2025) on the thread:
> "Option A is the best: distribute ordinary schema files with these declarations (in the cedar-policy/cedar repo or elsewhere). Users would have to provide this schema file in addition to the rest of their schema."

## Cedar RFC #69: Schema libraries

[cedar-policy/rfcs#69](https://github.com/cedar-policy/rfcs/pull/69), filed June 2024 by @cdisselkoen, split out from #58.

Proposes the mechanism by which external schema libraries can be imported into Cedar policies. Specifies the `extends` keyword and library namespace conventions.

## cedar-for-agents #76

[cedar-policy/cedar-for-agents#76](https://github.com/cedar-policy/cedar-for-agents/issues/76), closed April 2026 by @victornicolet.

Proposed adding a canonical schema library for AI-agent action verbs directly into cedar-for-agents. Victor's close-out:

> "This seems to be more of a problem for the different agent governance systems to agree upon, rather than a problem for the maintainers of the open-source Cedar policy language. The Cedar team does not maintain specific Cedar schema fragments and focuses on the general-purpose language. We have rejected similar RFCs in the past (see this RFC and this one). We would love for the community to start such a discussion instead."

This repo is the community-venue response Victor asked for.

## What this repo does differently

- Community-maintained, not Cedar-team maintained. No expectation that the Cedar team will review or merge schema changes.
- Narrow scope by design: four action verbs (`exec`, `open`, `connect`, `request_tool`), not a general standard library.
- Versioned independently of Cedar. Consumers pin to major version here; Cedar language version is handled separately.
- Neutral on downstream framework choice. Works with any Cedar-based agent-governance system.

## Coordination with Cedar team

This library does not require Cedar-team approval. However, when the Cedar RFCs #58 / #69 mechanism lands (schema library imports via `extends`), this schema is designed to be imported via that mechanism without change. Design vocabulary in this repo follows the conventions proposed in #58 and #69 where they apply.

## Follow-on references

- [in-toto/attestation#549](https://github.com/in-toto/attestation/pull/549) (proposed Decision Receipt predicate type; references this schema as input format for verifier tooling)
- [microsoft/agent-governance-toolkit](https://github.com/microsoft/agent-governance-toolkit) (AGT; consumers may map their internal actions to this schema)
- [draft-farley-acta-signed-receipts](https://datatracker.ietf.org/doc/draft-farley-acta-signed-receipts/) (IETF draft; receipt format referenced by downstream systems that emit signed decisions against these verbs)
