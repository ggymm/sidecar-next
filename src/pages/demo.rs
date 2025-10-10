use gpui::*;
use gpui_component::StyledExt;
use gpui_component::input::InputEvent;
use gpui_component::input::InputState;

use crate::MainView;
use crate::comps::card;
use crate::comps::label;
use crate::comps::page;
use crate::comps::textarea;

pub struct DemoPage {
    input1: Entity<InputState>,
    input2: Entity<InputState>,
    _subs: Vec<Subscription>,
}

impl DemoPage {
    pub fn build(
        window: &mut Window,
        cx: &mut Context<MainView>,
    ) -> AnyView {
        AnyView::from(cx.new(|cx| {
            let input1 = cx.new(|cx| InputState::new(window, cx).default_value("测试文本"));
            let input2 = cx.new(|cx| InputState::new(window, cx).placeholder("Another input"));

            let subs = vec![
                cx.subscribe_in(&input1, window, Self::on_input_event),
                cx.subscribe_in(&input2, window, Self::on_input_event),
            ];

            Self {
                input1,
                input2,
                _subs: subs,
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
            InputEvent::Focus => println!("[Demo] Focus"),
            InputEvent::Blur => println!("[Demo] Blur"),
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
                        .child(textarea(&self.input1, |input| input.cleanable()).overflow_hidden()),
                )
                .child(
                    card()
                        .h_64()
                        .child(label("输入框测试 2"))
                        .child(textarea(&self.input2, |input| input)),
                ),
        )
    }
}
