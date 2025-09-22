use gpui::*;

pub struct DnsPage;

impl Render for DnsPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}

