use gpui::*;

pub struct ManualPage;

impl Render for ManualPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
