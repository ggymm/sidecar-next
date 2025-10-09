use gpui::*;
use gpui_component::StyledExt;
use gpui_component::input::InputEvent;
use gpui_component::input::InputState;
use gpui_component::input::TextInput;


use crate::INPUT_BG;
use crate::INPUT_BORDER;
use crate::INPUT_PADDING;
use crate::MainView;
use crate::PAGE_GAP;
use crate::PAGE_PADDING;
use crate::comps::Card;

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

impl Focusable for DemoPage {
    fn focus_handle(
        &self,
        cx: &App,
    ) -> FocusHandle {
        self.input1.focus_handle(cx)
    }
}

impl Render for DemoPage {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let input_bg = rgb(INPUT_BG);
        let page_padding = Edges::all(px(PAGE_PADDING));
        let input_padding = Edges::all(px(INPUT_PADDING));

        div()
            .id("demo")
            .w_full()
            .paddings(page_padding)
            .gap(px(PAGE_GAP))
            .v_flex()
            .track_focus(&self.focus_handle(cx))
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .flex_1()
                    .child(
                        Card::new()
                            .height(px(240.))
                            .child(div().text_sm().text_color(white()).child("输入框测试 1"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_lg()
                                    .overflow_hidden()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.input1)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .cleanable(),
                                    ),
                            ),
                    )
                    .child(
                        Card::new()
                            .height(px(240.))
                            .child(div().text_sm().text_color(white()).child("输入框测试 2"))
                            .child(
                                div()
                                    .flex_1()
                                    .bg(input_bg)
                                    .border_1()
                                    .border_color(rgb(INPUT_BORDER))
                                    .rounded_lg()
                                    .paddings(input_padding)
                                    .child(
                                        TextInput::new(&self.input2)
                                            .appearance(false)
                                            .focus_bordered(false)
                                            .text_color(white())
                                            .h_full(),
                                    ),
                            ),
                    ),
            )
    }
}
