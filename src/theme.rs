use gpui::Hsla;

// Hex color constants for shared UI styling
pub const COLOR_SIDEBAR_BG: u32 = 0x202020;      // Sidebar background
pub const COLOR_CONTENT_BG: u32 = 0x282828;      // App content background

pub const COLOR_PANEL_BG: u32 = 0x2D2D2D;        // Panel outer background
pub const COLOR_PANEL_INNER_BG: u32 = 0x1E1E1E;  // Panel inner background
pub const COLOR_PANEL_BORDER: u32 = 0x3C3C3C;    // Panel border
pub const COLOR_PANEL_INNER_BORDER: u32 = 0x404040; // Inner border

#[inline]
pub fn hsla_from_rgb(hex: u32) -> Hsla { gpui::rgb(hex).into() }

