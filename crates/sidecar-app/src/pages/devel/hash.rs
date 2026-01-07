use std::{
    fs::File,
    io::{BufReader, Read},
};

use digest::Digest;
use gpui::*;
use gpui_component::{Disableable, input::InputState, radio::RadioGroup};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

use crate::{
    MainView,
    comps::{button, card, label, page, textarea},
};

const CHUNK_SIZE: usize = 512 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct HashResults {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

impl HashResults {
    pub fn to_string(&self) -> String {
        format!(
            "MD5       {}\nSHA1      {}\nSHA256    {}\nSHA512    {}",
            self.md5, self.sha1, self.sha256, self.sha512
        )
    }
}

pub fn calc_text_hash(text: &str) -> std::io::Result<HashResults> {
    let bytes = text.as_bytes();

    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    md5.update(bytes);
    sha1.update(bytes);
    sha256.update(bytes);
    sha512.update(bytes);

    Ok(HashResults {
        md5: hex::encode(md5.finalize()),
        sha1: hex::encode(sha1.finalize()),
        sha256: hex::encode(sha256.finalize()),
        sha512: hex::encode(sha512.finalize()),
    })
}

pub fn calc_file_hash(path: &str) -> std::io::Result<HashResults> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, file);
    let mut buffer = vec![0u8; CHUNK_SIZE];

    let mut md5 = Md5::new();
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut sha512 = Sha512::new();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let slice = &buffer[..n];
        md5.update(slice);
        sha1.update(slice);
        sha256.update(slice);
        sha512.update(slice);
    }

    Ok(HashResults {
        md5: hex::encode(md5.finalize()),
        sha1: hex::encode(sha1.finalize()),
        sha256: hex::encode(sha256.finalize()),
        sha512: hex::encode(sha512.finalize()),
    })
}

#[derive(Clone, Debug, PartialEq)]
enum InputType {
    Text,
    File,
}

pub struct HashPage {
    input: Entity<InputState>,
    input_type: InputType,
    output: Entity<InputState>,
}

impl HashPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input = cx.new(|cx| InputState::new(window, cx).multi_line(true));
            let output = cx.new(|cx| InputState::new(window, cx).multi_line(true));

            Self {
                input,
                input_type: InputType::Text,
                output,
            }
        }))
    }

    fn calc_hash(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let in_val = self.input.read(cx).value().to_string();
        if in_val.trim().is_empty() {
            return;
        }

        let result = match self.input_type {
            InputType::Text => calc_text_hash(&in_val),
            InputType::File => calc_file_hash(&in_val),
        };

        if let Ok(out_val) = result {
            self.output.update(cx, |state, cx2| {
                state.set_value(out_val.to_string(), window, cx2);
            });
        }
        cx.notify();
    }

    fn browse_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.input.update(cx, |state, cx2| {
            state.set_value("/path/to/file".to_string(), window, cx2);
        });
        cx.notify();
    }
}

impl Render for HashPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let input_type = self.input_type.clone();

        page()
            .size_full()
            .child(
                card().child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(label("输入类型"))
                        .child(
                            div().flex().flex_col().justify_end().p_2().child(
                                RadioGroup::horizontal("input-type")
                                    .selected_index(match input_type {
                                        InputType::Text => Some(0),
                                        InputType::File => Some(1),
                                    })
                                    .child("文本类型")
                                    .child("文件类型")
                                    .on_click(cx.listener(|this, index, _window, cx| {
                                        let input_type = match *index {
                                            0 => InputType::Text,
                                            1 => InputType::File,
                                            _ => InputType::Text,
                                        };
                                        this.input_type = input_type;
                                        cx.notify();
                                    })),
                            ),
                        ),
                ),
            )
            .child(
                card()
                    .flex_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(label("输入内容"))
                            .child(
                                div()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        button(cx, "browse-file")
                                            .label("选择文件")
                                            .disabled(input_type != InputType::File)
                                            .on_click(cx.listener(|this, _ev, window, cx| {
                                                this.browse_file(window, cx);
                                            })),
                                    )
                                    .child(button(cx, "calc-hash").label("计算Hash").on_click(cx.listener(
                                        |this, _ev, window, cx| {
                                            this.calc_hash(window, cx);
                                        },
                                    ))),
                            ),
                    )
                    .child(textarea(&self.input, |input| input)),
            )
            .child(
                card()
                    .flex_1()
                    .child(label("输出结果"))
                    .child(textarea(&self.output, |input| {
                        input.font_family("monospace").disabled(true)
                    })),
            )
    }
}
