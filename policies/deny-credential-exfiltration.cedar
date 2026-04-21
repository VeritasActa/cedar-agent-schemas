// deny-credential-exfiltration: block agent read access to common
// credential paths regardless of trust tier.
//
// Covers SSH keys, cloud provider credentials, Kubernetes kubeconfig,
// shell histories, and environment files. These paths commonly contain
// secrets that an agent has no legitimate reason to read even under
// workspace policies.
//
// This is an OWASP OAT-05 (Tool Misuse) guard applied at the filesystem
// boundary; composes with runtime sandbox enforcement (nono, OpenShell,
// Hyperlight) as a defense-in-depth policy.

forbid (
    principal,
    action == Agent::Action::"open",
    resource is Agent::File
)
when {
    resource.path like "*/.ssh/*" ||
    resource.path like "*/.aws/credentials" ||
    resource.path like "*/.aws/config" ||
    resource.path like "*/.kube/config" ||
    resource.path like "*/.bash_history" ||
    resource.path like "*/.zsh_history" ||
    resource.path like "*/.env" ||
    resource.path like "*/id_rsa*" ||
    resource.path like "*/id_ed25519*"
};
