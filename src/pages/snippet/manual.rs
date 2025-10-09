use gpui::*;

use crate::MainView;
use crate::comps::Page;

pub struct ManualPage;

impl ManualPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| ManualPage))
    }
}

impl Render for ManualPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Page::new().size_full()
    }
}
