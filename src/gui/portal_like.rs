use crate::{
    build_unit_selector,
    util::{convert_units, friendly_ftoa, length_from_unit, PosExt, Unit, DEGREE_SIGN},
    LayoutGen,
    Number::*,
    Position, RcUi, Rect, Repack, Size, ADDED_HEIGHT, GROUP_H_PADDING, GROUP_V_PADDING,
    LINE_V_PADDING,
};
use fltk::{frame::Frame, group::Group, input::FloatInput, menu::Choice, prelude::*};
use fpvsetup::{MonitorConfiguration, MonitorDimensions};
use std::{cmp::max, convert::TryInto};
use uom::si::angle::degree;

#[derive(Clone)]
pub struct PortalLike {
    pub containing_group: Group,
    pub fov_label: Frame,
    pub fov_output: FloatInput,
    pub move_label_1: Frame,
    pub move_output: FloatInput,
    pub move_unit_selector: Choice,
    pub move_label_2: Frame,
    pub move_units_output: FloatInput,
    pub move_label_3: Frame,
}
impl PortalLike {
    pub fn new() -> Self {
        let containing_group = Group::default().with_label("Portal-like");

        let fov_label = Frame::default().with_label("Field of view:");
        let mut fov_output = FloatInput::default();
        fov_output.set_readonly(true);

        let move_label_1 = Frame::default().with_label("Move the camera back");
        let mut move_output = FloatInput::default();
        move_output.set_readonly(true);
        let move_unit_selector =
            build_unit_selector(&move_output, Some(Unit::Meters), Plural, false);

        let move_label_2 = Frame::default().with_label("(");
        let mut move_units_output = FloatInput::default();
        move_units_output.set_readonly(true);
        let move_label_3 = Frame::default().with_label("units)");

        containing_group.end();

        Self {
            containing_group,
            fov_label,
            fov_output,
            move_label_1,
            move_output,
            move_unit_selector,
            move_label_2,
            move_units_output,
            move_label_3,
        }
    }
    pub fn apply_layout(&mut self, layout: &PortalLikeLayout, pos: Position) {
        self.containing_group
            .set_rect(layout.containing_group.with_added_pos(pos));
        self.fov_label
            .set_rect(layout.fov_label.with_added_pos(pos));
        self.fov_output
            .set_rect(layout.fov_output.with_added_pos(pos));
        self.move_label_1
            .set_rect(layout.move_label_1.with_added_pos(pos));
        self.move_output
            .set_rect(layout.move_output.with_added_pos(pos));
        self.move_unit_selector
            .set_rect(layout.move_unit_selector.with_added_pos(pos));
        self.move_label_2
            .set_rect(layout.move_label_2.with_added_pos(pos));
        self.move_units_output
            .set_rect(layout.move_units_output.with_added_pos(pos));
        self.move_label_3
            .set_rect(layout.move_label_3.with_added_pos(pos));
    }
    pub fn update(ui: &RcUi) {
        let mut _u = ui.borrow_mut();
        let u = _u.as_mut().unwrap();
        let mp = &mut u.monitor_properties;
        let pl = &mut u.output_tabs.portal_like;
        let us = &mut u.unit_setup;
        let width = mp.width_input.value().parse::<f64>();
        let height = mp.width_input.value().parse::<f64>();
        let distance = mp.distance_input.value().parse::<f64>();
        let app_per_real = us.app_per_real_input.value().parse::<f64>();
        if let (Ok(width), Ok(height), Ok(distance), Ok(app_per_real)) =
            (width, height, distance, app_per_real)
        {
            let move_unit = pl.move_unit_selector.value().try_into().unwrap();
            let width_unit = mp.width_unit_selector.value().try_into().unwrap();
            let height_unit = mp.height_unit_selector.value().try_into().unwrap();
            let distance_unit = mp.distance_unit_selector.value().try_into().unwrap();
            let width = length_from_unit(width, width_unit);
            let height = length_from_unit(height, height_unit);
            let distance = length_from_unit(distance, distance_unit);

            let mov = convert_units(distance, move_unit);
            let monitor_conf = MonitorConfiguration {
                dimensions: MonitorDimensions::WidthAndHeight { width, height },
                distance,
            };
            let fov = monitor_conf.fov();

            pl.fov_output.set_value(&format!(
                "{}{}",
                &friendly_ftoa(fov.get::<degree>()),
                DEGREE_SIGN,
            ));
            pl.move_output.set_value(&friendly_ftoa(mov));
            pl.move_units_output
                .set_value(&friendly_ftoa(mov * app_per_real));
        }
    }
}
impl LayoutGen<'_> for PortalLike {
    type Arguments = ();
    type Layout = PortalLikeLayout;
    fn generate_layout(&self, _: Self::Arguments) -> Self::Layout {
        const NUM_LINES: i32 = 2;

        let height_l1;
        let mut width_l1 = GROUP_H_PADDING * 2;

        let fov_label = Rect(
            Position(GROUP_H_PADDING, GROUP_V_PADDING),
            self.fov_label.measure_label().repack(),
        );
        height_l1 = fov_label.h() + ADDED_HEIGHT;
        width_l1 += fov_label.w();

        let fov_output = Rect(fov_label.to_right(5), Size(70, height_l1));
        width_l1 += fov_output.w();

        let height_l2;
        let mut width_l2 = GROUP_H_PADDING * 2;
        let move_label_1 = Rect(
            fov_label.to_bottom(LINE_V_PADDING),
            self.move_label_1.measure_label().repack(),
        );
        height_l2 = move_label_1.h() + ADDED_HEIGHT;
        width_l2 += move_label_1.w();

        let move_output = Rect(move_label_1.to_right(5), Size(70, height_l2));
        width_l2 += move_output.w();

        let move_unit_selector = Rect(move_output.to_right(5), Size(105, height_l2));
        width_l2 += move_unit_selector.w();

        let move_label_2 = Rect(
            move_unit_selector.to_right(5),
            self.move_label_2.measure_label().repack(),
        );
        width_l2 += move_label_2.w();

        let move_units_output = Rect(move_label_2.to_right(2), Size(70, height_l2));
        width_l2 += move_units_output.w();

        let move_label_3 = Rect(
            move_units_output.to_right(5),
            self.move_label_3.measure_label().repack(),
        );
        width_l2 += move_label_3.w();

        let total_width = max(width_l1, width_l2);
        let total_height =
            height_l1 + height_l2 + LINE_V_PADDING * (NUM_LINES - 1) + GROUP_V_PADDING * 2;

        let total_size = Size(total_width, total_height);
        PortalLikeLayout {
            total_size,
            containing_group: Rect(Position(0, 0), total_size),
            fov_label,
            fov_output,
            move_label_1,
            move_output,
            move_unit_selector,
            move_label_2,
            move_units_output,
            move_label_3,
        }
    }
}

make_layout!(pub PortalLikeLayout, has
    containing_group,
    fov_label, fov_output,
    move_label_1, move_output, move_unit_selector,
    move_label_2, move_units_output, move_label_3,
);
