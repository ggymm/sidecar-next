use gpui::*;

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .p_5()
        .gap_3()
        .min_w_0()
        .min_h_0()
        .bg(rgb(0x333333))
        .rounded_lg()
}
