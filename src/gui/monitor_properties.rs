use crate::{
    build_unit_selector,
    layout::{LayoutGen, Position, Rect, Size},
    output_tabs::OutputTabs,
    util::{convert_units, friendly_ftoa, length_from_unit, PosExt, Repack, Unit},
    Number::*,
    RcUi, ADDED_HEIGHT, GROUP_H_PADDING, GROUP_V_PADDING, LINE_V_PADDING,
};
use fltk::{frame::Frame, input::FloatInput, menu::Choice, prelude::*};
use fpvsetup::{find_common_aspect_ratio, MonitorDimensions};
use std::{cmp::max, convert::TryInto, rc::Rc};
use uom::si::length::centimeter;

#[derive(Clone)]
pub struct MonitorProperties {
    pub width_label: Frame,
    pub width_input: FloatInput,
    pub width_unit_selector: Choice,
    pub height_label: Frame,
    pub height_input: FloatInput,
    pub height_unit_selector: Choice,
    pub diagonal_label: Frame,
    pub diagonal_input: FloatInput,
    pub diagonal_unit_selector: Choice,
    pub aspect_label: Frame,
    pub aspect_n_input: FloatInput,
    pub aspect_sep: Frame,
    pub aspect_d_input: FloatInput,
    pub distance_label: Frame,
    pub distance_input: FloatInput,
    pub distance_unit_selector: Choice,
}
impl MonitorProperties {
    /// Generates a not yet laid out monitor properties panel.
    pub fn new(ui: &RcUi, monitor_dimensions: Option<MonitorDimensions>) -> Self {
        let width_label = Frame::default().with_label("Monitor width:");
        let mut width_input = FloatInput::default();
        let r = Rc::clone(ui);
        width_input.set_callback(move || Self::width_or_height_change_handler(&r));
        width_input.set_trigger(CallbackTrigger::Changed);
        if let Some(dim) = monitor_dimensions {
            let [width, _] = dim.width_and_height();
            // We know that the default is centimeters. Should be modified if the default ever changes.
            width_input.set_value(&friendly_ftoa(width.get::<centimeter>()))
        }
        let width_unit_selector =
            build_unit_selector(&width_input, Some(Unit::Centimeters), Plural, false);

        let height_label = Frame::default().with_label(", height:");
        let mut height_input = FloatInput::default();
        let r = Rc::clone(ui);
        height_input.set_callback(move || Self::width_or_height_change_handler(&r));
        height_input.set_trigger(CallbackTrigger::Changed);
        if let Some(dim) = monitor_dimensions {
            let [_, height] = dim.width_and_height();
            // Same as above.
            height_input.set_value(&friendly_ftoa(height.get::<centimeter>()))
        }
        let height_unit_selector =
            build_unit_selector(&height_input, Some(Unit::Centimeters), Plural, false);

        let diagonal_label = Frame::default().with_label("Monitor diagonal:");
        let mut diagonal_input = FloatInput::default();
        let r = Rc::clone(ui);
        diagonal_input.set_callback(move || Self::diagonal_or_aspect_change_handler(&r));
        diagonal_input.set_trigger(CallbackTrigger::Changed);
        let diagonal_unit_selector =
            build_unit_selector(&diagonal_input, Some(Unit::Inches), Plural, false);

        let aspect_label = Frame::default().with_label(", aspect ratio:");
        let mut aspect_n_input = FloatInput::default();
        let r = Rc::clone(ui);
        aspect_n_input.set_callback(move || Self::diagonal_or_aspect_change_handler(&r));
        aspect_n_input.set_trigger(CallbackTrigger::Changed);
        let mut aspect_sep = Frame::default().with_label(":");
        aspect_sep.set_label_font(Font::HelveticaBold);
        let mut aspect_d_input = FloatInput::default();
        let r = Rc::clone(ui);
        aspect_d_input.set_callback(move || Self::diagonal_or_aspect_change_handler(&r));
        aspect_d_input.set_trigger(CallbackTrigger::Changed);

        let distance_label = Frame::default().with_label("Viewing distance:");
        let mut distance_input = FloatInput::default();
        let r = Rc::clone(ui);
        distance_input.set_callback(move || OutputTabs::update(&r));
        distance_input.set_trigger(CallbackTrigger::Changed);

        let distance_unit_selector =
            build_unit_selector(&distance_input, Some(Unit::Centimeters), Plural, false);

        Self {
            width_label,
            width_input,
            width_unit_selector,
            height_label,
            height_input,
            height_unit_selector,
            diagonal_label,
            diagonal_input,
            diagonal_unit_selector,
            aspect_label,
            aspect_n_input,
            aspect_sep,
            aspect_d_input,
            distance_label,
            distance_input,
            distance_unit_selector,
        }
    }
    pub fn apply_layout(&mut self, layout: &MonitorPropertiesLayout, pos: Position) {
        self.width_label
            .set_rect(layout.width_label.with_added_pos(pos));
        self.width_input
            .set_rect(layout.width_input.with_added_pos(pos));
        self.width_unit_selector
            .set_rect(layout.width_unit_selector.with_added_pos(pos));
        self.height_label
            .set_rect(layout.height_label.with_added_pos(pos));
        self.height_input
            .set_rect(layout.height_input.with_added_pos(pos));
        self.height_unit_selector
            .set_rect(layout.height_unit_selector.with_added_pos(pos));
        self.diagonal_label
            .set_rect(layout.diagonal_label.with_added_pos(pos));
        self.diagonal_input
            .set_rect(layout.diagonal_input.with_added_pos(pos));
        self.diagonal_unit_selector
            .set_rect(layout.diagonal_unit_selector.with_added_pos(pos));
        self.aspect_label
            .set_rect(layout.aspect_label.with_added_pos(pos));
        self.aspect_n_input
            .set_rect(layout.aspect_n_input.with_added_pos(pos));
        self.aspect_sep
            .set_rect(layout.aspect_sep.with_added_pos(pos));
        self.aspect_d_input
            .set_rect(layout.aspect_d_input.with_added_pos(pos));
        self.distance_label
            .set_rect(layout.distance_label.with_added_pos(pos));
        self.distance_input
            .set_rect(layout.distance_input.with_added_pos(pos));
        self.distance_unit_selector
            .set_rect(layout.distance_unit_selector.with_added_pos(pos));
    }

