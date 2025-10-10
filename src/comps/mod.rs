use gpui::*;
use gpui_component::input::{InputState, TextInput};

pub fn page() -> Div {
    div()
        .flex()
        .flex_col()
        .p_5()
        .gap_5()
        .min_w_0()
        .min_h_0()
        .bg(rgb(0x282828))
}

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .p_5()
        .gap_3()
        .min_w_0()
        .min_h_0()
        .bg(rgb(0x333333))
        .rounded_lg()
}

pub fn label(title: impl IntoElement) -> Div {
    div().text_sm().text_color(white()).child(title)
}

pub fn input(state: &Entity<InputState>) -> TextInput {
    TextInput::new(state)
        .h_full()
        .appearance(false)
        .focus_bordered(false)
        .text_color(white())
}

pub fn textarea(
    state: &Entity<InputState>,
    config: impl FnOnce(TextInput) -> TextInput,
) -> Div {
    div()
        .flex_1()
        .p_1()
        .bg(rgb(0x242424))
        .border_1()
        .border_color(rgb(0x404040))
        .rounded_lg()
        .child(config(input(state)))
}
