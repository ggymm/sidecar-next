use gpui::*;
use gpui_component::StyledExt;
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;
use std::path::PathBuf;

use crate::CARD_BG;
use crate::CARD_GAP;
use crate::CARD_PADDING;
use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::MainView;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;
use crate::plugins::qrcode;
use crate::plugins::qrcode::parse_qrcode;

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
            let output = cx.new(|cx| InputState::new(window, cx).multi_line());

            Self { path: None, output }
        }))
    }

    fn choose_file(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        std::thread::spawn(move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("图片文件", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
                .set_title("选择二维码图片")
                .pick_file()
            {
                let output = parse_qrcode(path.to_str().unwrap()).unwrap();

                println!("{}", output);

               
            }
        });

        self.output.update(cx, |state, cx2| {
            state.set_value("", window, cx2);
        });
    }
}

impl Render for QrcodePage {
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

        div()
            .v_flex()
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .child(
                div()
                    .flex_1()
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
                            .justify_between()
                            .child(div().text_sm().text_color(white()).child("二维码"))
                            .child(
                                Button::new("choose_file")
                                    .info()
                                    .label("选择文件")
                                    .on_click(cx.listener(|this, _ev, window, cx| {
                                        this.choose_file(window, cx);
                                    })),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if let Some(path) = &self.path { div() } else { div() }),
                    ),
            )
            .child(
                div()
                    .h(px(240.))
                    .flex()
                    .flex_col()
                    .bg(card_bg)
                    .rounded_lg()
                    .paddings(card_padding)
                    .gap(px(CARD_GAP))
                    .child(div().text_sm().text_color(white()).child("解析结果"))
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
                                    .h_full(),
                            ),
                    ),
            )
    }
}
