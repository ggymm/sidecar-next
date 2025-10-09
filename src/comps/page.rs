use gpui::*;
use gpui_component::StyledExt;

pub const PAGE_BG: u32 = 0x282828;
pub const PAGE_GAP: f32 = 20.0;
pub const PAGE_PADDING: f32 = 20.0;

pub struct Page {
    w_full: bool,
    h_full: bool,
    children: Vec<AnyElement>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            w_full: true,
            h_full: false,
            children: Vec::new(),
        }
    }

    pub fn w_full(mut self) -> Self {
        self.w_full = true;
        self
    }

    pub fn h_full(mut self) -> Self {
        self.h_full = true;
        self
    }

    pub fn size_full(mut self) -> Self {
        self.w_full = true;
        self.h_full = true;
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

impl RenderOnce for Page {
    fn render(
        self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let mut container = div()
            .v_flex()
            .bg(rgb(PAGE_BG))
            .paddings(Edges::all(px(PAGE_PADDING)))
            .gap(px(PAGE_GAP));

        container = if self.w_full { container.w_full() } else { container };
        container = if self.h_full { container.h_full() } else { container };

        container.children(self.children)
    }
}

impl IntoElement for Page {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
