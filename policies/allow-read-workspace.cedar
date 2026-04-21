// allow-read-workspace: permit read-only file access inside the workspace
// for agents at ring 2 or higher.
//
// Intended usage: baseline policy for development / testing agents that
// need to read code, configs, and documentation within a defined workspace
// directory but should not be able to modify files or read outside the
// workspace scope.

permit (
    principal is Agent::Principal,
    action == Agent::Action::"open",
    resource is Agent::File
)
when {
    principal.ring >= 2 &&
    resource.path like "/workspace/*" &&
    context.mode == "read"
};
