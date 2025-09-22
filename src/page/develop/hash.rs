use gpui::*;

pub struct HashPage;

impl Render for HashPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}

