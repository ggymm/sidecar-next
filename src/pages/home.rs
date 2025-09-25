use gpui::*;
use gpui_component::StyledExt;

use crate::MainView;
use crate::plugins::system::basic_info::BasicInfo;
use crate::plugins::system::basic_info::collect_basic_info;

pub struct HomePage {
    system_info: Option<BasicInfo>,
}

impl HomePage {
    pub fn new() -> Self {
        Self { system_info: None }
    }

    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| HomePage::new()))
    }

    fn load_system_info(
        &mut self,
        cx: &mut Context<Self>,
    ) {
        if self.system_info.is_none() {
            // 在后台线程中收集系统信息
            let basic_info = collect_basic_info();
            self.system_info = Some(basic_info);
            cx.notify();
        }
    }

    fn format_bytes(
        &self,
        bytes: u64,
    ) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }

    fn build_info_row(
        &self,
        label: String,
        value: String,
    ) -> impl IntoElement {
        div()
            .flex()
            .py_2()
            .child(div().w(px(160.)).text_sm().text_color(rgb(0xB0B0B0)).child(label))
            .child(div().ml_5().flex_1().text_sm().text_color(white()).child(value))
    }

    fn build_cpu_card(
        &self,
        info: &BasicInfo,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .bg(rgb(0x2D2D2D))
            .border_1()
            .border_color(rgb(0x3C3C3C))
            .rounded_lg()
            .p_5()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .mb_4()
                    .child("CPU 信息"),
            )
            .child(self.build_info_row("架构".to_string(), info.cpu.arch.clone()))
            .child(self.build_info_row("处理器".to_string(), info.cpu.brand.clone()))
            .child(self.build_info_row("物理核心".to_string(), format!("{} 个", info.cpu.physical_cores)))
            .child(self.build_info_row("频率".to_string(), format!("{} MHz", info.cpu.frequency)))
    }

    fn build_memory_card(
        &self,
        info: &BasicInfo,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .bg(rgb(0x2D2D2D))
            .border_1()
            .border_color(rgb(0x3C3C3C))
            .rounded_lg()
            .p_5()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .mb_4()
                    .child("内存信息"),
            )
            .child(self.build_info_row("已使用".to_string(), self.format_bytes(info.memory.used)))
            .child(self.build_info_row("总内存".to_string(), self.format_bytes(info.memory.total)))
            .child(self.build_info_row("交换分区".to_string(), self.format_bytes(info.memory.total_swap)))
            .child(self.build_info_row(
                "使用率".to_string(),
                format!("{:.1}%", (info.memory.used as f64 / info.memory.total as f64) * 100.0),
            ))
    }

    fn build_os_card(
        &self,
        info: &BasicInfo,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .bg(rgb(0x2D2D2D))
            .border_1()
            .border_color(rgb(0x3C3C3C))
            .rounded_lg()
            .p_5()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .text_color(white())
                    .mb_4()
                    .child("系统信息"),
            )
            .child(self.build_info_row("当前用户".to_string(), info.env.current_user.clone()))
            .child(self.build_info_row("操作系统".to_string(), format!("{} {}", info.os.name, info.os.version)))
            .child(self.build_info_row("内核版本".to_string(), info.os.kernel_version.clone()))
            .child(self.build_info_row("序列号".to_string(), info.os.serial_number.clone()))
    }

    fn build_network_card(
        &self,
        info: &BasicInfo,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .bg(rgb(0x2D2D2D))
            .border_1()
            .border_color(rgb(0x3C3C3C))
            .rounded_lg()
            .p_5()
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
                    .child(self.build_info_row("网卡名称".to_string(), network.name.clone()))
                    .child(self.build_info_row("IP 地址".to_string(), network.ip_address.clone()))
                    .child(self.build_info_row("MAC 地址".to_string(), network.mac_address.clone()))
            } else {
                div().child(self.build_info_row("状态".to_string(), "网络信息获取失败".to_string()))
            })
    }
}

impl Render for HomePage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // 加载系统信息
        self.load_system_info(cx);

        div().w_full().flex().flex_col().child(
            // 内容区域
            div()
                .flex_1()
                .px_6()
                .py_6()
                .child(if let Some(info) = &self.system_info {
                    div()
                        .flex()
                        .flex_col()
                        .gap_5()
                        .child(
                            // 第一行：CPU 和 内存
                            div()
                                .flex()
                                .gap_5()
                                .child(self.build_cpu_card(info))
                                .child(self.build_memory_card(info)),
                        )
                        .child(
                            // 第二行：系统 和 网络
                            div()
                                .flex()
                                .gap_5()
                                .child(self.build_os_card(info))
                                .child(self.build_network_card(info)),
                        )
                } else {
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().text_lg().text_color(rgb(0x808080)).child("正在加载系统信息..."))
                }),
        )
    }
}
