use gpui::*;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::StyledExt;

use crate::MainView;
use crate::CARD_BG;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_PADDING;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;

const BUTTON_BG: u32 = 0x0078D4;

pub struct RandomPage {
    mac_input: Entity<InputState>,
    uuid_input: Entity<InputState>,
    phone_input: Entity<InputState>,
}

impl RandomPage {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let mac_input = cx.new(|cx| InputState::new(window, cx).placeholder("MAC地址").default_value("\u{200B}"));
            let uuid_input = cx.new(|cx| InputState::new(window, cx).placeholder("UUID.v4").default_value("\u{200B}"));
            let phone_input = cx.new(|cx| InputState::new(window, cx).placeholder("手机号码").default_value("\u{200B}"));

            Self {
                mac_input,
                uuid_input,
                phone_input,
            }
        }))
    }

    fn build_random_card(&self, label: &str, input: &Entity<InputState>) -> impl IntoElement {
        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);
        let button_bg = rgb(BUTTON_BG);
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .h(px(80.))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .bg(card_bg)
            .rounded_lg()
            .paddings(card_padding)
            .child(div().text_sm().text_color(white()).child(""))
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div().w(px(480.)).child(TextInput::new(input)),
                    )
                    .child(
                        div()
                            .w(px(100.))
                            .h(px(34.))
                            .bg(button_bg)
                            .rounded_md()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(white())
                                    .child("生成")
                            ),
                    )
                    .child(
                        div()
                            .w(px(100.))
                            .h(px(34.))
                            .bg(button_bg)
                            .rounded_md()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(white())
                                    .child("复制")
                            ),
                    ),
            )
    }
}

impl Render for RandomPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let page_padding = Edges::all(px(PAGE_PADDING));

        div()
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .v_flex()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .child("随机数据"),
            )
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .child(self.build_random_card("MAC地址", &self.mac_input))
                    .child(self.build_random_card("UUID.v4", &self.uuid_input))
                    .child(self.build_random_card("手机号码", &self.phone_input)),
            )
    }
}
