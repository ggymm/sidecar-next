use gpui::*;

pub struct HomePage;

impl Render for HomePage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // 空页面
        div().size_full()
    }
}

