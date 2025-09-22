mod assets;
mod page;

use gpui::*;
use gpui_component::{
    resizable::{h_resizable, resizable_panel, ResizableState},
    sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
    Icon, Root, Theme,
};

pub struct MainView {
    selected: SharedString,
    resizable_state: Entity<ResizableState>,
}

impl MainView {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let resizable_state = ResizableState::new(cx);
        Self {
            selected: "/home".into(),
            resizable_state,
        }
    }

    fn menu_item(&mut self, cx: &mut Context<Self>, label: &'static str, path: &'static str) -> SidebarMenuItem {
        let seg = path.trim_start_matches('/').split('/').last().unwrap_or("");
        let icon = Icon::default().path(format!("icons/{seg}.svg"));
        SidebarMenuItem::new(label)
            .icon(icon)
            .active(self.selected.as_str() == path)
            .on_click(cx.listener(move |this, _ev: &ClickEvent, window, _cx| {
                this.select(path);
                window.refresh();
            }))
    }

    fn menu_group(
        &mut self,
        cx: &mut Context<Self>,
        group_label: &'static str,
        items: &[(&'static str, &'static str)],
    ) -> SidebarGroup<SidebarMenu> {
        let mut menu = SidebarMenu::new();
        for (label, path) in items {
            menu = menu.child(self.menu_item(cx, *label, *path));
        }
        SidebarGroup::new(group_label).child(menu)
    }

    // no main_menu; home is placed under header, other sections keep their group titles

    // 图标路径与页面路径保持一致：取最后一段作为图标文件名（icons/{last}.svg）

    fn select(&mut self, path: &str) {
        if self.selected.as_str() != path {
            self.selected = path.to_string().into();
        }
    }

    fn sidebar(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Local wrapper to unify Sidebar children type
        enum Section {
            Menu(SidebarMenu),
            Group(SidebarGroup<SidebarMenu>),
        }

        impl gpui_component::Collapsible for Section {
            fn collapsed(self, collapsed: bool) -> Self {
                match self {
                    Section::Menu(m) => Section::Menu(m.collapsed(collapsed)),
                    Section::Group(g) => Section::Group(g.collapsed(collapsed)),
                }
            }
            fn is_collapsed(&self) -> bool {
                match self {
                    Section::Menu(m) => m.is_collapsed(),
                    Section::Group(g) => g.is_collapsed(),
                }
            }
        }

        impl IntoElement for Section {
            type Element = AnyElement;
            fn into_element(self) -> Self::Element {
                match self {
                    // 给单独的菜单加上与分组一致的左内边距
                    Section::Menu(m) => div().p_2().child(m).into_any_element(),
                    Section::Group(g) => g.into_any_element(),
                }
            }
        }

        let home = Section::Menu(SidebarMenu::new().child(self.menu_item(cx, "首页", "/home")));
        let tools = Section::Group(self.menu_group(cx, "便捷工具", &[("文件分享", "/toolkit/share")]));
        let convert = Section::Group(self.menu_group(
            cx,
            "转换工具",
            &[("Base64", "/convert/base64"), ("时间戳转换", "/convert/timestamp")],
        ));
        let develop = Section::Group(self.menu_group(
            cx,
            "开发工具",
            &[
                ("证书解析", "/develop/cert"),
                ("哈希散列", "/develop/hash"),
                ("加解密工具", "/develop/crypto"),
                ("随机数据", "/develop/random"),
                ("二维码", "/develop/qrcode"),
            ],
        ));
        let network = Section::Group(self.menu_group(
            cx,
            "网络工具",
            &[("域名查询", "/network/dns"), ("端口占用查询", "/network/port")],
        ));
        let snippet = Section::Group(self.menu_group(
            cx,
            "代码片段",
            &[("代码库", "/snippet/code"), ("命令手册", "/snippet/manual")],
        ));

        Sidebar::left()
            .width(relative(1.0))
            .header(SidebarHeader::new().child("Sidecar"))
            .child(home)
            .child(tools)
            .child(convert)
            .child(develop)
            .child(network)
            .child(snippet)
            .footer(SidebarMenu::new().child(self.menu_item(cx, "设置", "/setting")))
    }

    fn content(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.selected.as_str() {
            "/home" => cx.new(|_| page::home::HomePage).into_any_element(),
            "/toolkit/share" => cx.new(|_| page::toolkit::share::SharePage).into_any_element(),
            "/convert/base64" => cx.new(|_| page::convert::base64::Base64Page).into_any_element(),
            "/convert/timestamp" => cx.new(|_| page::convert::timestamp::TimestampPage).into_any_element(),
            "/develop/cert" => cx.new(|_| page::develop::cert::CertPage).into_any_element(),
            "/develop/hash" => cx.new(|_| page::develop::hash::HashPage).into_any_element(),
            "/develop/crypto" => cx.new(|_| page::develop::crypto::CryptoPage).into_any_element(),
            "/develop/random" => cx.new(|_| page::develop::random::RandomPage).into_any_element(),
            "/develop/qrcode" => cx.new(|_| page::develop::qrcode::QrcodePage).into_any_element(),
            "/network/dns" => cx.new(|_| page::network::dns::DnsPage).into_any_element(),
            "/network/port" => cx.new(|_| page::network::port::PortPage).into_any_element(),
            "/snippet/code" => cx.new(|_| page::snippet::code::CodePage).into_any_element(),
            "/snippet/manual" => cx.new(|_| page::snippet::manual::ManualPage).into_any_element(),
            "/setting" => cx.new(|_| page::setting::SettingPage).into_any_element(),
            _ => div().size_full().into_any_element(),
        }
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .child(
                h_resizable("layout", self.resizable_state.clone())
                    .child(
                        resizable_panel()
                            .size(px(240.))
                            .size_range(px(180.)..px(360.))
                            .child(self.sidebar(window, cx)),
                    )
                    .child(resizable_panel().child(self.content(window, cx))),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(assets::FsAssets);

    app.run(move |cx| {
        gpui_component::init(cx);
        cx.activate(true);

        // Adjust sidebar background color to #202020
        Theme::global_mut(cx).sidebar = rgb(0x202020).into();

        // 当最后一个窗口关闭时退出应用
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let mut window_size = size(px(960.), px(640.));

        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        let window_bounds = Bounds::centered(None, window_size, cx);

        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                window_min_size: Some(Size {
                    width: px(800.),
                    height: px(560.),
                }),
                kind: WindowKind::Normal,
                #[cfg(target_os = "linux")]
                window_decorations: Some(gpui::WindowDecorations::Server),
                ..Default::default()
            };

            let window = cx
                .open_window(options, |window, cx| {
                    let view = cx.new(|cx| MainView::new(window, cx));
                    cx.new(|cx| Root::new(view.into(), window, cx))
                })
                .expect("failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                    window.set_window_title("Sidecar");
                })
                .expect("failed to update window");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
