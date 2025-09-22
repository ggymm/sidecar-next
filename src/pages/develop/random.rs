use gpui::*;

pub struct RandomPage;

impl Render for RandomPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}

