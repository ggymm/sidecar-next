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

pub struct CertPage {
    input: Entity<InputState>,
    output: Entity<InputState>,
    last_input: SharedString,
    updating: bool,
}

impl CertPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("输入证书内容"));
            let output = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("解析结果"));

            Self {
                input,
                output,
                last_input: SharedString::default(),
                updating: false,
            }
        }))
    }
}

impl Render for CertPage {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if !self.updating {
            let in_val = self.input.read(cx).value();

            self.updating = true;
            if in_val != self.last_input {
                let out_val = if in_val.is_empty() {
                    String::new()
                } else {
                    String::new()
                };

                self.output.update(cx, |state, cx2| {
                    state.set_value(out_val, window, cx2);
                });
                self.last_input = in_val;
            }
            self.updating = false;
        }

        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);

        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .key_context("Input")
            .v_flex()
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .child(
                div()
                    .h(px(240.))
                    .flex()
                    .flex_col()
                    .bg(card_bg)
                    .rounded_lg()
                    .paddings(card_padding)
                    .gap(px(CARD_GAP))
                    .child(div().text_sm().text_color(white()).child("证书"))
                    .child(
                        div()
                            .flex_1()
                            .bg(input_bg)
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .rounded_lg()
                            .paddings(input_padding)
                            .child(
                                TextInput::new(&self.input)
                                    .appearance(false)
                                    .focus_bordered(false)
                                    .text_color(white())
                                    .h_full(),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_1()
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
                            .rounded_lg()
                            .paddings(input_padding)
                            .child(
                                TextInput::new(&self.output)
                                    .appearance(false)
                                    .focus_bordered(false)
                                    .text_color(white())
                                    .h_full(),
                            ),
                    ),
            )
    }
}
