use gpui::*;
use gpui_component::StyledExt;

pub struct Base64Page;

impl Render for Base64Page {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .child("Base64编解码")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .gap_5()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(rgb(0x2D2D2D))
                            .border_1()
                            .border_color(rgb(0x3C3C3C))
                            .rounded_lg()
                            .p_5()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(white())
                                    .child("原始内容")
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .flex_1()
                                    .bg(rgb(0x1E1E1E))
                                    .border_1()
                                    .border_color(rgb(0x404040))
                                    .rounded_md()
                                    .p_3()
                                    .text_color(white())
                                    .child("请输入要编码的内容...")
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .bg(rgb(0x2D2D2D))
                            .border_1()
                            .border_color(rgb(0x3C3C3C))
                            .rounded_lg()
                            .p_5()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(white())
                                    .child("编码内容")
                            )
                            .child(
                                div()
                                    .mt_4()
                                    .flex_1()
                                    .bg(rgb(0x1E1E1E))
                                    .border_1()
                                    .border_color(rgb(0x404040))
                                    .rounded_md()
                                    .p_3()
                                    .text_color(white())
                                    .child("编码结果将显示在这里...")
                            )
                    )
            )
    }
}

