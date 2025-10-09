use gpui::*;
use gpui_component::StyledExt;

pub const PAGE_BG: u32 = 0x282828;
pub const PAGE_GAP: f32 = 20.0;
pub const PAGE_PADDING: f32 = 20.0;

pub struct Page {
    children: Vec<AnyElement>,
}

impl Page {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub fn child(
        mut self,
        child: impl IntoElement,
    ) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl RenderOnce for Page {
    fn render(
        self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let container = div()
            .v_flex()
            .w_full()
            .h_full()
            .bg(rgb(PAGE_BG))
            .paddings(Edges::all(px(PAGE_PADDING)))
            .gap(px(PAGE_GAP));

        container.children(self.children)
    }
}

impl IntoElement for Page {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
