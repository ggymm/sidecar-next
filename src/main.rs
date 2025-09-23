use gpui::*;
use gpui_component::init;
use gpui_component::resizable::h_resizable;
use gpui_component::resizable::resizable_panel;
use gpui_component::resizable::ResizableState;
use gpui_component::sidebar::Sidebar;
use gpui_component::sidebar::SidebarGroup;
use gpui_component::sidebar::SidebarHeader;
use gpui_component::sidebar::SidebarMenu;
use gpui_component::sidebar::SidebarMenuItem;
use gpui_component::Collapsible;
use gpui_component::Icon;
use gpui_component::Root;
use gpui_component::Theme;
use std::borrow::Cow;
use std::fs::read;
use std::path::PathBuf;

use crate::pages::convert::base64::Base64Page;
use crate::pages::convert::timestamp::TimestampPage;
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

pub const COLOR_SIDEBAR_BG: u32 = 0x202020; 
pub const COLOR_CONTENT_BG: u32 = 0x282828;

pub struct MainView {
    selected: SharedString,
    resizable_state: Entity<ResizableState>,
    current_view: Option<AnyView>,
}

impl MainView {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let resizable_state = ResizableState::new(cx);
        Self {
            selected: "/home".into(),
            resizable_state,
            current_view: None,
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
        label: &'static str,
        items: &[(&'static str, &'static str)],
    ) -> SidebarGroup<SidebarMenu> {
        let mut menu = SidebarMenu::new();
        for (label, path) in items {
            menu = menu.child(self.menu_item(cx, *label, *path));
        }
        SidebarGroup::new(label).child(menu)
    }

    fn select(&mut self, path: &str) {
        if self.selected.as_str() != path {
            self.selected = path.to_string().into();
            self.current_view = None;
        }
    }

    fn sidebar(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        enum Section {
            Menu(SidebarMenu),
            Group(SidebarGroup<SidebarMenu>),
        }

        impl Collapsible for Section {
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
                    Section::Menu(m) => div().p_2().child(m).into_any_element(),
                    Section::Group(g) => g.into_any_element(),
                }
            }
        }

        Sidebar::left()
            .width(relative(1.0))
            .header(SidebarHeader::new().child("Sidecar"))
            .child(Section::Menu(
                SidebarMenu::new().child(self.menu_item(cx, "首页", "/home")),
            ))
            .child(Section::Group(self.menu_group(
                cx,
                "便捷工具",
                &[("文件分享", "/toolkit/share")],
            )))
            .child(Section::Group(self.menu_group(
                cx,
                "转换工具",
                &[("Base64", "/convert/base64"), ("时间戳转换", "/convert/timestamp")],
            )))
            .child(Section::Group(self.menu_group(
                cx,
                "开发工具",
                &[
                    ("证书解析", "/develop/cert"),
                    ("哈希散列", "/develop/hash"),
                    ("加解密工具", "/develop/crypto"),
                    ("随机数据", "/develop/random"),
                    ("二维码", "/develop/qrcode"),
                ],
            )))
            .child(Section::Group(self.menu_group(
                cx,
                "网络工具",
                &[("域名查询", "/network/dns"), ("端口占用查询", "/network/port")],
            )))
            .child(Section::Group(self.menu_group(
                cx,
                "代码片段",
                &[("代码库", "/snippet/code"), ("命令手册", "/snippet/manual")],
            )))
            .footer(SidebarMenu::new().child(self.menu_item(cx, "设置", "/setting")))
    }

    fn content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        if self.current_view.is_none() {
            let view = match self.selected.as_str() {
                "/home" => AnyView::from(cx.new(|_| HomePage::new())),
                "/toolkit/share" => AnyView::from(cx.new(|_| SharePage)),
                "/convert/base64" => AnyView::from(cx.new(|cx| Base64Page::new(window, cx))),
                "/convert/timestamp" => AnyView::from(cx.new(|_| TimestampPage)),
                "/develop/cert" => AnyView::from(cx.new(|_| CertPage)),
                "/develop/hash" => AnyView::from(cx.new(|_| HashPage)),
                "/develop/crypto" => AnyView::from(cx.new(|_| CryptoPage)),
                "/develop/random" => AnyView::from(cx.new(|_| RandomPage)),
                "/develop/qrcode" => AnyView::from(cx.new(|_| QrcodePage)),
                "/network/dns" => AnyView::from(cx.new(|_| DnsPage)),
                "/network/port" => AnyView::from(cx.new(|_| PortPage)),
                "/snippet/code" => AnyView::from(cx.new(|_| CodePage)),
                "/snippet/manual" => AnyView::from(cx.new(|_| ManualPage)),
                "/setting" => AnyView::from(cx.new(|_| SettingPage)),
                _ => AnyView::from(cx.new(|_| EmptyView)),
            };
            self.current_view = Some(view);
        }
        self.current_view.clone().unwrap().into_any_element()
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(COLOR_CONTENT_BG)).child(
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

pub struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(COLOR_CONTENT_BG))
    }
}

pub struct FsAssets;

impl AssetSource for FsAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
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

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
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
        Theme::global_mut(cx).sidebar = rgb(COLOR_SIDEBAR_BG).into();

        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                window_min_size: Some(Size {
                    width: px(1280.),
                    height: px(800.),
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
