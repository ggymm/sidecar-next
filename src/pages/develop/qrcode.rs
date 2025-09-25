use gpui::*;
use gpui_component::StyledExt;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;

use crate::CARD_BG;
use crate::CARD_GAP;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::MainView;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;

const BUTTON_BG: u32 = 0x0078D4;

pub struct QrcodePage {
    parsed_output: Entity<InputState>,
    selected_file: Option<SharedString>,
}

impl QrcodePage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let parsed_output = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("解析结果"));

            Self {
                parsed_output,
                selected_file: None,
            }
        }))
    }
}

impl Render for QrcodePage {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);
        let button_bg = rgb(BUTTON_BG);

        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .key_context("Input")
            .w_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .v_flex()
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .flex_1()
                    .child(
                        // File selection and image display card
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .gap(px(CARD_GAP))
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .child(div().text_sm().text_color(white()).child("文件"))
                                    .child(
                                        div()
                                            .w(px(120.))
                                            .h(px(34.))
                                            .bg(button_bg)
                                            .rounded_md()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(div().text_sm().text_color(white()).child("选择文件")),
                                    ),
                            )
                            .child(div().flex_1().flex().items_center().justify_center().child(
                                if self.selected_file.is_some() {
                                    div().text_sm().text_color(rgb(0x808080)).child("图片预览区域")
                                } else {
                                    div().text_sm().text_color(rgb(0x808080)).child("请选择二维码图片文件")
                                },
                            )),
                    )
                    .child(
                        // Parse result card
                        div()
                            .h(px(240.))
                            .flex()
                            .flex_col()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .gap(px(CARD_GAP))
                            .child(div().text_sm().text_color(white()).child("解析结果"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_md()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.parsed_output)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .h_full(),
                                    ),
                            ),
                    ),
            )
    }
}
