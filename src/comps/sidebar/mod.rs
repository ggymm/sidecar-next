use std::rc::Rc;

use gpui::AnyElement;
use gpui::App;
use gpui::ClickEvent;
use gpui::Component;
use gpui::DefiniteLength;
use gpui::ElementId;
use gpui::SharedString;
use gpui::Window;

use gpui::InteractiveElement;
use gpui::IntoElement;
use gpui::ParentElement;
use gpui::RenderOnce;
use gpui::StatefulInteractiveElement;
use gpui::Styled;

use gpui::div;
use gpui::px;

use gpui_component::ActiveTheme as _;

use crate::comps::Collapsible;
use crate::comps::Icon;
use crate::comps::StyledExt;

mod group;
mod header;

pub use group::*;
pub use header::*;

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

    pub fn width(
        mut self,
        width: impl Into<DefiniteLength>,
    ) -> Self {
        self.width = width.into();
        self
    }

    pub fn collapsed(
        mut self,
        collapsed: bool,
    ) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn header(
        mut self,
        header: impl IntoElement,
    ) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn footer(
        mut self,
        footer: impl IntoElement,
    ) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn child(
        mut self,
        child: E,
    ) -> Self {
        self.content.push(child);
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.content.extend(children);
        self
    }
}

impl<E: Collapsible + IntoElement + 'static> RenderOnce for Sidebar<E> {
    fn render(
        self,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
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

        let sections = content.drain(..).enumerate().map(|(ix, section)| {
            div()
                .id(ElementId::Integer(ix as u64))
                .child(section.collapsed(collapsed))
        });

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

    pub fn child(
        mut self,
        child: impl Into<SidebarMenuItem>,
    ) -> Self {
        self.items.push(child.into());
        self
    }
}

impl Collapsible for SidebarMenu {
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

impl RenderOnce for SidebarMenu {
    fn render(
        self,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
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

pub struct SidebarMenuItem {
    id: ElementId,
    icon: Option<Icon>,
    label: SharedString,
    event: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    active: bool,
    collapsed: bool,
}

impl SidebarMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Integer(0),
            icon: None,
            label: label.into(),
            event: Rc::new(|_, _, _| {}),
            active: false,
            collapsed: false,
        }
    }

    pub fn icon(
        mut self,
        icon: impl Into<Icon>,
    ) -> Self {
        self.icon = Some(icon.into());
        self
    }

    fn id(
        mut self,
        id: impl Into<ElementId>,
    ) -> Self {
        self.id = id.into();
        self
    }

    pub fn active(
        mut self,
        active: bool,
    ) -> Self {
        self.active = active;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.event = Rc::new(handler);
        self
    }

    pub fn collapsed(
        mut self,
        collapsed: bool,
    ) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl RenderOnce for SidebarMenuItem {
    fn render(
        self,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let event = self.event.clone();
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
            row = row.child(div().flex_1().overflow_hidden().text_sm().child(self.label.clone()));
        }

        if self.active {
            row = row
                .bg(cx.theme().sidebar_accent)
                .text_color(cx.theme().sidebar_accent_foreground)
                .font_semibold();
        }

        row.on_click(move |ev, window, cx| event(ev, window, cx))
    }
}

impl IntoElement for SidebarMenuItem {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
