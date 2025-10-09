use gpui::*;
use gpui_component::input::InputState;
use openssl::x509::X509;

use crate::MainView;
use crate::comps::card;
use crate::comps::page;
use crate::comps::text;

pub struct CertPage {
    input: Entity<InputState>,
    output: Entity<InputState>,
    last_input: SharedString,
    updating: bool,
}

impl CertPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line());
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());

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
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if !self.updating {
            let in_val = self.input.read(cx).value();
            if in_val != self.last_input {
                self.updating = true;

                let pem = in_val.to_string();
                cx.spawn_in(window, async move |this, cx| {
                    let out_text = cx
                        .background_executor()
                        .spawn(async move {
                            if pem.trim().is_empty() {
                                return String::new();
                            }
                            X509::from_pem(pem.as_bytes())
                                .and_then(|cert| cert.to_text())
                                .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
                                .unwrap_or_else(|e| format!("解析失败: {}", e))
                        })
                        .await;

                    let _ = cx.update(|window, cx| {
                        let _ = this.update(cx, |this, cx| {
                            this.output.update(cx, |state, cx2| {
                                state.set_value(out_text, window, cx2);
                            });
                            this.last_input = this.input.read(cx).value();
                            this.updating = false;
                        });
                    });
                })
                .detach();
            }
        }

        page()
            .size_full()
            .child(
                card()
                    .h(px(240.))
                    .child(div().text_sm().text_color(white()).child("证书"))
                    .child(text(&self.input, |input| input.h_full())),
            )
            .child(
                card()
                    .flex_1()
                    .child(div().text_sm().text_color(white()).child("解析结果"))
                    .child(text(&self.output, |input| input.h_full())),
            )
    }
}