    pub fn width_or_height_change_handler(ui: &RcUi) {
        let mut _p = ui.borrow_mut();
        let p = &mut _p.as_mut().unwrap().monitor_properties;
        let width = p.width_input.value().parse::<f64>();
        let height = p.height_input.value().parse::<f64>();
        if let (Ok(width), Ok(height)) = (width, height) {
            let width_unit = p.width_unit_selector.value().try_into().unwrap();
            let height_unit = p.height_unit_selector.value().try_into().unwrap();
            let dimensions = MonitorDimensions::WidthAndHeight {
                width: length_from_unit(width, width_unit),
                height: length_from_unit(height, height_unit),
            };
            let diagonal = convert_units(
                dimensions.diagonal(),
                p.diagonal_unit_selector.value().try_into().unwrap(),
            );
            let aspect = dimensions.aspect();
            p.diagonal_input.set_value(&friendly_ftoa(diagonal));
            let [n, d] = find_common_aspect_ratio(aspect, 0.1).unwrap_or([aspect, 1.0]);
            p.aspect_n_input.set_value(&friendly_ftoa(n));
            p.aspect_d_input.set_value(&friendly_ftoa(d));

            drop(_p);
            OutputTabs::update(ui);
        }
    }
    fn diagonal_or_aspect_change_handler(ui: &RcUi) {
        let mut _p = ui.borrow_mut();
        let p = &mut _p.as_mut().unwrap().monitor_properties;
        let diagonal = p.diagonal_input.value().parse::<f64>();
        let aspect_n = p.aspect_n_input.value().parse::<f64>();
        let aspect_d = p.aspect_d_input.value().parse::<f64>();
        if let (Ok(diagonal), [Ok(n), Ok(d)]) = (diagonal, [aspect_n, aspect_d]) {
            let diagonal_unit = p.diagonal_unit_selector.value().try_into().unwrap();
            let dimensions = MonitorDimensions::DiagonalAndAspect {
                diagonal: length_from_unit(diagonal, diagonal_unit),
                aspect: n / d,
            };
            let [width, height] = dimensions.width_and_height();
            let width = convert_units(width, p.width_unit_selector.value().try_into().unwrap());
            let height = convert_units(height, p.height_unit_selector.value().try_into().unwrap());
            p.width_input.set_value(&friendly_ftoa(width));
            p.height_input.set_value(&friendly_ftoa(height));

            drop(_p);
            OutputTabs::update(ui);
        }
    }
}

