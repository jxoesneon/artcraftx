//! Suite window management state.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppModule {
    Photocraft,
    Filmcraft,
    Vectorcraft,
    Soundcraft,
    Cadcraft,
}

pub struct SuiteShellState {
    pub active_app: AppModule,
}

impl SuiteShellState {
    pub fn new() -> Self {
        Self { active_app: AppModule::Photocraft }
    }

    pub fn switch_app(&mut self, app: AppModule) {
        self.active_app = app;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_switching() {
        let mut s = SuiteShellState::new();
        s.switch_app(AppModule::Filmcraft);
        assert_eq!(s.active_app, AppModule::Filmcraft);
    }
}
