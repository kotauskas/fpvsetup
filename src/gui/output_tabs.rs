use crate::{
    util::PosExt, Focused, FocusedLayout, LayoutGen, PortalLike, PortalLikeLayout, Position, RcUi,
    Rect, Size, GROUP_H_PADDING, GROUP_V_PADDING,
};
use core::cmp::max;
use fltk::{group::Tabs, prelude::*};

#[derive(Clone)]
pub struct OutputTabs {
    pub tabs: Tabs,
    pub portal_like: PortalLike,
    pub focused: Focused,
}
impl OutputTabs {
    pub fn new(ui: &RcUi) -> Self {
        let tabs = Tabs::default();
        let portal_like = PortalLike::new();
        let focused = Focused::new(ui);
        tabs.end();
        Self {
            tabs,
            portal_like,
            focused,
        }
    }
    pub fn apply_layout(
        &mut self,
        layout: &OutputTabsLayout,
        portal_like_layout: &PortalLikeLayout,
        focused_layout: &FocusedLayout,
        pos: Position,
    ) {
        self.tabs.set_rect(layout.tabs.with_added_pos(pos));

        self.portal_like
            .apply_layout(portal_like_layout, layout.portal_like.pos() + pos);

        self.focused
            .apply_layout(focused_layout, layout.focused.pos() + pos);
    }
    pub fn update(ui: &RcUi) {
        PortalLike::update(ui);
        Focused::update(ui);
    }
}
impl<'a> LayoutGen<'a> for OutputTabs {
    type Arguments = (&'a PortalLikeLayout, &'a FocusedLayout, i32);
    type Layout = OutputTabsLayout;

    fn generate_layout(
        &self,
        (portal_like_layout, focused_layout, fill_width): Self::Arguments,
    ) -> Self::Layout {
        const TABS_HEADER_HEIGHT: i32 = 21;

        let Size(pl_w, pl_h) = portal_like_layout.total_size;
        let Size(fo_w, fo_h) = focused_layout.total_size;
        let aggregate_width = [pl_w, fo_w, fill_width - GROUP_H_PADDING * 2]
            .iter()
            .copied()
            .reduce(max)
            .unwrap();
        let aggregate_height = max(pl_h, fo_h) + TABS_HEADER_HEIGHT;

        let tabs = Rect(
            Position(GROUP_H_PADDING, GROUP_V_PADDING),
            Size(aggregate_width, aggregate_height),
        );
        let portal_like = Rect(
            tabs.pos() + Position(0, TABS_HEADER_HEIGHT),
            portal_like_layout.total_size,
        );
        let focused = Rect(
            tabs.pos() + Position(0, TABS_HEADER_HEIGHT),
            focused_layout.total_size,
        );

        let total_width = GROUP_H_PADDING * 2 + tabs.w();
        let total_height = GROUP_V_PADDING * 2 + tabs.h();
        let total_size = Size(total_width, total_height);

        OutputTabsLayout {
            total_size,
            tabs,
            portal_like,
            focused,
        }
    }
}

make_layout!(pub OutputTabsLayout, has tabs, portal_like, focused);
