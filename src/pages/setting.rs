use gpui::*;

pub struct SettingPage;

impl Render for SettingPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}

