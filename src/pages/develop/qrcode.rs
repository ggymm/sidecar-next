use gpui::*;
use crate::MainView;

pub struct QrcodePage;

impl QrcodePage {
    pub fn build(_window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|_| QrcodePage))
    }
}

impl Render for QrcodePage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
