use gpui::*;

use crate::comps::Collapsible;
use crate::comps::StyledExt;
use crate::comps::ActiveTheme;

pub struct SidebarGroup<E: Collapsible + IntoElement + 'static> {
    label: SharedString,
    collapsed: bool,
    children: Vec<E>,
}

impl<E: Collapsible + IntoElement + 'static> SidebarGroup<E> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            collapsed: false,
            children: Vec::new(),
        }
    }

    pub fn child(
        mut self,
        child: E,
    ) -> Self {
        self.children.push(child);
        self
    }

    #[allow(dead_code)]
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children.extend(children);
        self
    }
}

impl<E: Collapsible + IntoElement + 'static> Collapsible for SidebarGroup<E> {
    fn collapsed(
        mut self,
        collapsed: bool,
    ) -> Self {
        self.collapsed = collapsed;
        self
    }

    fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl<E: Collapsible + IntoElement + 'static> RenderOnce for SidebarGroup<E> {
    fn render(
        self,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let label = self.label.clone();
        let items = self.children.into_iter().enumerate().map(|(ix, child)| {
            div()
                .id(ElementId::Integer(ix as u64))
                .child(child.collapsed(self.collapsed))
        });

        let mut block = div().v_flex().gap(px(8.)).px(px(12.)).pb(px(8.));

        if !self.collapsed {
            block = block.child(
                div()
                    .text_xs()
                    .text_color(theme.sidebar_foreground.opacity(0.7))
                    .child(label),
            );
        }

        block.children(items)
    }
}

impl<E: Collapsible + IntoElement + 'static> IntoElement for SidebarGroup<E> {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
