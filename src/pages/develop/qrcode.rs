use gpui::*;

pub struct QrcodePage;

impl Render for QrcodePage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
