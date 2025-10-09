use gpui::*;
use gpui_component::StyledExt;

pub const CARD_BG: u32 = 0x333333;
pub const CARD_GAP: f32 = 12.0;
pub const CARD_PADDING: f32 = 20.0;

pub struct Card {
    height: Option<DefiniteLength>,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            height: None,
            children: Vec::new(),
        }
    }

    pub fn height(
        mut self,
        height: impl Into<DefiniteLength>,
    ) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn child(
        mut self,
        child: impl IntoElement,
    ) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for Card {
    fn render(
        self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let mut container = div()
            .flex()
            .flex_col()
            .min_w_0()
            .min_h_0()
            .bg(rgb(CARD_BG))
            .rounded_lg()
            .paddings(Edges::all(px(CARD_PADDING)))
            .gap(px(CARD_GAP));

        container = match self.height {
            Some(length) => container.h(length),
            None => container.flex_1(),
        };

        container.children(self.children)
    }
}

impl IntoElement for Card {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
