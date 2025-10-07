use crate::comps::traits::Disableable;
use gpui::{
    div, px, App, ClickEvent, Component, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement, Refineable, RenderOnce, SharedString, StyleRefinement, Styled,
    StatefulInteractiveElement, Window,
};
use gpui::prelude::FluentBuilder as _;
use gpui_component::ActiveTheme as _;

/// Variants supported by the custom button implementation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Info,
    Success,
    Warning,
    Ghost,
    Link,
    Text,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Secondary
    }
}

/// Builders for switching button variants via a fluent API.
pub trait ButtonVariants: Sized {
    fn with_variant(self, variant: ButtonVariant) -> Self;

    fn primary(self) -> Self {
        self.with_variant(ButtonVariant::Primary)
    }

    fn danger(self) -> Self {
        self.with_variant(ButtonVariant::Danger)
    }

    fn info(self) -> Self {
        self.with_variant(ButtonVariant::Info)
    }

    fn success(self) -> Self {
        self.with_variant(ButtonVariant::Success)
    }

    fn warning(self) -> Self {
        self.with_variant(ButtonVariant::Warning)
    }

    fn ghost(self) -> Self {
        self.with_variant(ButtonVariant::Ghost)
    }

    fn link(self) -> Self {
        self.with_variant(ButtonVariant::Link)
    }

    fn text(self) -> Self {
        self.with_variant(ButtonVariant::Text)
    }
}

/// Minimal button implementation mirroring the subset of gpui-component APIs
/// currently used by the application.
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    variant: ButtonVariant,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            variant: ButtonVariant::default(),
            disabled: false,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ButtonVariants for Button {
    fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Button {
            id,
            label,
            variant,
            disabled,
            on_click,
            style,
        } = self;

        let skin = ButtonSkin::from_theme(cx, variant);
        let theme = cx.theme();

        let mut button = div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .px(px(12.))
            .py(px(6.))
            .rounded(theme.radius)
            .text_sm()
            .border_1()
            .border_color(skin.border)
            .when(!disabled, |this| {
                this.bg(skin.base_bg)
                    .text_color(skin.base_fg)
                    .cursor_pointer()
                    .hover(|style| {
                        style
                            .bg(skin.hover_bg)
                            .text_color(skin.hover_fg)
                            .border_color(skin.border)
                    })
                    .active(|style| {
                        style
                            .bg(skin.active_bg)
                            .text_color(skin.base_fg)
                            .border_color(skin.border)
                    })
            })
            .when(disabled, |this| {
                this.bg(theme.muted)
                    .text_color(theme.muted_foreground)
                    .opacity(0.6)
                    .cursor_default()
            });

        button.style().refine(&style);

        if let Some(label) = label {
            button = button.child(label);
        }

        if let Some(on_click) = on_click.filter(|_| !disabled) {
            button = button.on_click(on_click);
        }

        button
    }
}

impl IntoElement for Button {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

struct ButtonSkin {
    base_bg: Hsla,
    base_fg: Hsla,
    border: Hsla,
    hover_bg: Hsla,
    hover_fg: Hsla,
    active_bg: Hsla,
}

impl ButtonSkin {
    fn from_theme(cx: &App, variant: ButtonVariant) -> Self {
        let theme = cx.theme();
        match variant {
            ButtonVariant::Primary => Self {
                base_bg: theme.primary,
                base_fg: theme.primary_foreground,
                border: theme.primary,
                hover_bg: theme.primary_hover,
                hover_fg: theme.primary_foreground,
                active_bg: theme.primary_active,
            },
            ButtonVariant::Danger => Self {
                base_bg: theme.danger,
                base_fg: theme.danger_foreground,
                border: theme.danger,
                hover_bg: theme.danger_hover,
                hover_fg: theme.danger_foreground,
                active_bg: theme.danger_active,
            },
            ButtonVariant::Info => Self {
                base_bg: theme.info,
                base_fg: theme.info_foreground,
                border: theme.info,
                hover_bg: theme.info_hover,
                hover_fg: theme.info_foreground,
                active_bg: theme.info_active,
            },
            ButtonVariant::Success => Self {
                base_bg: theme.success,
                base_fg: theme.success_foreground,
                border: theme.success,
                hover_bg: theme.success_hover,
                hover_fg: theme.success_foreground,
                active_bg: theme.success_active,
            },
            ButtonVariant::Warning => Self {
                base_bg: theme.warning,
                base_fg: theme.warning_foreground,
                border: theme.warning,
                hover_bg: theme.warning_hover,
                hover_fg: theme.warning_foreground,
                active_bg: theme.warning_active,
            },
            ButtonVariant::Ghost => Self {
                base_bg: theme.transparent,
                base_fg: theme.secondary_foreground,
                border: theme.transparent,
                hover_bg: theme.secondary_hover,
                hover_fg: theme.secondary_foreground,
                active_bg: theme.secondary_active,
            },
            ButtonVariant::Link => Self {
                base_bg: theme.transparent,
                base_fg: theme.link,
                border: theme.transparent,
                hover_bg: theme.transparent,
                hover_fg: theme.link_hover,
                active_bg: theme.transparent,
            },
            ButtonVariant::Text => Self {
                base_bg: theme.transparent,
                base_fg: theme.foreground,
                border: theme.transparent,
                hover_bg: theme.transparent,
                hover_fg: theme.foreground,
                active_bg: theme.transparent,
            },
            ButtonVariant::Secondary => Self {
                base_bg: theme.secondary,
                base_fg: theme.secondary_foreground,
                border: theme.border,
                hover_bg: theme.secondary_hover,
                hover_fg: theme.secondary_foreground,
                active_bg: theme.secondary_active,
            },
        }
    }
}
