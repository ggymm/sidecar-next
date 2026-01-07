use base64::{Engine, engine::general_purpose};
use gpui::*;
use gpui_component::input::InputState;

use crate::{
    MainView,
    comps::{card, label, page, textarea},
    pages::utils::strip_str,
};

pub struct Base64Page {
    input: Entity<InputState>,
    output: Entity<InputState>,
    last_input: SharedString,
    last_output: SharedString,
    updating: bool,
}

impl Base64Page {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line(true));
            let output = cx.new(|cx| InputState::new(window, cx).multi_line(true));

            Self {
                input,
                output,
                last_input: SharedString::default(),
                last_output: SharedString::default(),
                updating: false,
            }
        }))
    }
}

impl Render for Base64Page {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if !self.updating {
            let in_val = self.input.read(cx).value();
            let out_val = self.output.read(cx).value();

            enum Src {
                In,
                Out,
            }

            let src = if in_val != self.last_input {
                Some(Src::In)
            } else if out_val != self.last_output {
                Some(Src::Out)
            } else {
                None
            };

            if let Some(src) = src {
                self.updating = true;
                match src {
                    Src::In => {
                        let enc = general_purpose::STANDARD.encode(in_val.as_str());
                        self.output.update(cx, |state, cx2| {
                            state.set_value(enc, window, cx2);
                        });
                        self.last_input = in_val;
                        self.last_output = self.output.read(cx).value();
                    }
                    Src::Out => {
                        let cleaned = strip_str(out_val.as_str());
                        if let Ok(bytes) = general_purpose::STANDARD.decode(cleaned.as_bytes()) {
                            let s = String::from_utf8(bytes)
                                .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).to_string());
                            self.input.update(cx, |state, cx2| {
                                state.set_value(s, window, cx2);
                            });
                        }
                        self.last_input = self.input.read(cx).value();
                        self.last_output = out_val;
                    }
                }
                self.updating = false;
            }
        }

        page()
            .size_full()
            .child(
                card()
                    .flex_1()
                    .child(label("原始内容"))
                    .child(textarea(&self.input, |input| input)),
            )
            .child(
                card()
                    .flex_1()
                    .child(label("编码内容"))
                    .child(textarea(&self.output, |input| input)),
            )
    }
}
