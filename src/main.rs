use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::read;
use std::path::PathBuf;

use gpui::*;
use gpui_component::Collapsible;
use gpui_component::Icon;
use gpui_component::Root;
use gpui_component::StyledExt;
use gpui_component::Theme;
use gpui_component::init;
use gpui_component::resizable::ResizableState;
use gpui_component::resizable::h_resizable;
use gpui_component::resizable::resizable_panel;
use gpui_component::sidebar::Sidebar;
use gpui_component::sidebar::SidebarGroup;
use gpui_component::sidebar::SidebarHeader;
use gpui_component::sidebar::SidebarMenu;
use gpui_component::sidebar::SidebarMenuItem;
use indexmap::IndexMap;
use once_cell::sync::Lazy;

use crate::pages::convert::base64::Base64Page;
use crate::pages::convert::timestamp::TimestampPage;
use crate::pages::demo::DemoPage;
use crate::pages::develop::cert::CertPage;
use crate::pages::develop::crypto::CryptoPage;
use crate::pages::develop::hash::HashPage;
use crate::pages::develop::qrcode::QrcodePage;
use crate::pages::develop::random::RandomPage;
use crate::pages::home::HomePage;
use crate::pages::network::dns::DnsPage;
use crate::pages::network::port::PortPage;
use crate::pages::setting::SettingPage;
use crate::pages::snippet::code::CodePage;
use crate::pages::snippet::manual::ManualPage;
use crate::pages::toolkit::share::SharePage;

mod pages;
mod plugins;

pub const SIDE_BG: u32 = 0x202020;
pub const SIDE_MIN: f32 = 120.;
pub const SIDE_MAX: f32 = 360.;
pub const SIDE_SIZE: f32 = 240.;
pub const HEAD_BORDER: u32 = 0x404040;
pub const HEAD_PADDING_X: f32 = 20.0;
pub const HEAD_PADDING_Y: f32 = 12.0;
pub const PAGE_BG: u32 = 0x282828;
pub const PAGE_GAP: f32 = 20.0;
pub const PAGE_PADDING: f32 = 20.0;
pub const CARD_BG: u32 = 0x333333;
pub const CARD_GAP: f32 = 12.0;
pub const CARD_PADDING: f32 = 20.0;
pub const INPUT_BG: u32 = 0x242424;
pub const INPUT_BORDER: u32 = 0x404040;
pub const INPUT_PADDING: f32 = 4.0;
pub const COMMON_GAP: f32 = 20.0;
pub const COMMON_PADDING: f32 = 20.0;
pub const COMMON_PADDING_M: f32 = 12.0;

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
        "/toolkit/share",
        View {
            key: "/toolkit/share",
            icon: "icons/share.svg",
            title: "文件分享",
            group: Some("便捷工具"),
            build: SharePage::build,
        },
    );
    m.insert(
        "/convert/base64",
        View {
            key: "/convert/base64",
            icon: "icons/base64.svg",
            title: "Base64",
            group: Some("转换工具"),
            build: Base64Page::build,
        },
    );
    m.insert(
        "/convert/timestamp",
        View {
            key: "/convert/timestamp",
            icon: "icons/timestamp.svg",
            title: "时间戳转换",
            group: Some("转换工具"),
            build: TimestampPage::build,
        },
    );
    m.insert(
        "/develop/cert",
        View {
            key: "/develop/cert",
            icon: "icons/cert.svg",
            title: "证书解析",
            group: Some("开发工具"),
            build: CertPage::build,
        },
    );
    m.insert(
        "/develop/hash",
        View {
            key: "/develop/hash",
            icon: "icons/hash.svg",
            title: "哈希散列",
            group: Some("开发工具"),
            build: HashPage::build,
        },
    );
    m.insert(
        "/develop/crypto",
        View {
            key: "/develop/crypto",
            icon: "icons/crypto.svg",
            title: "加解密工具",
            group: Some("开发工具"),
            build: CryptoPage::build,
        },
    );
    m.insert(
        "/develop/random",
        View {
            key: "/develop/random",
            icon: "icons/random.svg",
            title: "随机数据",
            group: Some("开发工具"),
            build: RandomPage::build,
        },
    );
    m.insert(
        "/develop/qrcode",
        View {
            key: "/develop/qrcode",
            icon: "icons/qrcode.svg",
            title: "二维码",
            group: Some("开发工具"),
            build: QrcodePage::build,
        },
    );
    m.insert(
        "/network/dns",
        View {
            key: "/network/dns",
            icon: "icons/dns.svg",
            title: "域名查询",
            group: Some("网络工具"),
            build: DnsPage::build,
        },
    );
    m.insert(
        "/network/port",
        View {
            key: "/network/port",
            icon: "icons/port.svg",
            title: "端口占用查询",
            group: Some("网络工具"),
            build: PortPage::build,
        },
    );
    m.insert(
        "/snippet/code",
        View {
            key: "/snippet/code",
            icon: "icons/code.svg",
            title: "代码库",
            group: Some("代码片段"),
            build: CodePage::build,
        },
    );
    m.insert(
        "/snippet/manual",
        View {
            key: "/snippet/manual",
            icon: "icons/manual.svg",
            title: "命令手册",
            group: Some("代码片段"),
            build: ManualPage::build,
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
            resizable_state: ResizableState::new(cx),
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
                    Section::Menu(m) => div().p_2().child(m).into_any_element(),
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

            let mut sidebar = Sidebar::left()
                .width(relative(1.0))
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
        div().size_full().bg(rgb(PAGE_BG)).child(
            h_resizable("layout", self.resizable_state.clone())
                .child(
                    resizable_panel()
                        .size(px(SIDE_SIZE))
                        .size_range(px(SIDE_MIN)..px(SIDE_MAX))
                        .child(self.sidebar(window, cx)),
                )
                .child(
                    div()
                        .flex_1()
                        .v_flex()
                        .h_full()
                        .min_h_0()
                        .overflow_hidden()
                        .child(
                            div()
                                .id("head")
                                .flex()
                                .items_center()
                                .border_b_1()
                                .border_color(rgb(HEAD_BORDER))
                                .px(px(HEAD_PADDING_X))
                                .py(px(HEAD_PADDING_Y))
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
                                .min_h_0()
                                .overflow_y_scroll()
                                .child(self.content(window, cx)),
                        )
                        .into_any_element(),
                ),
        )
    }
}

pub struct EmptyView;

impl Render for EmptyView {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().size_full().bg(rgb(PAGE_BG))
    }
}

pub struct FsAssets;

impl AssetSource for FsAssets {
    fn load(
        &self,
        path: &str,
    ) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let full = manifest_dir.join("assets").join(path);

        match read(full) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(_) => Ok(None),
        }
    }

    fn list(
        &self,
        _path: &str,
    ) -> Result<Vec<SharedString>> {
        Ok(Vec::new())
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

        let window_size = size(px(1280.), px(800.));
        let window_bounds = Bounds::centered(None, window_size, cx);
        Theme::global_mut(cx).sidebar = rgb(SIDE_BG).into();

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
                    cx.new(|cx| Root::new(view.into(), window, cx))
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
