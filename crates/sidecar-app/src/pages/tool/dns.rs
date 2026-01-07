use gpui::*;
use gpui_component::{Disableable, input::InputState};

use crate::{
    MainView,
    comps::{button, card, label, page, textarea},
};

pub struct DnsPage {
    input: Entity<InputState>,
    output: Entity<InputState>,
    running: bool,
}

impl DnsPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).placeholder("输入域名，例如: example.com"));
            let output = cx.new(|cx| InputState::new(window, cx).multi_line(true));
            Self {
                input,
                output,
                running: false,
            }
        }))
    }

    fn start_query(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}

impl Render for DnsPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let running = self.running;

        page()
            .size_full()
            .child(
                card().child(
                    div()
                        .flex()
                        .gap_4()
                        .items_center()
                        .child(textarea(&self.input, |input| input))
                        .child(
                            button(cx, "dns-query")
                                .disabled(running)
                                .label(if running { "查询中..." } else { "查询" })
                                .on_click(cx.listener(|this, _ev, window, cx| {
                                    this.start_query(window, cx);
                                })),
                        ),
                ),
            )
            .child(
                card()
                    .flex_1()
                    .child(label("输出"))
                    .child(textarea(&self.output, |input| {
                        input.font_family("monospace").disabled(true)
                    })),
            )
    }
}
