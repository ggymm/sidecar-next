use gpui::*;
use crate::MainView;

pub struct CertPage;

impl CertPage {
    pub fn build(_window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|_| CertPage))
    }
}

impl Render for CertPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
