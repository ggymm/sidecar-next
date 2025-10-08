use std::fmt::Debug;

use gpui::{DefiniteLength, Edges, FontWeight, Styled};

pub trait StyledExt: Styled + Sized {
    #[inline]
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    #[inline]
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    #[inline]
    fn font_semibold(self) -> Self {
        self.font_weight(FontWeight::SEMIBOLD)
    }

    fn paddings<L>(
        self,
        paddings: impl Into<Edges<L>>,
    ) -> Self
    where
        L: Into<DefiniteLength> + Clone + Default + Debug + PartialEq,
    {
        let paddings = paddings.into();
        self.pt(paddings.top.into())
            .pb(paddings.bottom.into())
            .pl(paddings.left.into())
            .pr(paddings.right.into())
    }

    fn margins<L>(
        self,
        margins: impl Into<Edges<L>>,
    ) -> Self
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
