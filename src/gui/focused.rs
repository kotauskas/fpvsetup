use crate::{
    build_unit_selector,
    layout::{LayoutGen, Position, Rect, Size},
    util::{friendly_ftoa, length_from_unit, PosExt, Repack, Unit, DEGREE_SIGN},
    Number::*,
    RcUi, ADDED_HEIGHT, GROUP_H_PADDING, GROUP_V_PADDING, LINE_V_PADDING,
};
use fltk::{frame::Frame, group::Group, input::FloatInput, menu::Choice, prelude::*};
use fpvsetup::MonitorConfiguration;
use std::{cmp::max, convert::TryInto, rc::Rc};
use uom::si::angle::degree;

#[derive(Clone)]
pub struct Focused {
    pub containing_group: Group,
    pub accurate_distance_label_1: Frame,
    pub accurate_distance_input: FloatInput,
    pub accurate_distance_label_2: Frame,
    pub accurate_distance_unit_selector: Choice,
    pub fov_output_label: Frame,
    pub fov_output: FloatInput,
}
impl Focused {
    pub fn new(ui: &RcUi) -> Self {
        let containing_group = Group::default().with_label("Focused");

        let accurate_distance_label_1 = Frame::default().with_label("Accurate scale");
        let mut accurate_distance_input = FloatInput::default();
        let r = Rc::clone(&ui);
        accurate_distance_input.set_callback(move || Self::update(&r));
        accurate_distance_input.set_trigger(CallbackTrigger::Changed);
        let accurate_distance_unit_selector =
            build_unit_selector(&accurate_distance_input, Some(Unit::Meters), Plural, false);
        let accurate_distance_label_2 = Frame::default().with_label("away from the camera");

        let fov_output_label = Frame::default().with_label("Camera field of view:");
        let mut fov_output = FloatInput::default();
        fov_output.set_readonly(true);

        containing_group.end();

        Self {
            containing_group,
            accurate_distance_label_1,
            accurate_distance_input,
            accurate_distance_label_2,
            accurate_distance_unit_selector,
            fov_output_label,
            fov_output,
        }
    }
    pub fn apply_layout(&mut self, layout: &FocusedLayout, pos: Position) {
        self.containing_group
            .set_rect(layout.containing_group.with_added_pos(pos));
        self.accurate_distance_label_1
            .set_rect(layout.accurate_distance_label_1.with_added_pos(pos));
        self.accurate_distance_input
            .set_rect(layout.accurate_distance_input.with_added_pos(pos));
        self.accurate_distance_label_2
            .set_rect(layout.accurate_distance_label_2.with_added_pos(pos));
        self.accurate_distance_unit_selector
            .set_rect(layout.accurate_distance_unit_selector.with_added_pos(pos));
        self.fov_output_label
            .set_rect(layout.fov_output_label.with_added_pos(pos));
        self.fov_output
            .set_rect(layout.fov_output.with_added_pos(pos));
    }
    pub fn update(ui: &RcUi) {
        let mut _u = ui.borrow_mut();
        let u = _u.as_mut().unwrap();
        let mp = &mut u.monitor_properties;
        let fo = &mut u.output_tabs.focused;
        let width = mp.width_input.value().parse::<f64>();
        let height = mp.height_input.value().parse::<f64>();
        let distance = mp.distance_input.value().parse::<f64>();
        let accurate_distance = fo.accurate_distance_input.value().parse::<f64>();
        if let (Ok(width), Ok(height), Ok(distance), Ok(accurate_distance)) =
            (width, height, distance, accurate_distance)
        {
            let width_unit = mp.width_unit_selector.value().try_into().unwrap();
            let height_unit = mp.height_unit_selector.value().try_into().unwrap();
            let distance_unit = mp.distance_unit_selector.value().try_into().unwrap();
            let accurate_distance_unit = fo
                .accurate_distance_unit_selector
                .value()
                .try_into()
                .unwrap();
            let width = length_from_unit(width, width_unit);
            let height = length_from_unit(height, height_unit);
            let distance = length_from_unit(distance, distance_unit);
            let accurate_distance = length_from_unit(accurate_distance, accurate_distance_unit);

            let monitor_conf = MonitorConfiguration {
                dimensions: fpvsetup::MonitorDimensions::WidthAndHeight { width, height },
                distance,
            };
            let fov = monitor_conf.monitor_fov_for_distance(accurate_distance, true);
            fo.fov_output.set_value(&format!(
                "{}{}",
                &friendly_ftoa(fov.get::<degree>()),
                DEGREE_SIGN,
            ));
        }
    }
}
impl LayoutGen<'_> for Focused {
    type Layout = FocusedLayout;
    type Arguments = ();

    fn generate_layout(&self, _: Self::Arguments) -> Self::Layout {
        const NUM_LINES: i32 = 2;

        let mut width_l1 = GROUP_H_PADDING * 2;
        let height_l1;

        let accurate_distance_label_1 = Rect(
            Position(GROUP_H_PADDING, GROUP_V_PADDING),
            self.accurate_distance_label_1.measure_label().repack(),
        );
        height_l1 = accurate_distance_label_1.h() + ADDED_HEIGHT;
        width_l1 += accurate_distance_label_1.w();

        let accurate_distance_input =
            Rect(accurate_distance_label_1.to_right(5), Size(70, height_l1));
        width_l1 += accurate_distance_input.w();

        let accurate_distance_unit_selector =
            Rect(accurate_distance_input.to_right(5), Size(105, height_l1));
        width_l1 += accurate_distance_unit_selector.w();

        let accurate_distance_label_2 = Rect(
            accurate_distance_unit_selector.to_right(5),
            self.accurate_distance_label_2.measure_label().repack(),
        );
        width_l1 += accurate_distance_label_2.w();

        let mut width_l2 = GROUP_H_PADDING * 2;
        let height_l2;

        let fov_output_label = Rect(
            accurate_distance_label_1.to_bottom(LINE_V_PADDING),
            self.fov_output_label.measure_label().repack(),
        );
        height_l2 = fov_output_label.h() + ADDED_HEIGHT;
        width_l2 += fov_output_label.w();

        let fov_output = Rect(fov_output_label.to_right(5), Size(70, height_l2));
        width_l2 += fov_output.w();

        let total_width = max(width_l1, width_l2);
        let total_height =
            height_l1 + height_l2 + LINE_V_PADDING * (NUM_LINES - 1) + GROUP_V_PADDING * 2;
        let total_size = Size(total_width, total_height);
        let containing_group = Rect(Position(0, 0), total_size);
        FocusedLayout {
            total_size,
            containing_group,
            accurate_distance_label_1,
            accurate_distance_input,
            accurate_distance_unit_selector,
            accurate_distance_label_2,
            fov_output_label,
            fov_output,
        }
    }
}

make_layout!(pub FocusedLayout, has
    containing_group,
    accurate_distance_label_1,
    accurate_distance_input,
    accurate_distance_unit_selector,
    accurate_distance_label_2,
    fov_output_label,
    fov_output,
);
