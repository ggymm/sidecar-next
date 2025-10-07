use gpui::*;
use crate::comps::StyledExt;

use crate::CARD_BG;
use crate::CARD_PADDING;
use crate::COMMON_GAP;
use crate::COMMON_PADDING_M;
use crate::MainView;
use crate::PAGE_PADDING;
use crate::pages::utils::format_bytes;
use crate::plugins::system::basic_info::BasicInfo;
use crate::plugins::system::basic_info::collect_basic_info;
use crate::plugins::system::dynamic_info::DynamicInfo;

pub struct HomePage {
    basic_info: Option<BasicInfo>,
    dynamic_info: Option<DynamicInfo>,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            basic_info: None,
            dynamic_info: None,
        }
    }

    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| HomePage::new()))
    }

    fn build_info(
        &self,
        label: String,
        value: String,
    ) -> impl IntoElement {
        div()
            .flex()
            .py(px(COMMON_PADDING_M))
            .child(div().w(px(180.)).text_sm().text_color(white()).child(label))
            .child(div().ml_5().flex_1().text_sm().text_color(white()).child(value))
    }

    fn basic_load(
        &mut self,
        cx: &mut Context<Self>,
    ) {
        if self.basic_info.is_none() {
            self.basic_info = Some(collect_basic_info());
            cx.notify();
        }
    }

    fn dynamic_load(
        &mut self,
        _cx: &mut Context<Self>,
    ) {
        if self.dynamic_info.is_none() {}
    }
}

impl Render for HomePage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // 加载系统信息
        self.basic_load(cx);
        self.dynamic_load(cx);

        let card_bg = rgb(CARD_BG);

        let page_padding = Edges::all(px(PAGE_PADDING));
        let card_padding = Edges::all(px(CARD_PADDING));

        div()
            .v_flex()
            .size_full()
            .paddings(page_padding)
            .gap(px(COMMON_GAP))
            .child(if let Some(info) = &self.basic_info {
                div()
                    .flex()
                    .gap(px(COMMON_GAP))
                    .child(
                        div()
                            .flex_1()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(white())
                                    .mb_4()
                                    .child("系统信息"),
                            )
                            .child(self.build_info("用户".to_string(), info.env.current_user.clone()))
                            .child(self.build_info("序列号".to_string(), info.os.serial_number.clone()))
                            .child(
                                self.build_info(
                                    "操作系统".to_string(),
                                    format!("{} {}", info.os.name, info.os.version),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(white())
                                    .mb_4()
                                    .child("网络信息"),
                            )
                            .child(if let Some(network) = &info.network {
                                div()
                                    .child(self.build_info("网卡名称".to_string(), network.name.clone()))
                                    .child(self.build_info("IP 地址".to_string(), network.ip_address.clone()))
                                    .child(self.build_info("MAC 地址".to_string(), network.mac_address.clone()))
                            } else {
                                div().child(self.build_info("状态".to_string(), "网络信息获取失败".to_string()))
                            }),
                    )
            } else {
                div()
            })
            .child(if let Some(info) = &self.basic_info {
                div()
                    .flex()
                    .gap(px(COMMON_GAP))
                    .child(
                        div()
                            .flex_1()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(white())
                                    .mb_4()
                                    .child("CPU 信息"),
                            )
                            .child(self.build_info("架构".to_string(), info.cpu.arch.clone()))
                            .child(self.build_info("处理器".to_string(), info.cpu.brand.clone()))
                            .child(self.build_info("物理核心".to_string(), format!("{} 个", info.cpu.physical_cores)))
                            .child(self.build_info("频率".to_string(), format!("{} MHz", info.cpu.frequency))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .bg(card_bg)
                            .rounded_lg()
                            .paddings(card_padding)
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(white())
                                    .mb_4()
                                    .child("内存信息"),
                            )
                            .child(self.build_info("已使用".to_string(), format_bytes(info.memory.used)))
                            .child(self.build_info("总内存".to_string(), format_bytes(info.memory.total)))
                            .child(self.build_info("交换分区".to_string(), format_bytes(info.memory.total_swap)))
                            .child(self.build_info(
                                "使用率".to_string(),
                                format!("{:.1}%", (info.memory.used as f64 / info.memory.total as f64) * 100.0),
                            )),
                    )
            } else {
                div()
            })
    }
}
