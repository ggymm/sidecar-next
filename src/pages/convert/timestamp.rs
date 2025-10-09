use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use gpui::*;
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;

use crate::COMMON_GAP;
use crate::INPUT_BG;
use crate::MainView;
use crate::comps::card;
use crate::comps::page;

pub struct TimestampPage {
    tz_input: Entity<InputState>,
    ts_input: Entity<InputState>,
    last_input: SharedString,
    common_output: String,
    iso8601_output: String,
    rfc7231_output: String,
    updating: bool,
}

impl TimestampPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let ts = Utc::now().timestamp().to_string();
            let tz_input = cx.new(|cx| InputState::new(window, cx));
            let ts_input = cx.new(|cx| InputState::new(window, cx).default_value(&ts));

            let mut page = Self {
                tz_input,
                ts_input,
                last_input: ts.clone().into(),
                common_output: String::new(),
                iso8601_output: String::new(),
                rfc7231_output: String::new(),
                updating: false,
            };
            page.update_outputs(&ts);
            page
        }))
    }

    fn update_outputs(
        &mut self,
        timestamp_str: &str,
    ) {
        if timestamp_str.trim().is_empty() {
            self.common_output = String::new();
            self.iso8601_output = String::new();
            self.rfc7231_output = String::new();
            return;
        }

        match timestamp_str.trim().parse::<i64>() {
            Ok(timestamp) => {
                let (sec, _offset_secs) = if timestamp > 9999999999 {
                    (timestamp / 1000, 8 * 3600)
                } else {
                    (timestamp, 8 * 3600)
                };

                if let Some(dt_utc) = DateTime::from_timestamp(sec, 0) {
                    let offset = FixedOffset::east_opt(8 * 3600).unwrap_or(FixedOffset::east_opt(0).unwrap());
                    let dt_local = dt_utc.with_timezone(&offset);

                    self.common_output = dt_local.format("%Y-%m-%d %H:%M:%S").to_string();
                    self.iso8601_output = dt_local.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
                    self.rfc7231_output = dt_utc.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
                } else {
                    self.common_output = "无效的时间戳".to_string();
                    self.iso8601_output = "无效的时间戳".to_string();
                    self.rfc7231_output = "无效的时间戳".to_string();
                }
            }
            Err(_) => {
                self.common_output = "无效的时间戳".to_string();
                self.iso8601_output = "无效的时间戳".to_string();
                self.rfc7231_output = "无效的时间戳".to_string();
            }
        }
    }

    fn update_timestamp(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ts = Utc::now().timestamp().to_string();
        self.ts_input.update(cx, |state, cx2| {
            state.set_value(ts.clone(), window, cx2);
        });
        self.update_outputs(&ts);
        cx.notify();
    }
}

impl Render for TimestampPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if !self.updating {
            let in_val = self.ts_input.read(cx).value();
            if in_val != self.last_input {
                self.update_outputs(&in_val);
                self.last_input = in_val;
            }
        }

        let input_bg = rgb(INPUT_BG);

        page()
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("时间戳"))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(COMMON_GAP))
                                .child(
                                    TextInput::new(&self.ts_input)
                                        .w(px(240.))
                                        .bg(input_bg)
                                        .focus_bordered(false)
                                        .text_color(white()),
                                )
                                .child(
                                    Button::new("update_timestamp")
                                        .info()
                                        .label("当前时间")
                                        .h_full()
                                        .on_click(cx.listener(|this, _ev, window, cx| {
                                            this.update_timestamp(window, cx);
                                        })),
                                ),
                        ),
                ),
            )
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("时区"))
                        .child(
                            TextInput::new(&self.tz_input)
                                .w(px(360.))
                                .bg(input_bg)
                                .focus_bordered(false)
                                .text_color(white()),
                        ),
                ),
            )
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("Common"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0xA0A0A0))
                                .child(self.common_output.clone()),
                        ),
                ),
            )
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("ISO 8601"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0xA0A0A0))
                                .child(self.iso8601_output.clone()),
                        ),
                ),
            )
            .child(
                card().flex_1().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(div().text_sm().text_color(white()).child("RFC 7231"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0xA0A0A0))
                                .child(self.rfc7231_output.clone()),
                        ),
                ),
            )
    }
}
