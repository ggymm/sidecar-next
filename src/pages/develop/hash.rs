use gpui::*;
use crate::MainView;

pub struct HashPage;

impl HashPage {
    pub fn build(_window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|_| HashPage))
    }
}

impl Render for HashPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
