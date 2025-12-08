use gpui::*;

use crate::{comps::page, MainView};

pub struct CodePage;

impl CodePage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| CodePage))
    }
}

impl Render for CodePage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
    }
}
