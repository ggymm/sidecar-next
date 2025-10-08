use gpui::App;
use gpui::Component;
use gpui::SharedString;
use gpui::StyleRefinement;
use gpui::Window;

use gpui::IntoElement;
use gpui::RenderOnce;
use gpui::Styled;

use gpui::svg;

use gpui_component::{Sizable, Size as UiSize};

pub struct Icon {
    path: SharedString,
    style: StyleRefinement,
    size: Option<UiSize>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            path: "".into(),
            style: StyleRefinement::default(),
            size: None,
        }
    }
}

impl Clone for Icon {
    fn clone(&self) -> Self {
        let mut icon = Self::default();
        icon.path = self.path.clone();
        icon.style = self.style.clone();
        icon.size = self.size;
        icon
    }
}

impl Icon {
    pub fn path(
        mut self,
        path: impl Into<SharedString>,
    ) -> Self {
        self.path = path.into();
        self
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }

    fn text_color(
        mut self,
        color: impl Into<gpui::Hsla>,
    ) -> Self {
        self.style.text.get_or_insert_with(Default::default).color = Some(color.into());
        self
    }
}

impl Sizable for Icon {
    fn with_size(
        mut self,
        size: impl Into<UiSize>,
    ) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(
        self,
        window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let mut svg = svg().flex_none();
        *svg.style() = self.style;

        let color = {
            let style = svg.style();
            style
                .text
                .as_ref()
                .and_then(|text| text.color)
                .unwrap_or_else(|| window.text_style().color)
        };
        svg = svg.flex_shrink_0().text_color(color);

        svg = match self.size {
            Some(UiSize::Size(px)) => svg.size(px),
            Some(UiSize::XSmall) => svg.size_3(),
            Some(UiSize::Small) => svg.size_3p5(),
            Some(UiSize::Medium) => svg.size_4(),
            Some(UiSize::Large) => svg.size_6(),
            None => {
                let text_size = window.text_style().font_size.to_pixels(window.rem_size());
                svg.size(text_size)
            }
        };

        if self.path.is_empty() { svg } else { svg.path(self.path) }
    }
}

impl IntoElement for Icon {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
