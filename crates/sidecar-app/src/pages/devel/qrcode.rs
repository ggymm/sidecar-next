use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use gpui::*;
use gpui_component::input::InputState;
use image;
use rqrr::PreparedImage;

use crate::{
    MainView,
    comps::{button, card, label, page, textarea},
};

pub fn parse_qrcode<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let p = path.as_ref();
    // 读取图片
    let img = match image::open(p) {
        Ok(img) => img,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not open image: {} {}", p.display(), e),
            ));
        }
    };

    let img = img.to_luma8();
    let mut img = PreparedImage::prepare(img);

    let grids = img.detect_grids();
    if grids.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("can not find qrcode in image: {}", p.display()),
        ));
    }

    let (_, content) = match grids[0].decode() {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("can not decode qrcode in image: {} {}", p.display(), e),
            ));
        }
    };

    Ok(content)
}

pub struct QrcodePage {
    path: Option<PathBuf>,
    output: Entity<InputState>,
}

impl QrcodePage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let output = cx.new(|cx| InputState::new(window, cx).multi_line(true));

            Self { path: None, output }
        }))
    }

    fn choose_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            multiple: false,
            directories: false,
            prompt: Some("选择二维码图片".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            match rx.await {
                Ok(Ok(Some(mut paths))) => {
                    if let Some(path) = paths.pop() {
                        let _ = cx.update(|_window, cx| {
                            let _ = this.update(cx, |this, _cx| {
                                this.path = Some(path.clone());
                            });
                        });

                        let qrcode = path.clone();
                        let output = cx
                            .background_executor()
                            .spawn(async move { parse_qrcode(&qrcode) })
                            .await
                            .unwrap_or_else(|e| format!("解析失败: {}", e));

                        let _ = cx.update(|window, cx| {
                            let _ = this.update(cx, |this, cx| {
                                this.output.update(cx, |state, cx2| {
                                    state.set_value(output, window, cx2);
                                });
                            });
                        });
                    }
                }
                Ok(Ok(None)) => {
                    // 忽略
                }
                Ok(Err(e)) => {
                    let _ = cx.update(|window, cx| {
                        let _ = this.update(cx, |this, cx| {
                            this.output.update(cx, |state, cx2| {
                                state.set_value(format!("无法打开文件选择器: {}", e), window, cx2);
                            });
                        });
                    });
                }
                Err(_canceled) => {
                    // 忽略
                }
            }
        })
        .detach();
    }
}

impl Render for QrcodePage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page()
            .size_full()
            .child(
                card()
                    .flex_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(label("二维码"))
                            .child(button(cx, "choose-file").label("选择文件").on_click(cx.listener(
                                |this, _ev, window, cx| {
                                    this.choose_file(window, cx);
                                },
                            ))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .min_w_0()
                            .min_h_0()
                            .child(if let Some(path) = &self.path {
                                img(path.clone())
                                    .max_w_full()
                                    .max_h_full()
                                    .object_fit(ObjectFit::Contain)
                            } else {
                                img("")
                            }),
                    ),
            )
            .child(
                card()
                    .h_64()
                    .child(label("解析结果"))
                    .child(textarea(&self.output, |input| input)),
            )
    }
}
