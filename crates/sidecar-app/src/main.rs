use std::{borrow::Cow, collections::HashMap};

use gpui::*;
use gpui_component::{
    Collapsible, Icon, Root, Side, StyledExt, Theme, ThemeMode, init,
    resizable::{ResizableState, h_resizable, resizable_panel},
    scroll::ScrollbarShow,
    sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
};
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

use crate::pages::{
    conv::{base64::Base64Page, timestamp::TimestampPage},
    demo::DemoPage,
    devel::{
        cert::CertPage, crypto::CryptoPage, hash::HashPage, json::JsonPage, qrcode::QrcodePage, random::RandomPage,
    },
    home::HomePage,
    manual::{code::CodePage, custom::CustomPage},
    setting::SettingPage,
    system::{macos::MacosPage, windows::WindowsPage},
    tool::{dns::DnsPage, port::PortPage, share::SharePage},
};

mod comps;
mod pages;

struct View {
    key: &'static str,
    icon: &'static str,
    title: &'static str,
    group: Option<&'static str>,
    build: fn(&mut Window, &mut Context<MainView>) -> AnyView,
}

static VIEWS: Lazy<IndexMap<&'static str, View>> = Lazy::new(|| {
    let mut m: IndexMap<&'static str, View> = IndexMap::new();
    m.insert(
        "/home",
        View {
            key: "/home",
            icon: "icons/home.svg",
            title: "首页",
            group: None,
            build: HomePage::build,
        },
    );
    m.insert(
        "/demo",
        View {
            key: "/demo",
            icon: "icons/apps.svg",
            title: "演示页面",
            group: None,
            build: DemoPage::build,
        },
    );
    m.insert(
        "/conv/base64",
        View {
            key: "/conv/base64",
            icon: "icons/base64.svg",
            title: "Base64",
            group: Some("转换工具"),
            build: Base64Page::build,
        },
    );
    m.insert(
        "/conv/timestamp",
        View {
            key: "/conv/timestamp",
            icon: "icons/timestamp.svg",
            title: "时间戳转换",
            group: Some("转换工具"),
            build: TimestampPage::build,
        },
    );
    m.insert(
        "/devel/cert",
        View {
            key: "/devel/cert",
            icon: "icons/cert.svg",
            title: "证书解析",
            group: Some("开发工具"),
            build: CertPage::build,
        },
    );
    m.insert(
        "/devel/hash",
        View {
            key: "/devel/hash",
            icon: "icons/hash.svg",
            title: "哈希散列",
            group: Some("开发工具"),
            build: HashPage::build,
        },
    );
    m.insert(
        "/devel/crypto",
        View {
            key: "/devel/crypto",
            icon: "icons/crypto.svg",
            title: "加解密工具",
            group: Some("开发工具"),
            build: CryptoPage::build,
        },
    );
    m.insert(
        "/devel/json",
        View {
            key: "/devel/json",
            icon: "icons/json.svg",
            title: "Json格式化",
            group: Some("开发工具"),
            build: JsonPage::build,
        },
    );
    m.insert(
        "/devel/random",
        View {
            key: "/devel/random",
            icon: "icons/random.svg",
            title: "随机数据",
            group: Some("开发工具"),
            build: RandomPage::build,
        },
    );
    m.insert(
        "/devel/qrcode",
        View {
            key: "/devel/qrcode",
            icon: "icons/qrcode.svg",
            title: "二维码",
            group: Some("开发工具"),
            build: QrcodePage::build,
        },
    );
    m.insert(
        "/tool/share",
        View {
            key: "/tool/share",
            icon: "icons/share.svg",
            title: "文件分享",
            group: Some("便捷工具"),
            build: SharePage::build,
        },
    );
    m.insert(
        "/tool/dns",
        View {
            key: "/tool/dns",
            icon: "icons/dns.svg",
            title: "域名查询",
            group: Some("便捷工具"),
            build: DnsPage::build,
        },
    );
    m.insert(
        "/tool/port",
        View {
            key: "/tool/port",
            icon: "icons/port.svg",
            title: "端口占用查询",
            group: Some("便捷工具"),
            build: PortPage::build,
        },
    );
    m.insert(
        "/manual/code",
        View {
            key: "/manual/code",
            icon: "icons/code.svg",
            title: "代码库",
            group: Some("技术手册"),
            build: CodePage::build,
        },
    );
    m.insert(
        "/manual/custom",
        View {
            key: "/manual/custom",
            icon: "icons/manual.svg",
            title: "自定义手册",
            group: Some("技术手册"),
            build: CustomPage::build,
        },
    );
    m.insert(
        "/system/macos",
        View {
            key: "/system/macos",
            icon: "icons/macos.svg",
            title: "macOS配置",
            group: Some("系统配置"),
            build: MacosPage::build,
        },
    );
    m.insert(
        "/system/windows",
        View {
            key: "/system/windows",
            icon: "icons/windows.svg",
            title: "windows配置",
            group: Some("系统配置"),
            build: WindowsPage::build,
        },
    );
    m.insert(
        "/setting",
        View {
            key: "/setting",
            icon: "icons/setting.svg",
            title: "设置",
            group: None,
            build: SettingPage::build,
        },
    );
    m
});

