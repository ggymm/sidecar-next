use gpui::*;
use gpui_component::StyledExt;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;

use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;

pub fn text_area(state: &Entity<InputState>) -> Div {
    div()
        .flex_1()
        .bg(rgb(INPUT_BG))
        .border_1()
        .border_color(rgb(INPUT_BORDER))
        .rounded_lg()
        .paddings(Edges::all(px(INPUT_PADDING)))
        .child(
            TextInput::new(state)
                .appearance(false)
                .focus_bordered(false)
                .text_color(white())
                .h_full(),
        )
}
