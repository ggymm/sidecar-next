use gpui::*;
use crate::MainView;

pub struct CryptoPage;

impl CryptoPage {
    pub fn build(_window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|_| CryptoPage))
    }
}

impl Render for CryptoPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
