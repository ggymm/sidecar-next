use crate::MainView;
use gpui::*;

pub struct DnsPage;

impl DnsPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| DnsPage))
    }
}

impl Render for DnsPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().w_full()
    }
}
