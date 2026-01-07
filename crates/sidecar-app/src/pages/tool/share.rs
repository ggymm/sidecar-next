use gpui::*;

use crate::{MainView, comps::page};

pub struct SharePage;

impl SharePage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| SharePage))
    }
}

impl Render for SharePage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page().size_full()
    }
}
