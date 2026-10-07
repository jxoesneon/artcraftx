//! Shell theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub shell_bar: Color,
    pub active_tab: Color,
}

impl Theme {
    pub fn shell_theme() -> Self {
        Self {
            shell_bar: Color(16, 18, 22),
            active_tab: Color(0, 229, 255),
        }
    }
}
