use crate::comps::{Collapsible, Icon, StyledExt};
use gpui::{
    div, px, AnyElement, App, ClickEvent, Component, DefiniteLength, ElementId, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window,
};
use gpui_component::ActiveTheme as _;
use std::rc::Rc;

const DEFAULT_WIDTH: f32 = 255.0;
const COLLAPSED_WIDTH: f32 = 48.0;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn is_left(self) -> bool {
        matches!(self, Side::Left)
    }
}

/// Container for navigation sections.
pub struct Sidebar<E: Collapsible + IntoElement + 'static> {
    content: Vec<E>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    side: Side,
    width: DefiniteLength,
    collapsed: bool,
}

impl<E: Collapsible + IntoElement + 'static> Sidebar<E> {
    fn new(side: Side) -> Self {
        Self {
            content: Vec::new(),
            header: None,
            footer: None,
            side,
            width: px(DEFAULT_WIDTH).into(),
            collapsed: false,
        }
    }

    pub fn left() -> Self {
        Self::new(Side::Left)
    }

    #[allow(dead_code)]
    pub fn right() -> Self {
        Self::new(Side::Right)
    }

    pub fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.width = width.into();
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn child(mut self, child: E) -> Self {
        self.content.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = E>) -> Self {
        self.content.extend(children);
        self
    }
}

impl<E: Collapsible + IntoElement + 'static> RenderOnce for Sidebar<E> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Sidebar {
            mut content,
            header,
            footer,
            side,
            width,
            collapsed,
        } = self;

        let collapsed_width = px(COLLAPSED_WIDTH);

        let mut container = div()
            .v_flex()
            .id("sidebar")
            .w(width)
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(cx.theme().sidebar)
            .text_color(cx.theme().sidebar_foreground)
            .border_1()
            .border_color(cx.theme().sidebar_border);

        if collapsed {
            container = container.w(collapsed_width);
        }

        container = if side.is_left() {
            container.border_r_1()
        } else {
            container.border_l_1()
        };

        if let Some(header) = header {
            container = container.child(div().p(px(12.)).child(header));
        }

        let sections = content
            .drain(..)
            .enumerate()
            .map(|(ix, section)| div().id(ElementId::Integer(ix as u64)).child(section.collapsed(collapsed)));

        container = container.child(
            div()
                .id("sidebar-content")
                .v_flex()
                .flex_1()
                .min_h_0()
                .gap(px(8.))
                .px(px(8.))
                .pb(px(8.))
                .overflow_y_scroll()
                .children(sections),
        );

        if let Some(footer) = footer {
            container = container.child(div().p(px(12.)).child(footer));
        }

        container
    }
}

impl<E: Collapsible + IntoElement + 'static> IntoElement for Sidebar<E> {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

/// Section grouping with optional label.
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

    pub fn child(mut self, child: E) -> Self {
        self.children.push(child);
        self
    }

    #[allow(dead_code)]
    pub fn children(mut self, children: impl IntoIterator<Item = E>) -> Self {
        self.children.extend(children);
        self
    }
}

impl<E: Collapsible + IntoElement + 'static> Collapsible for SidebarGroup<E> {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl<E: Collapsible + IntoElement + 'static> RenderOnce for SidebarGroup<E> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let label = self.label.clone();
        let items = self
            .children
            .into_iter()
            .enumerate()
            .map(|(ix, child)| div().id(ElementId::Integer(ix as u64)).child(child.collapsed(self.collapsed)));

        let mut block = div()
            .v_flex()
            .gap(px(8.))
            .px(px(12.))
            .pb(px(8.));

        if !self.collapsed {
            block = block.child(
                div()
                    .text_xs()
                    .text_color(cx.theme().sidebar_foreground.opacity(0.7))
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

/// Header area at the top of the sidebar.
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

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    #[allow(dead_code)]
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    #[allow(dead_code)]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    #[allow(dead_code)]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = div()
            .h_flex()
            .gap(px(8.))
            .children(self.children.into_iter());

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

/// Vertical list of clickable entries.
pub struct SidebarMenu {
    collapsed: bool,
    items: Vec<SidebarMenuItem>,
}

impl SidebarMenu {
    pub fn new() -> Self {
        Self {
            collapsed: false,
            items: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl Into<SidebarMenuItem>) -> Self {
        self.items.push(child.into());
        self
    }
}

impl Collapsible for SidebarMenu {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().v_flex().gap(px(6.)).children(
            self.items
                .into_iter()
                .enumerate()
                .map(|(ix, item)| item.id(ElementId::Integer(ix as u64)).collapsed(self.collapsed)),
        )
    }
}

impl IntoElement for SidebarMenu {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

/// Single clickable row within a menu.
pub struct SidebarMenuItem {
    id: ElementId,
    icon: Option<Icon>,
    label: SharedString,
    handler: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    active: bool,
    collapsed: bool,
}

impl SidebarMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Integer(0),
            icon: None,
            label: label.into(),
            handler: Rc::new(|_, _, _| {}),
            active: false,
            collapsed: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.handler = Rc::new(handler);
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarMenuItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let handler = self.handler.clone();
        let mut row = div()
            .h_flex()
            .id(self.id.clone())
            .items_center()
            .gap(px(8.))
            .px(px(12.))
            .py(px(8.))
            .rounded(cx.theme().radius)
            .cursor_pointer()
            .hover(|style| {
                style
                    .bg(cx.theme().sidebar_accent.opacity(0.8))
                    .text_color(cx.theme().sidebar_accent_foreground)
            });

        if let Some(icon) = self.icon.clone() {
            row = row.child(icon);
        }

        if self.collapsed {
            row = row.justify_center();
        } else {
            row = row.child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .text_sm()
                    .child(self.label.clone()),
            );
        }

        if self.active {
            row = row
                .bg(cx.theme().sidebar_accent)
                .text_color(cx.theme().sidebar_accent_foreground)
                .font_semibold();
        }

        row.on_click(move |ev, window, cx| (handler)(ev, window, cx))
    }
}

impl IntoElement for SidebarMenuItem {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
