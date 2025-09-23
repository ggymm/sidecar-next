use gpui::*;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use gpui_component::StyledExt;
use ::pem;
use x509_parser::prelude::*;

use crate::MainView;
use crate::CARD_BG;
use crate::CARD_GAP;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;

pub struct CertPage {
    input: Entity<InputState>,
    output: Entity<InputState>,
    last_input: SharedString,
    updating: bool,
}

impl CertPage {
    pub fn build(window: &mut Window, cx: &mut Context<MainView>) -> AnyView {
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.updating {
            let in_val = self.input.read(cx).value();

            if in_val != self.last_input {
                self.updating = true;

                // 解析 x509 证书
                let parsed_result = if in_val.is_empty() {
                    String::new()
                } else {
                    // 如果输入文本包含 字符串的 \n 替换为实际换行符
                    let text = in_val.replace("\\n", "\n");

                    match ::pem::parse(&text) {
                        Ok(pem_data) => {
                            match X509Certificate::from_der(&pem_data.contents()) {
                                Ok((_, cert)) => {
                                    let mut result = String::new();
                                    result.push_str("证书信息:\n");
                                    result.push_str("===========================================\n\n");

                                    result.push_str(&format!("主题: {}\n\n", cert.subject()));
                                    result.push_str(&format!("颁发者: {}\n\n", cert.issuer()));
                                    result.push_str(&format!("序列号: {}\n\n", cert.serial.to_str_radix(16).to_uppercase()));
                                    result.push_str(&format!("版本: v{}\n\n", cert.version()));

                                    result.push_str("有效期:\n");
                                    result.push_str(&format!("  从: {}\n", cert.validity().not_before));
                                    result.push_str(&format!("  到: {}\n\n", cert.validity().not_after));

                                    result.push_str(&format!("签名算法: {}\n", cert.signature_algorithm.algorithm));

                                    result
                                }
                                Err(e) => format!("解析失败: {}", e),
                            }
                        }
                        Err(e) => format!("解析失败: {}", e),
                    }
                };

                self.output.update(cx, |state, cx2| state.set_value(parsed_result, window, cx2));
                self.last_input = in_val;
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
                    .child("证书解析"),
            )
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .flex_1()
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
                                    .rounded_md()
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
                                    .rounded_md()
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