pub struct MainView {
    views: HashMap<&'static str, AnyView>,
    title: SharedString,
    selected: SharedString,
    resizable_state: Entity<ResizableState>,
}

impl MainView {
    fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut views = HashMap::new();
        for view in VIEWS.values() {
            views.insert(view.key, (view.build)(window, cx));
        }

        Self {
            views,
            title: "首页".into(),
            selected: "/home".into(),
            resizable_state: cx.new(|_| ResizableState::default()),
        }
    }

    fn menu(
        &mut self,
        cx: &mut Context<Self>,
        key: &'static str,
    ) -> SidebarMenuItem {
        let (icon, title) = VIEWS.get(key).map(|d| (d.icon, d.title)).unwrap_or_default();

        let mut item = SidebarMenuItem::new(title)
            .active(self.selected.as_str() == key)
            .on_click(cx.listener(move |this, _ev: &ClickEvent, window, _cx| {
                if this.selected.as_str() != key {
                    this.title = title.into();
                    this.selected = key.to_string().into();
                }
                window.refresh();
            }));
        if !icon.is_empty() {
            item = item.icon(Icon::default().path(icon));
        }
        item
    }

    fn sidebar(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        enum Section {
            Menu(SidebarMenu),
            Group(SidebarGroup<SidebarMenu>),
        }

        impl IntoElement for Section {
            type Element = AnyElement;
            fn into_element(self) -> Self::Element {
                match self {
                    Section::Menu(m) => div().child(m).into_any_element(),
                    Section::Group(g) => g.into_any_element(),
                }
            }
        }

        impl Collapsible for Section {
            fn collapsed(
                self,
                collapsed: bool,
            ) -> Self {
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

        {
            let mut groups: Vec<(&'static str, Vec<&'static str>)> = Vec::new();
            for v in VIEWS.values() {
                if let Some(group) = v.group {
                    if let Some((_, keys)) = groups.iter_mut().find(|(g, _)| *g == group) {
                        keys.push(v.key);
                    } else {
                        groups.push((group, vec![v.key]));
                    }
                }
            }

            let mut sidebar = Sidebar::new(Side::Left)
                .w(relative(1.0))
                .header(SidebarHeader::new().child("Sidecar"))
                .child(Section::Menu(SidebarMenu::new().child(self.menu(cx, "/home"))))
                .child(Section::Menu(SidebarMenu::new().child(self.menu(cx, "/demo"))));
            for (name, keys) in groups {
                let mut menus = SidebarMenu::new();
                for key in keys {
                    menus = menus.child(self.menu(cx, key));
                }
                sidebar = sidebar.child(Section::Group(SidebarGroup::new(name).child(menus)));
            }
            sidebar.footer(SidebarMenu::new().child(self.menu(cx, "/setting")))
        }
    }

    fn content(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        self.views
            .get(self.selected.as_str())
            .cloned()
            .unwrap_or_else(|| {
                self.views
                    .get("/home")
                    .cloned()
                    .unwrap_or_else(|| AnyView::from(_cx.new(|_| EmptyView)))
            })
            .into_any_element()
    }
}

impl Render for MainView {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let modal_layer = Root::render_dialog_layer(window, cx);
        let drawer_layer = Root::render_sheet_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .w_full()
            .h_full()
            .child(
                h_resizable("layout")
                    .with_state(&self.resizable_state)
                    .child(
                        resizable_panel()
                            .size(px(240.0))
                            .size_range(px(120.)..px(360.))
                            .child(self.sidebar(window, cx)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .h_full()
                            .min_h_0()
                            .overflow_hidden()
                            .child(
                                div()
                                    .id("header")
                                    .flex()
                                    .items_center()
                                    .border_b_1()
                                    .border_color(rgb(0x404040))
                                    .px_4()
                                    .py_3()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_semibold()
                                            .text_color(white())
                                            .child(self.title.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .id("content")
                                    .flex_1()
                                    .min_w_0()
                                    .min_h_0()
                                    .overflow_y_scroll()
                                    .child(self.content(window, cx)),
                            )
                            .into_any_element(),
                    ),
            )
            .children(modal_layer)
            .children(drawer_layer)
            .children(notification_layer)
    }
}

pub struct EmptyView;

impl Render for EmptyView {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().size_full()
    }
}

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
struct FsAssets;

impl AssetSource for FsAssets {
    fn load(
        &self,
        path: &str,
    ) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(
        &self,
        path: &str,
    ) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

fn main() {
    let app = Application::new().with_assets(FsAssets);
    app.run(move |cx| {
        init(cx);
        cx.activate(true);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let theme = Theme::global_mut(cx);
        theme.mode = ThemeMode::Dark;
        theme.sidebar = rgb(0x202020).into();
        theme.background = rgb(0x282828).into();
        theme.font_size = px(16.);
        theme.scrollbar_show = ScrollbarShow::Hover;

        let window_size = size(px(1280.), px(800.));
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
                    width: window_size.width,
                    height: window_size.height,
                }),
                kind: WindowKind::Normal,
                #[cfg(target_os = "linux")]
                window_decorations: Some(WindowDecorations::Server),
                ..Default::default()
            };

            let window = cx
                .open_window(options, |window, cx| {
                    let view = cx.new(|cx| MainView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("failed to open window");
            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                })
                .expect("failed to update window");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
