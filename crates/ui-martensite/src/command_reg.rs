//! Shell command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "shell.switch_app", label: "Switch Active Workspace" },
];
