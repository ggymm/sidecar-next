use gpui::*;

pub struct CryptoPage;

impl Render for CryptoPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}

