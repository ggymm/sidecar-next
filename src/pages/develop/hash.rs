use gpui::*;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::StyledExt;

use crate::MainView;
use crate::CARD_BG;
use crate::CARD_GAP;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;

const BUTTON_BG: u32 = 0x0078D4;

pub struct HashPage {
    input_content: Entity<InputState>,
    output_result: Entity<InputState>,
    input_type: SharedString,
}

impl HashPage {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input_content = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("输入内容"));
            let output_result = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("输出结果"));

            Self {
                input_content,
                output_result,
                input_type: "文本类型".into(),
            }
        }))
    }
}

impl Render for HashPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        // Input type selection card
                        div()
                            .h(px(80.))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("输入类型"))
                            .child(
                                div()
                                    .w(px(260.))
                                    .h(px(34.))
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_md()
                                    .paddings(input_padding)
                                    .flex()
                                    .items_center()
                                    .child(div().text_sm().text_color(white()).child(self.input_type.clone())),
                            ),
                    )
                    .child(
                        // Input content card
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
                                    .child(div().text_sm().text_color(white()).child("输入内容"))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
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
                                            )
                                            .child(
                                                div()
                                                    .w(px(120.))
                                                    .h(px(34.))
                                                    .bg(button_bg)
                                                    .rounded_md()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(div().text_sm().text_color(white()).child("计算 Hash")),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_md()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.input_content)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .h_full(),
                                    ),
                            ),
                    )
                    .child(
                        // Output result card
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .gap(px(CARD_GAP))
                            .child(div().text_sm().text_color(white()).child("输出结果"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_md()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.output_result)
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
