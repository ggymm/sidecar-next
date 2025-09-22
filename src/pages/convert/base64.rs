use gpui::*;
use gpui_component::input::{InputState, TextInput};
use gpui_component::StyledExt;
use crate::theme;
use base64::{engine::general_purpose, Engine as _};

pub struct Base64Page {
    input: Entity<InputState>,
    output: Entity<InputState>,
    updating: bool,
    last_input: SharedString,
    last_output: SharedString,
}

impl Base64Page {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("输入内容"));
        let output = cx.new(|cx| InputState::new(window, cx).multi_line().placeholder("输出内容"));

        // 下一帧聚焦输入框，保证可输入
        cx.on_next_frame(window, |this, window, cx| {
            cx.focus_view(&this.input, window);
        });

        Self {
            input,
            output,
            updating: false,
            last_input: SharedString::default(),
            last_output: SharedString::default(),
        }
    }
}

impl Render for Base64Page {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 双向同步（左输入→右Base64，右输入→左解码）。仅在内容变化时触发，避免循环。
        if !self.updating {
            let in_val = self.input.read(cx).value();
            if in_val != self.last_input {
                self.updating = true;
                let enc = general_purpose::STANDARD.encode(in_val.as_str());
                self.output
                    .update(cx, |state, cx2| state.set_value(enc, window, cx2));
                self.last_input = in_val;
                self.last_output = self.output.read(cx).value();
                self.updating = false;
            } else {
                let out_val = self.output.read(cx).value();
                if out_val != self.last_output {
                    self.updating = true;
                    let cleaned: String = out_val.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    if let Ok(bytes) = general_purpose::STANDARD.decode(cleaned.as_bytes()) {
                        let s = String::from_utf8(bytes)
                            .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).to_string());
                        self.input
                            .update(cx, |state, cx2| state.set_value(s, window, cx2));
                    }
                    self.last_output = out_val;
                    self.last_input = self.input.read(cx).value();
                    self.updating = false;
                }
            }
        }

        // UI：左右两栏面板 + 标题，深色主题风格
        let panel_bg = rgb(theme::COLOR_PANEL_BG);
        let inner_bg = rgb(theme::COLOR_PANEL_INNER_BG);
        let border = rgb(theme::COLOR_PANEL_BORDER);

        div()
            .key_context("Input")
            .size_full()
            .p_6()
            .gap_4()
            .v_flex()
            .child(div().text_lg().font_semibold().text_color(white()).child("Base64 编解码"))
            .child(
                // 垂直堆叠：上 原始内容 / 下 编码内容
                div()
                    .v_flex()
                    .gap_4()
                    .flex_1()
                    // 上：原始内容
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(panel_bg)
                            .border_1()
                            .border_color(border)
                            .rounded_lg()
                            .p_5()
                            .gap_3()
                            .child(div().text_sm().text_color(white()).child("原始内容"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(inner_bg)
                                    .border_1()
                                    .border_color(rgb(0x404040))
                                    .rounded_md()
                                    .p_2()
                                    .child(
                                        TextInput::new(&self.input)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .h_full(),
                                    ),
                            ),
                    )
                    // 下：编码内容
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(panel_bg)
                            .border_1()
                            .border_color(border)
                            .rounded_lg()
                            .p_5()
                            .gap_3()
                            .child(div().text_sm().text_color(white()).child("编码内容"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(inner_bg)
                                    .border_1()
                                    .border_color(rgb(0x404040))
                                    .rounded_md()
                                    .p_2()
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


// 使用 base64 库进行编解码
