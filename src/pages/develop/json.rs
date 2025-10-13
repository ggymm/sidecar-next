use gpui::*;

use crate::MainView;
use crate::comps::page;

pub struct JsonPage;

impl JsonPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| JsonPage))
    }
}

impl Render for JsonPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page().child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_xl().text_color(rgb(0x808080)).child("功能开发中...")),
        )
    }
}
