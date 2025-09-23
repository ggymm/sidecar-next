use gpui::*;
use crate::MainView;

pub struct RandomPage;

impl RandomPage {
    pub fn build(_window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|_| RandomPage))
    }
}

impl Render for RandomPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
