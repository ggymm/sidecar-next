use gpui::*;
use gpui_component::StyledExt;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;

pub const INPUT_BG: u32 = 0x242424;
pub const INPUT_BORDER: u32 = 0x404040;
pub const INPUT_PADDING: f32 = 4.0;

fn input(state: &Entity<InputState>) -> TextInput {
    TextInput::new(state)
        .appearance(false)
        .focus_bordered(false)
        .text_color(white())
}

pub fn text(
    state: &Entity<InputState>,
    config: impl FnOnce(TextInput) -> TextInput,
) -> Div {
    div()
        .flex_1()
        .bg(rgb(INPUT_BG))
        .border_1()
        .border_color(rgb(INPUT_BORDER))
        .rounded_lg()
        .paddings(Edges::all(px(INPUT_PADDING)))
        .child(config(input(state)))
}
