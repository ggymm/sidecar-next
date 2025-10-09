use gpui::*;
use gpui_component::StyledExt;

pub const CARD_BG: u32 = 0x333333;
pub const CARD_GAP: f32 = 12.0;
pub const CARD_PADDING: f32 = 20.0;

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .min_w_0()
        .min_h_0()
        .bg(rgb(CARD_BG))
        .rounded_lg()
        .paddings(Edges::all(px(CARD_PADDING)))
        .gap(px(CARD_GAP))
}

pub fn card_named(title: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .min_w_0()
        .min_h_0()
        .bg(rgb(CARD_BG))
        .rounded_lg()
        .paddings(Edges::all(px(CARD_PADDING)))
        .gap(px(CARD_GAP))
        .child(div().text_sm().text_color(white()).child(title))
}
