use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dark
    }
}

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub sidebar: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_accent_foreground: Hsla,
    pub radius: Pixels,
}

impl ThemePalette {
    pub fn light() -> Self {
        Self {
            sidebar: color(0xf6f6f7),                   // 侧边栏背景
            sidebar_foreground: color(0x1f1f23),        // 侧边栏前景
            sidebar_border: color(0xe0e0e5),            // 侧边栏边框
            sidebar_accent: color(0xd9e6ff),            // 侧边栏强调色
            sidebar_accent_foreground: color(0x101019), // 强调前景
            radius: px(8.),
        }
    }

    pub fn dark() -> Self {
        Self {
            sidebar: color(0x202020),                   // 侧边栏背景
            sidebar_foreground: color(0xe6e6e6),        // 侧边栏前景
            sidebar_border: color(0x404040),            // 侧边栏边框
            sidebar_accent: color(0x353535),            // 侧边栏强调色
            sidebar_accent_foreground: color(0xffffff), // 强调前景
            radius: px(8.),
        }
    }
}

#[derive(Debug, Clone)]
struct ThemeStore {
    light: ThemePalette,
    dark: ThemePalette,
    mode: ThemeMode,
}

impl ThemeStore {
    fn new(
        light: ThemePalette,
        dark: ThemePalette,
        mode: ThemeMode,
    ) -> Self {
        Self { light, dark, mode }
    }

    fn current(&self) -> &ThemePalette {
        match self.mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }

    fn set_mode(
        &mut self,
        mode: ThemeMode,
    ) {
        self.mode = mode;
    }

    fn ensure_global(
        cx: &mut App,
        mode: ThemeMode,
    ) {
        if !cx.has_global::<ThemeStore>() {
            let mut store = ThemeStore::new(ThemePalette::light(), ThemePalette::dark(), ThemeMode::Light);
            store.set_mode(mode);
            cx.set_global(store);
        } else {
            ThemeStore::global_mut(cx).set_mode(mode);
        }
    }

    fn global(cx: &App) -> &Self {
        cx.global::<ThemeStore>()
    }

    fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<ThemeStore>()
    }
}

impl Global for ThemeStore {}

pub trait ThemeAccess {
    fn theme(&self) -> &ThemePalette;
}

impl ThemeAccess for App {
    fn theme(&self) -> &ThemePalette {
        ThemeStore::global(self).current()
    }
}

pub fn init_theme(
    cx: &mut App,
    mode: ThemeMode,
) {
    ThemeStore::ensure_global(cx, mode);
}

fn color(hex: u32) -> Hsla {
    rgb(hex).into()
}
