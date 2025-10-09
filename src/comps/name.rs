use gpui::*;

pub fn name(title: impl IntoElement) -> Div {
    div().text_sm().text_color(white()).child(title)
}
