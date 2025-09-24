use gpui::*;
use gpui::prelude::FluentBuilder;
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
use crate::plugins::hash::{calculate_file_hash, calculate_text_hash};

const BUTTON_BG: u32 = 0x0078D4;

#[derive(Clone, Copy, PartialEq)]
enum InputType {
    Text,
    File,
}

pub struct HashPage {
    input_type: InputType,
    input_content: Entity<InputState>,
    output_result: Entity<InputState>,
    dropdown_open: bool,
}

impl HashPage {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input_content = cx.new(|cx| InputState::new(window, cx).multi_line());
            let output_result = cx.new(|cx| InputState::new(window, cx).multi_line());

            Self {
                input_type: InputType::Text,
                input_content,
                output_result,
                dropdown_open: false,
            }
        }))
    }

    fn toggle_dropdown(&mut self, cx: &mut Context<Self>) {
        self.dropdown_open = !self.dropdown_open;
        cx.notify();
    }

    fn select_input_type(&mut self, input_type: InputType, window: &mut Window, cx: &mut Context<Self>) {
        if self.input_type != input_type {
            self.input_type = input_type;
            // 清空输入输出框
            self.input_content.update(cx, |state, cx2| {
                state.set_value("".to_string(), window, cx2);
            });
            self.output_result.update(cx, |state, cx2| {
                state.set_value("".to_string(), window, cx2);
            });
        }
        self.dropdown_open = false;
        cx.notify();
    }

    fn browse_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input_content.update(cx, |state, cx2| {
            state.set_value("/path/to/file".to_string(), window, cx2);
        });
        cx.notify();
    }

    fn calculate_hash(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input = self.input_content.read(cx).value().to_string();
        if input.trim().is_empty() {
            return;
        }

        match self.input_type {
            InputType::Text => {
                let results = calculate_text_hash(&input);
                self.output_result.update(cx, |state, cx2| {
                    state.set_value(results.to_string(), window, cx2);
                });
            }
            InputType::File => {
                match calculate_file_hash(&input) {
                    Ok(results) => {
                        self.output_result.update(cx, |state, cx2| {
                            state.set_value(results.to_string(), window, cx2);
                        });
                    }
                    Err(_) => {}
                }
            }
        }
        cx.notify();
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
                                    .relative()
                                    .w(px(260.))
                                    .h(px(34.))
                                    .child(
                                        div()
                                            .w_full()
                                            .h_full()
                                            .bg(input_bg)
                                            .border_1()
                                            .border_color(rgb(INPUT_BORDER))
                                            .rounded_md()
                                            .paddings(input_padding)
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .cursor_pointer()
                                            .hover(|style| style.border_color(rgb(0x0078D4)))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(white())
                                                    .child(match self.input_type {
                                                        InputType::Text => "文本类型",
                                                        InputType::File => "文件类型",
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0xA0A0A0))
                                                    .child("▼"),
                                            )
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _ev: &MouseDownEvent, _window, cx| {
                                                    this.toggle_dropdown(cx);
                                                }),
                                            ),
                                    )
                                    .when(self.dropdown_open, |d| {
                                        d.child(
                                            div()
                                                .absolute()
                                                .top(px(36.))
                                                .left(px(0.))
                                                .w_full()
                                                .bg(input_bg)
                                                .border_1()
                                                .border_color(rgb(INPUT_BORDER))
                                                .rounded_md()
                                                .v_flex()
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_2()
                                                        .text_sm()
                                                        .text_color(white())
                                                        .cursor_pointer()
                                                        .hover(|style| style.bg(rgb(0x404040)))
                                                        .child("文本类型")
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(|this, _ev: &MouseDownEvent, window, cx| {
                                                                this.select_input_type(InputType::Text, window, cx);
                                                            }),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .px_3()
                                                        .py_2()
                                                        .text_sm()
                                                        .text_color(white())
                                                        .cursor_pointer()
                                                        .hover(|style| style.bg(rgb(0x404040)))
                                                        .child("文件类型")
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(|this, _ev: &MouseDownEvent, window, cx| {
                                                                this.select_input_type(InputType::File, window, cx);
                                                            }),
                                                        ),
                                                ),
                                        )
                                    }),
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
                                                    .when(self.input_type != InputType::File, |div| {
                                                        div.opacity(0.3)
                                                    })
                                                    .when(self.input_type == InputType::File, |div| {
                                                        div.cursor_pointer()
                                                            .hover(|style| style.bg(rgb(0x106EBE)))
                                                    })
                                                    .child(div().text_sm().text_color(white()).child("选择文件"))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _ev: &MouseDownEvent, window, cx| {
                                                            if this.input_type == InputType::File {
                                                                this.browse_file(window, cx);
                                                            }
                                                        }),
                                                    ),
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
                                                    .cursor_pointer()
                                                    .hover(|style| style.bg(rgb(0x106EBE)))
                                                    .child(div().text_sm().text_color(white()).child("计算 Hash"))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _ev: &MouseDownEvent, window, cx| {
                                                            this.calculate_hash(window, cx);
                                                        }),
                                                    ),
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
                                            .font_family("monospace")
                                            .h_full(),
                                    ),
                            ),
                    ),
            )
    }
}
