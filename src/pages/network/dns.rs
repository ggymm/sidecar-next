use gpui::*;
use gpui_component::{Disableable, StyledExt};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
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
use crate::plugins::dns::start_dns_query;

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
            let input = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("输入域名，例如: example.com")
            });
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());
            Self { input, output, running: false }
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
            // Stream lines as they arrive
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
        let card_bg = rgb(CARD_BG);
        let input_bg = rgb(INPUT_BG);

        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        let running = self.running;

        div()
            .v_flex()
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .bg(card_bg)
                    .rounded_lg()
                    .paddings(card_padding)
                    .gap(px(CARD_GAP))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(CARD_GAP))
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
                                            .text_color(white()),
                                    ),
                            )
                            .child(
                                Button::new("dns-query")
                                    .primary()
                                    .disabled(running)
                                    .label(if running { "查询中..." } else { "查询" })
                                    .on_click(cx.listener(|this, _ev, window, cx| {
                                        this.start_query(window, cx);
                                    })),
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
                    .child(div().text_sm().text_color(white()).child("输出"))
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
