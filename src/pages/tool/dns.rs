use gpui::*;
use gpui_component::{input::InputState, Disableable};

use crate::{
    comps::{button, card, label, page, textarea},
    plugins::dns::start_dns_query,
    MainView,
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
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());
            Self {
                input,
                output,
                running: false,
            }
        }))
    }

    fn append_line(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        line: impl Into<String>,
    ) {
        let line = line.into();
        self.output.update(cx, |state, cx2| {
            let mut cur = state.value().to_string();
            if !cur.is_empty() {
                cur.push('\n');
            }
            cur.push_str(&line);
            state.set_value(cur, window, cx2);
        });
    }

    fn start_query(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.running {
            return;
        }
        let domain = self.input.read(cx).value().to_string();
        if domain.trim().is_empty() {
            return;
        }
        // Clear output and mark running
        self.output.update(cx, |state, cx2| {
            state.set_value(String::new(), window, cx2);
        });
        self.running = true;
        cx.notify();

        let rx = start_dns_query(domain.clone());

        cx.spawn_in(window, async move |this, cx| {
            while let Ok(line) = rx.recv() {
                let _ = cx.update(|window, cx| {
                    let _ = this.update(cx, |this, cx| {
                        this.append_line(window, cx, line);
                    });
                });
            }
            let _ = cx.update(|_window, cx| {
                let _ = this.update(cx, |this, _cx| {
                    this.running = false;
                });
            });
        })
        .detach();
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
                        .gap_5()
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
