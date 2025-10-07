use gpui::{
    div, px, App, Component, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};
use gpui_component::ActiveTheme as _;
use std::rc::Rc;

/// Simplified radio group component covering the behaviours used in the app.
pub struct RadioGroup {
    options: Vec<SharedString>,
    style: StyleRefinement,
    horizontal: bool,
    disabled: bool,
    selected_index: Option<usize>,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl RadioGroup {
    fn new(id: impl Into<ElementId>) -> Self {
        let _ = id.into();
        Self {
            options: Vec::new(),
            style: StyleRefinement::default(),
            horizontal: false,
            disabled: false,
            selected_index: None,
            on_change: None,
        }
    }

    pub fn horizontal(id: impl Into<ElementId>) -> Self {
        Self::new(id).layout_horizontal()
    }

    pub fn layout_horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn child(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push(label.into());
        self
    }

}

impl Styled for RadioGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut container = div();
        *container.style() = self.style;

        let layout = if self.horizontal {
            div().flex().gap(px(12.)).flex_wrap()
        } else {
            div().flex().flex_col().gap(px(8.))
        };

        container.child(layout.children(self.options.into_iter().enumerate().map(|(ix, option)| {
            let is_selected = self.selected_index == Some(ix);
            let disabled = self.disabled;

            let indicator_inner = div()
                .size(px(8.))
                .rounded_full()
                .bg(theme.primary_foreground);

            let mut indicator = div()
                .size(px(16.))
                .rounded_full()
                .border_2()
                .border_color(theme.border)
                .bg(theme.background);

            if is_selected {
                indicator = indicator
                    .border_color(theme.primary)
                    .bg(theme.primary)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(indicator_inner);
            }

            let mut row = div()
                .flex()
                .items_center()
                .gap(px(10.))
                .px(px(10.))
                .py(px(6.))
                .rounded(theme.radius)
                .border_1()
                .border_color(theme.border)
                .text_sm()
                .text_color(theme.foreground)
                .child(indicator)
                .child(option.clone());

            if disabled {
                row = row.opacity(0.5).cursor_default();
            } else {
                row = row.cursor_pointer();
                if let Some(on_change) = self.on_change.clone() {
                    row = row.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        on_change(&ix, window, cx);
                    });
                }
            }

            if is_selected {
                row = row.bg(theme.primary.opacity(0.12)).border_color(theme.primary);
            }

            row
        })))
    }
}

impl IntoElement for RadioGroup {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
