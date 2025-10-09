use gpui::*;
use gpui_component::StyledExt;

pub const PAGE_BG: u32 = 0x282828;
pub const PAGE_GAP: f32 = 20.0;
pub const PAGE_PADDING: f32 = 20.0;

pub fn page() -> Div {
    div()
        .v_flex()
        .w_full()
        .bg(rgb(PAGE_BG))
        .paddings(Edges::all(px(PAGE_PADDING)))
        .gap(px(PAGE_GAP))
}
