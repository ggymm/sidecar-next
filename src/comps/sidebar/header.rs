use gpui::*;

use crate::comps::StyledExt;
use crate::comps::ThemeAccess;

pub struct SidebarHeader {
    children: Vec<AnyElement>,
    selected: bool,
    collapsed: bool,
}

impl SidebarHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            selected: false,
            collapsed: false,
        }
    }

    pub fn child(
        mut self,
        child: impl IntoElement,
    ) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    #[allow(dead_code)]
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl IntoElement>,
    ) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    #[allow(dead_code)]
    pub fn selected(
        mut self,
        selected: bool,
    ) -> Self {
        self.selected = selected;
        self
    }

    #[allow(dead_code)]
    pub fn collapsed(
        mut self,
        collapsed: bool,
    ) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarHeader {
    fn render(
        self,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let content = div().h_flex().gap(px(8.)).children(self.children.into_iter());

        let mut header = div()
            .h_flex()
            .gap(px(8.))
            .px(px(12.))
            .py(px(10.))
            .rounded(cx.theme().radius)
            .cursor_pointer()
            .hover(|style| {
                style
                    .bg(cx.theme().sidebar_accent)
                    .text_color(cx.theme().sidebar_accent_foreground)
            });

        header = header.child(content);

        if self.selected {
            header = header
                .bg(cx.theme().sidebar_accent)
                .text_color(cx.theme().sidebar_accent_foreground);
        }

        if self.collapsed {
            header = header.justify_center();
        }

        header
    }
}

impl IntoElement for SidebarHeader {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
