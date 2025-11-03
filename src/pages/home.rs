use gpui::*;
use gpui_component::StyledExt;

use crate::{
    comps::{card, label, page},
    pages::utils::format_bytes,
    plugins::system::basic_info::{collect_basic_info, BasicInfo},
    MainView,
};

pub struct HomePage {
    basic_info: Option<BasicInfo>,
}

impl HomePage {
    pub fn new() -> Self {
        Self { basic_info: None }
    }

    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| HomePage::new()))
    }

    fn basic_info_ref(
        &mut self,
        cx: &mut Context<Self>,
    ) -> &BasicInfo {
        if self.basic_info.is_none() {
            self.basic_info = Some(collect_basic_info());
            cx.notify();
        }
        self.basic_info.as_ref().unwrap()
    }
}

fn card_name(title: &str) -> impl IntoElement {
    div()
        .text_lg()
        .font_semibold()
        .text_color(white())
        .mb_4()
        .child(title.to_string())
}

fn card_item(
    title: &str,
    value: impl AsRef<str>,
) -> impl IntoElement {
    let value = value.as_ref();
    div().flex().py_2().child(label(title.to_string()).w_48()).child(
        div()
            .ml_5()
            .flex_1()
            .text_sm()
            .text_color(white())
            .child(value.to_string()),
    )
}

impl Render for HomePage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let info = self.basic_info_ref(cx);
        let network = info.network.as_ref();

        page()
            .size_full()
            .child(
                div()
                    .flex()
                    .gap_5()
                    .child(
                        card()
                            .flex_1()
                            .child(card_name("系统信息"))
                            .child(card_item("用户", &info.env.current_user.trim()))
                            .child(card_item("序列号", &info.os.serial_number.trim()))
                            .child(card_item(
                                "操作系统",
                                format!("{} {}", &info.os.name.trim(), &info.os.version.trim()),
                            )),
                    )
                    .child(match network {
                        Some(net) => card()
                            .flex_1()
                            .child(card_name("网络信息"))
                            .child(card_item("网卡名称", net.name.trim()))
                            .child(card_item("IP 地址", net.ip_address.trim()))
                            .child(card_item("MAC 地址", net.mac_address.trim())),
                        None => card(),
                    }),
            )
            .child(
                div()
                    .flex()
                    .gap_5()
                    .child(
                        card()
                            .flex_1()
                            .child(card_name("CPU 信息"))
                            .child(card_item("架构", &info.cpu.arch.trim()))
                            .child(card_item("处理器", &info.cpu.brand.trim()))
                            .child(card_item("物理核心", &info.cpu.physical_cores.to_string())),
                    )
                    .child(
                        card()
                            .flex_1()
                            .child(card_name("内存信息"))
                            .child(card_item("已使用", format_bytes(info.memory.used)))
                            .child(card_item("总内存", format_bytes(info.memory.total)))
                            .child(card_item("交换分区", format_bytes(info.memory.total_swap))),
                    ),
            )
    }
}
