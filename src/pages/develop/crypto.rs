use gpui::*;
use gpui_component::StyledExt;

use crate::MainView;
use crate::PAGE_PADDING;

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
        let page_padding = Edges::all(px(PAGE_PADDING));

        div().w_full().paddings(page_padding).v_flex().child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_xl().text_color(rgb(0x808080)).child("功能开发中...")),
        )
    }
}
