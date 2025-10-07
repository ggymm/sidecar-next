use std::fmt::Debug;

use gpui::{DefiniteLength, Edges, FontWeight, Styled};

/// Additional layout helpers that mirror the subset of gpui-component's
/// `StyledExt` used by this project.
pub trait StyledExt: Styled + Sized {
    /// Apply horizontal flexbox layout with vertically centered items.
    #[inline]
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Apply vertical flexbox layout.
    #[inline]
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Apply semibold font weight.
    #[inline]
    fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SEMIBOLD)
    }

    /// Conveniently set paddings using `Edges`.
    fn paddings<L>(self, paddings: impl Into<Edges<L>>) -> Self
    where
        L: Into<DefiniteLength> + Clone + Default + Debug + PartialEq,
    {
        let paddings = paddings.into();
        self.pt(paddings.top.into())
            .pb(paddings.bottom.into())
            .pl(paddings.left.into())
            .pr(paddings.right.into())
    }

    /// Conveniently set margins using `Edges`.
    fn margins<L>(self, margins: impl Into<Edges<L>>) -> Self
    where
        L: Into<DefiniteLength> + Clone + Default + Debug + PartialEq,
    {
        let margins = margins.into();
        self.mt(margins.top.into())
            .mb(margins.bottom.into())
            .ml(margins.left.into())
            .mr(margins.right.into())
    }
}

impl<E: Styled> StyledExt for E {}
