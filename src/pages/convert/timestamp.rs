use gpui::*;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::scroll::ScrollbarAxis;
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

pub struct TimestampPage {
    timezone_input: Entity<InputState>,
    timestamp_input: Entity<InputState>,
}

impl TimestampPage {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let timezone_input = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("时区...")
                    .default_value("(UTC+08:00) 中国标准时间")
            });
            
            let timestamp_input = cx.new(|cx| InputState::new(window, cx).placeholder("输入时间戳..."));

            Self {
                timezone_input,
                timestamp_input,
            }
        }))
    }
}

impl Render for TimestampPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);
        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .v_flex()
            // 页面内不再显示标题
            .child(
                div()
                    .v_flex()
                    .gap(px(PAGE_GAP))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("时区"))
                            .child(
                                div()
                                    .w(px(480.))
                                    .h(px(36.))
                                    .overflow_hidden()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_lg()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.timezone_input)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .h_full(),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("时间戳"))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_5()
                                    .child(
                                        div()
                                            .w(px(240.))
                                            .h(px(36.))
                                            .overflow_hidden()
                                            .bg(input_bg)
                                            .border_1()
                                            .border_color(rgb(INPUT_BORDER))
                                            .rounded_lg()
                                            .paddings(input_padding)
                                            .child(
                                                TextInput::new(&self.timestamp_input)
                                                    .appearance(false)
                                                    .focus_bordered(false)
                                                    .text_color(white()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .bg(rgb(0x0078D4))
                                            .rounded_md()
                                            .text_sm()
                                            .text_color(white())
                                            .cursor_pointer()
                                            .hover(|style| style.bg(rgb(0x106EBE)))
                                            .child("当前时间"),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .gap(px(PAGE_GAP))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("Common"))
                            .child(div().text_sm().text_color(rgb(0xA0A0A0)).child("2024-01-01 00:00:00")),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("ISO 8601"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xA0A0A0))
                                    .child("2024-01-01T00:00:00+08:00"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(card_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(div().text_sm().text_color(white()).child("RFC 7231"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xA0A0A0))
                                    .child("Mon, 01 Jan 2024 00:00:00 GMT"),
                            ),
                    ),
            )
    }
}
