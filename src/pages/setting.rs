use crate::MainView;
use gpui::*;

pub struct SettingPage;

impl SettingPage {
    pub fn build(
        _window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|_| SettingPage))
    }
}

impl Render for SettingPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().size_full()
    }
}
