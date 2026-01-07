use gpui::*;

use crate::{MainView, comps::page};

pub struct WindowsPage;

impl WindowsPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| WindowsPage))
    }
}

impl Render for WindowsPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
    }
}
