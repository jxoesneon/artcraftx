//! Suite shell integration tests.

use artcraftx_ui_martensite::{ArtcraftXApp, shell::AppModule};

#[test]
fn test_shell_workflow() {
    let mut app = ArtcraftXApp::new();
    assert_eq!(app.shell.active_app, AppModule::Photocraft);
    app.shell.switch_app(AppModule::Cadcraft);
    assert_eq!(app.shell.active_app, AppModule::Cadcraft);
}
