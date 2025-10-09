use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use gpui::*;
use gpui_component::ContextModal;
use gpui_component::StyledExt;
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;

use crate::COMMON_GAP;
use crate::INPUT_BG;
use crate::MainView;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;
use crate::comps::Card;

fn gen_mac() -> String {
    let mut x = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let mut b = [0u8; 6];
    for i in 0..6 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        b[i] = x as u8;
    }
    b[0] = (b[0] | 0x02) & 0xFE;
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        b[0], b[1], b[2], b[3], b[4], b[5]
    )
}

fn gen_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn gen_phone() -> String {
    let mut x = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let mut next_digit = || -> u8 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        (x % 10) as u8
    };

    let mut s = String::with_capacity(11);
    s.push('1');
    for _ in 0..10 {
        s.push((b'0' + next_digit()) as char);
    }
    s
}

pub struct RandomPage {
    mac_output: Entity<InputState>,
    uuid_output: Entity<InputState>,
    phone_output: Entity<InputState>,
}

impl RandomPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let mac_output = cx.new(|cx| InputState::new(window, cx));
            let uuid_output = cx.new(|cx| InputState::new(window, cx));
            let phone_output = cx.new(|cx| InputState::new(window, cx));

            Self {
                mac_output,
                uuid_output,
                phone_output,
            }
        }))
    }

    fn build_item(
        &self,
        cx: &mut Context<Self>,
        label: &str,
        output: Entity<InputState>,
        generate: fn() -> String,
    ) -> impl IntoElement {
        let input_bg = rgb(INPUT_BG);

        let output_for_gen = output.clone();
        let output_for_copy = output.clone();

        Card::new().child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(div().text_sm().text_color(white()).child(label.to_string()))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(COMMON_GAP))
                        .child(
                            TextInput::new(&output)
                                .w(px(480.))
                                .bg(input_bg)
                                .focus_bordered(false)
                                .text_color(white()),
                        )
                        .child(
                            Button::new(("gen", output.entity_id()))
                                .info()
                                .label("生成")
                                .on_click(cx.listener(move |_this, _ev, window, cx| {
                                    let val = generate();
                                    output_for_gen.update(cx, |state, cx2| {
                                        state.set_value(val, window, cx2);
                                    });
                                })),
                        )
                        .child(
                            Button::new(("copy", output.entity_id()))
                                .info()
                                .label("复制")
                                .on_click(cx.listener(move |_this, _ev, window, cx| {
                                    let value = output_for_copy.read(cx).value();
                                    if !value.is_empty() {
                                        window.push_notification("已复制到剪切板", cx);
                                        cx.write_to_clipboard(ClipboardItem::new_string(value.to_string()));
                                    }
                                })),
                        ),
                ),
        )
    }
}

impl Render for RandomPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let page_padding = Edges::all(px(PAGE_PADDING));

        div()
            .v_flex()
            .size_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .child(self.build_item(cx, "MAC地址", self.mac_output.clone(), gen_mac))
            .child(self.build_item(cx, "UUID.v4", self.uuid_output.clone(), gen_uuid))
            .child(self.build_item(cx, "手机号码", self.phone_output.clone(), gen_phone))
    }
}
