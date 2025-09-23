use base64::engine::general_purpose;
use base64::Engine;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::StyledExt;

use crate::pages::utils::strip_str;
use crate::MainView;
use crate::CARD_BG;
use crate::CARD_GAP;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;

pub struct Base64Page {
    input: Entity<InputState>,
    output: Entity<InputState>,
    last_input: SharedString,
    last_output: SharedString,
    updating: bool,
}

impl Base64Page {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line());
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());

            Self {
                input,
                output,
                last_input: SharedString::default(),
                last_output: SharedString::default(),
                updating: false,
            }
        }))
    }
}

impl Render for Base64Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.updating {
            let in_val = self.input.read(cx).value();
            let out_val = self.output.read(cx).value();

            enum Src {
                In,
                Out,
            }

            let src = if in_val != self.last_input {
                Some(Src::In)
            } else if out_val != self.last_output {
                Some(Src::Out)
            } else {
                None
            };

            if let Some(src) = src {
                self.updating = true;
                match src {
                    Src::In => {
                        let enc = general_purpose::STANDARD.encode(in_val.as_str());
                        self.output.update(cx, |state, cx2| state.set_value(enc, window, cx2));
                        self.last_input = in_val;
                        self.last_output = self.output.read(cx).value();
                    }
                    Src::Out => {
                        let cleaned = strip_str(out_val.as_str());
                        if let Ok(bytes) = general_purpose::STANDARD.decode(cleaned.as_bytes()) {
                            let s = String::from_utf8(bytes)
                                .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).to_string());
                            self.input.update(cx, |state, cx2| state.set_value(s, window, cx2));
                        }
                        self.last_input = self.input.read(cx).value();
                        self.last_output = out_val;
                    }
                }
                self.updating = false;
            }
        }

        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);

        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .key_context("Input")
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .v_flex()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .child("Base64 编解码"),
            )
            .child(
                div()
                    .v_flex()
                    .gap(px(PAGE_GAP))
                    .flex_1()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .gap(px(CARD_GAP))
                            .child(div().text_sm().text_color(white()).child("原始内容"))
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
                            .child(div().text_sm().text_color(white()).child("编码内容"))
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
                    ),
            )
    }
}