impl LayoutGen<'_> for MonitorProperties {
    type Arguments = ();
    type Layout = MonitorPropertiesLayout;

    fn generate_layout(&self, _: Self::Arguments) -> Self::Layout {
        const NUM_LINES: i32 = 3;

        let height_l1;
        // Start out with this to include padding.
        let mut width_l1 = GROUP_H_PADDING * 2;

        let width_label = Rect(
            Position(GROUP_H_PADDING, GROUP_V_PADDING),
            self.width_label.measure_label().repack(),
        );
        height_l1 = width_label.h() + ADDED_HEIGHT;
        width_l1 += width_label.w();

        let width_input = Rect(width_label.to_right(5), Size(70, height_l1));
        width_l1 += width_input.w() + 5;

        let width_unit_selector = Rect(width_input.to_right(5), Size(105, height_l1));
        width_l1 += self.width_unit_selector.w() + 5;

        let height_label = Rect(
            width_unit_selector.to_right(0),
            self.height_label.measure_label().repack(),
        );
        width_l1 += height_label.w();

        let height_input = Rect(height_label.to_right(5), Size(70, height_l1));
        width_l1 += height_input.w() + 5;

        let height_unit_selector = Rect(height_input.to_right(5), Size(105, height_l1));
        width_l1 += height_unit_selector.w() + 5;

        let height_l2;
        let mut width_l2 = GROUP_H_PADDING * 2;

        let diagonal_label = Rect(
            width_label.to_bottom(LINE_V_PADDING),
            self.diagonal_label.measure_label().repack(),
        );
        height_l2 = diagonal_label.h() + ADDED_HEIGHT;
        width_l2 += diagonal_label.w();

        let diagonal_input = Rect(diagonal_label.to_right(5), Size(70, height_l2));
        width_l2 += diagonal_input.w() + 5;

        let diagonal_unit_selector = Rect(diagonal_input.to_right(5), Size(105, height_l2));
        width_l2 += diagonal_unit_selector.w() + 5;

        let aspect_label = Rect(
            diagonal_unit_selector.to_right(0),
            self.aspect_label.measure_label().repack(),
        );
        width_l2 += aspect_label.w();

        let aspect_n_input = Rect(aspect_label.to_right(5), Size(65, height_l2));
        width_l2 += aspect_n_input.w() + 5;

        let aspect_sep = Rect(
            aspect_n_input.to_right(1),
            self.aspect_sep.measure_label().repack(),
        );
        width_l2 += aspect_sep.w() + 1;

        let aspect_d_input = Rect(aspect_sep.to_right(1), Size(65, height_l2));
        width_l2 += aspect_d_input.w() + 1;

        let height_l3;
        let mut width_l3 = GROUP_H_PADDING * 2;

        let distance_label = Rect(
            diagonal_label.to_bottom(LINE_V_PADDING),
            self.distance_label.measure_label().repack(),
        );
        height_l3 = distance_label.h() + ADDED_HEIGHT;
        width_l3 += distance_label.w();

        let distance_input = Rect(distance_label.to_right(5), Size(70, height_l3));
        width_l3 += distance_input.w() + 5;

        let distance_unit_selector = Rect(distance_input.to_right(5), Size(105, height_l3));
        width_l3 += distance_unit_selector.w() + 5;

        let total_width = [width_l1, width_l2, width_l3]
            .iter()
            .copied()
            .reduce(max)
            .unwrap();
        let total_height = height_l1
            + height_l2
            + height_l3
            + LINE_V_PADDING * (NUM_LINES - 1)
            + GROUP_V_PADDING * 2;
        MonitorPropertiesLayout {
            total_size: Size(total_width, total_height),
            width_label,
            width_input,
            width_unit_selector,
            height_label,
            height_input,
            height_unit_selector,
            diagonal_label,
            diagonal_input,
            diagonal_unit_selector,
            aspect_label,
            aspect_n_input,
            aspect_sep,
            aspect_d_input,
            distance_label,
            distance_input,
            distance_unit_selector,
        }
    }
}

make_layout!(pub MonitorPropertiesLayout, has
    width_label, width_input, width_unit_selector,
    height_label, height_input, height_unit_selector,
    diagonal_label, diagonal_input, diagonal_unit_selector,
    aspect_label, aspect_n_input, aspect_sep, aspect_d_input,
    distance_label, distance_input, distance_unit_selector,
);
