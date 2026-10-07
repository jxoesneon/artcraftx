//! Sovereign retained-mode suite shell for ArtCraft-X built on Martensite.

pub mod command_reg;
pub mod menus;
pub mod shell;
pub mod theme;

pub struct ArtcraftXApp {
    pub shell: shell::SuiteShellState,
}

impl ArtcraftXApp {
    pub fn new() -> Self {
        Self {
            shell: shell::SuiteShellState::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_init() {
        let app = ArtcraftXApp::new();
        assert_eq!(app.shell.active_app, shell::AppModule::Photocraft);
    }
}
