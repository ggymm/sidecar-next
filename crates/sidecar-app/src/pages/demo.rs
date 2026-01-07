use gpui::*;
use gpui_component::{
    StyledExt,
    input::{InputEvent, InputState, TabSize},
};

use crate::{
    MainView,
    comps::{card, label, page, textarea},
};

pub struct DemoPage {
    subs: Vec<Subscription>,
    test_focus: Entity<InputState>,
    code_editor: Entity<InputState>,
}

impl DemoPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let test_focus = cx.new(|cx| InputState::new(window, cx).default_value("测试文本"));
            let code_editor = cx.new(|cx| {
                InputState::new(window, cx)
                    .code_editor("text")
                    .tab_size(TabSize {
                        tab_size: 2,
                        hard_tabs: false,
                    })
                    .line_number(true)
                    .soft_wrap(false)
            });

            let subs = vec![cx.subscribe_in(&test_focus, window, Self::on_input_event)];

            Self {
                subs,
                test_focus,
                code_editor,
            }
        }))
    }

    pub fn on_input_event(
        &mut self,
        _state: &Entity<InputState>,
        ev: &InputEvent,
        _w: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match ev {
            InputEvent::Blur => println!("[Demo] Blur"),
            InputEvent::Focus => println!("[Demo] Focus"),
            InputEvent::Change => println!("[Demo] Change"),
            InputEvent::PressEnter { secondary } => println!("[Demo] Enter secondary={}", secondary),
        }
    }
}

impl Render for DemoPage {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        page().child(
            div()
                .v_flex()
                .gap_4()
                .flex_1()
                .child(
                    card()
                        .h_64()
                        .child(label("输入框测试 1"))
                        .child(textarea(&self.test_focus, |input| input)),
                )
                .child(
                    card()
                        .h_64()
                        .w_full()
                        .child(label("gpui code_editor 最小示例（无高亮，仅测试粘贴性能）"))
                        .child(textarea(&self.code_editor, |input| input)),
                ),
        )
    }
}
