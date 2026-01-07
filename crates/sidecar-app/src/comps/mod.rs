use gpui::*;
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonVariants},
    input::{Input, InputState},
};

pub fn page() -> Div {
    div()
        .flex()
        .flex_col()
        .p_4()
        .gap_4()
        .min_w_0()
        .min_h_0()
        .bg(rgb(0x282828))
}

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .p_4()
        .gap_2()
        .min_w_0()
        .min_h_0()
        .bg(rgb(0x333333))
        .rounded_lg()
}

pub fn label(title: impl IntoElement) -> Div {
    div().text_sm().text_color(white()).child(title)
}

pub fn input(state: &Entity<InputState>) -> Input {
    Input::new(state)
        .h_full()
        .appearance(false)
        .focus_bordered(false)
        .text_color(white())
}

pub fn textarea(
    state: &Entity<InputState>,
    config: impl FnOnce(Input) -> Input,
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

pub fn button(
    cx: &App,
    id: impl Into<ElementId>,
) -> Button {
    Button::new(id).h_9().custom(
        ButtonCustomVariant::new(cx)
            .color(rgb(0x3f3f3f).into())
            .hover(rgb(0x444444).into())
            .active(rgb(0x393939).into())
            .border(rgb(0x474747).into())
            .foreground(rgb(0xfffffff).into()),
    )
}
