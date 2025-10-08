use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dark
    }
}

#[derive(Debug, Clone, Copy)]
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
            sidebar: rgb(0xf6f6f7).into(),                   // 侧边栏背景
            sidebar_foreground: rgb(0x1f1f23).into(),        // 侧边栏前景
            sidebar_border: rgb(0xe0e0e5).into(),            // 侧边栏边框
            sidebar_accent: rgb(0xd9e6ff).into(),            // 侧边栏强调色
            sidebar_accent_foreground: rgb(0x101019).into(), // 强调前景
            radius: px(8.),
        }
    }

    pub fn dark() -> Self {
        Self {
            sidebar: rgb(0x202020).into(),                   // 侧边栏背景
            sidebar_foreground: rgb(0xe6e6e6).into(),        // 侧边栏前景
            sidebar_border: rgb(0x404040).into(),            // 侧边栏边框
            sidebar_accent: rgb(0x353535).into(),            // 侧边栏强调色
            sidebar_accent_foreground: rgb(0xffffff).into(), // 强调前景
            radius: px(8.),
        }
    }
}

#[derive(Debug, Clone)]
struct ThemeStore {
    mode: ThemeMode,
    dark: ThemePalette,
    light: ThemePalette,
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
            ThemeMode::Dark => &self.dark,
            ThemeMode::Light => &self.light,
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
            cx.set_global(ThemeStore::new(ThemePalette::light(), ThemePalette::dark(), mode));
        } else {
            cx.global_mut::<ThemeStore>().set_mode(mode);
        }
    }
}

impl Global for ThemeStore {}

pub trait ActiveTheme {
    fn theme(&self) -> &ThemePalette;
}

impl ActiveTheme for App {
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
