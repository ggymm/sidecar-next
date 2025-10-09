use gpui::*;
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::radio::RadioGroup;
use gpui_component::{Disableable, StyledExt};

use crate::COMMON_GAP;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::MainView;
use crate::comps::card;
use crate::comps::page;
use crate::plugins::hash::calc_file_hash;
use crate::plugins::hash::calc_text_hash;

#[derive(Clone, Debug, PartialEq)]
enum InputType {
    Text,
    File,
}

pub struct HashPage {
    input: Entity<InputState>,
    input_type: InputType,
    output: Entity<InputState>,
}

impl HashPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line());
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());

            Self {
                input,
                input_type: InputType::Text,
                output,
            }
        }))
    }

    fn calc_hash(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let in_val = self.input.read(cx).value().to_string();
        if in_val.trim().is_empty() {
            return;
        }

        let result = match self.input_type {
            InputType::Text => calc_text_hash(&in_val),
            InputType::File => calc_file_hash(&in_val),
        };

        if let Ok(out_val) = result {
            self.output.update(cx, |state, cx2| {
                state.set_value(out_val.to_string(), window, cx2);
            });
        }
        cx.notify();
    }

    fn browse_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input.update(cx, |state, cx2| {
            state.set_value("/path/to/file".to_string(), window, cx2);
        });
        cx.notify();
    }
}

impl Render for HashPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let input_type = self.input_type.clone();

        let input_bg = rgb(INPUT_BG);
        let input_padding = Edges::all(px(INPUT_PADDING));

        page()
            .size_full()
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("输入类型"))
                        .child(
                            div().flex().flex_col().justify_end().paddings(input_padding).child(
                                RadioGroup::horizontal("input-type")
                                    .selected_index(match input_type {
                                        InputType::Text => Some(0),
                                        InputType::File => Some(1),
                                    })
                                    .child("文本类型")
                                    .child("文件类型")
                                    .on_change(cx.listener(|this, index, _window, cx| {
                                        let input_type = match *index {
                                            0 => InputType::Text,
                                            1 => InputType::File,
                                            _ => InputType::Text,
                                        };
                                        this.input_type = input_type;
                                        cx.notify();
                                    })),
                            ),
                        ),
                ),
            )
            .child(
                card()
                    .flex_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_sm().text_color(white()).child("输入内容"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(COMMON_GAP))
                                    .child(
                                        Button::new("browse-file")
                                            .label("选择文件")
                                            .disabled(input_type != InputType::File)
                                            .on_click(cx.listener(|this, _ev, window, cx| {
                                                this.browse_file(window, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("calc-hash")
                                            .primary()
                                            .label("计算Hash")
                                            .on_click(cx.listener(|this, _ev, window, cx| {
                                                this.calc_hash(window, cx);
                                            })),
                                    ),
                            ),
                    )
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
                card()
                    .flex_1()
                    .child(div().text_sm().text_color(white()).child("输出结果"))
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
                                    .font_family("monospace")
                                    .disabled(true)
                                    .h_full(),
                            ),
                    ),
            )
    }
}
