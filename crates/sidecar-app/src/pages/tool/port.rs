use gpui::*;

use crate::{MainView, comps::page};

pub struct PortPage;

impl PortPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| PortPage))
    }
}

impl Render for PortPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
    }
}
