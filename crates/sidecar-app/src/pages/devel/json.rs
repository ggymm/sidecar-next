use gpui::*;
use gpui_component::{ActiveTheme, Disableable, input::InputState};

use crate::{
    MainView,
    comps::{button, card, page, textarea},
    pages::utils::format_json,
};

pub struct JsonPage {
    error: Option<String>,
    running: bool,
    formatted: bool,
    input_original: Entity<InputState>,
    input_formatted: Entity<InputState>,
}

impl JsonPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input_original = cx.new(|cx| InputState::new(window, cx).multi_line(true));
            let input_formatted = cx.new(|cx| {
                InputState::new(window, cx)
                    .code_editor("json")
                    .searchable(true)
                    .line_number(true)
            });

            Self {
                error: None,
                running: false,
                formatted: false,
                input_original,
                input_formatted,
            }
        }))
    }

    fn format_json(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.running {
            return;
        }
        self.error = None;
        self.running = true;
        let raw = self.input_original.read(cx).value().to_string();
        if raw.trim().is_empty() {
            self.formatted = false;
            cx.notify();
            return;
        }
        cx.notify();

        // 格式化 JSON 字符串
        match format_json(&raw) {
            Ok(formatted) => {
                self.input_formatted.update(cx, |state, cx2| {
                    state.set_value(formatted.clone(), window, cx2);
                    state.set_line_number(true, window, cx2);
                });
                self.error = None;
                self.running = false;
                self.formatted = true;
            }
            Err(err) => {
                self.error = Some(err);
                self.running = false;
                self.formatted = false;
            }
        }
        cx.notify();
    }
}

impl Render for JsonPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let mut actions = div().flex().gap_4().items_center();

        let format_button = if self.running {
            button(cx, "format").label("格式化中…").disabled(true)
        } else {
            button(cx, "format")
                .label("格式化")
                .on_click(cx.listener(|this, _ev, window, cx| {
                    this.format_json(window, cx);
                }))
        };
        actions = actions.child(format_button);

        if self.formatted {
            actions = actions.child(button(cx, "rollback").label("查看原文").on_click(cx.listener(
                |this, _ev, _window, cx| {
                    this.formatted = false;
                    cx.notify();
                },
            )));
        }

        let mut panel = card().flex_1().child(actions);
        if let Some(err) = self.error.as_ref() {
            panel = panel.child(
                div()
                    .px_4()
                    .py_2()
                    .rounded_lg()
                    .bg(theme.danger)
                    .text_sm()
                    .text_color(theme.danger_foreground)
                    .child(format!("解析失败：{}", err)),
            );
        }

        panel = if self.formatted {
            panel.child(textarea(&self.input_formatted, |input| input))
        } else {
            panel.child(textarea(&self.input_original, |input| input))
        };
        page().size_full().child(panel)
    }
}
