use gpui::*;

use crate::{MainView, comps::page};

pub struct CryptoPage;

impl CryptoPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| CryptoPage))
    }
}

impl Render for CryptoPage {
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
