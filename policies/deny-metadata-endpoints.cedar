// deny-metadata-endpoints: block agent network access to cloud instance
// metadata endpoints regardless of trust tier.
//
// Covers AWS, GCP, and Azure Instance Metadata Service endpoints. These
// endpoints commonly leak credentials to any process on the host, so a
// forbid rule here protects against credential exfiltration by a
// compromised agent even at high trust.
//
// This is an OWASP OAT-02 (Privilege Escalation) guard.

forbid (
    principal,
    action == Agent::Action::"connect",
    resource is Agent::Endpoint
)
when {
    resource.host == "169.254.169.254" ||
    resource.host == "metadata.google.internal" ||
    resource.host == "metadata.azure.com" ||
    resource.host == "fd00:ec2::254"
};
