use crate::{
    build_unit_selector,
    layout::{LayoutGen, Position, Rect, Size},
    output_tabs::OutputTabs,
    util::{convert_units, friendly_ftoa, length_from_unit, PosExt, Repack, Unit},
    Number::*,
    RcUi, ADDED_HEIGHT, GROUP_H_PADDING, GROUP_V_PADDING, LINE_V_PADDING,
};
use fltk::{
    frame::Frame, input::FloatInput, menu::Choice, CallbackTrigger, InputExt, MenuExt, WidgetExt,
};
use std::{cmp::max, convert::TryInto, rc::Rc};

#[derive(Clone)]
pub struct UnitSetup {
    pub app_per_real_selector_label: Frame,
    pub app_per_real_unit_selector: Choice,
    pub app_per_real_input_label: Frame,
    pub app_per_real_input: FloatInput,
    pub real_per_app_selector_label: Frame,
    pub real_per_app_unit_selector: Choice,
    pub real_per_app_input_label: Frame,
    pub real_per_app_input: FloatInput,
}

impl UnitSetup {
    pub fn new(ui: &RcUi) -> Self {
        let app_per_real_selector_label = Frame::default().with_label("Length of one");
        let app_per_real_input_label = Frame::default().with_label("in application units:");
        let mut app_per_real_input = FloatInput::default();
        let r = Rc::clone(ui);
        app_per_real_input.set_callback(move || Self::app_per_real_change_handler(&r));
        app_per_real_input.set_trigger(CallbackTrigger::Changed);
        let app_per_real_unit_selector =
            build_unit_selector(&app_per_real_input, Some(Unit::Meters), Singular, true);

        let real_per_app_selector_label =
            Frame::default().with_label("Length of one application unit in");
        let real_per_app_input_label = Frame::default().with_label(":");
        let mut real_per_app_input = FloatInput::default();
        let r = Rc::clone(ui);
        real_per_app_input.set_callback(move || Self::real_per_app_change_handler(&r));
        real_per_app_input.set_trigger(CallbackTrigger::Changed);
        let real_per_app_unit_selector =
            build_unit_selector(&real_per_app_input, Some(Unit::Meters), Plural, false);
        Self {
            app_per_real_selector_label,
            app_per_real_unit_selector,
            app_per_real_input_label,
            app_per_real_input,
            real_per_app_selector_label,
            real_per_app_unit_selector,
            real_per_app_input_label,
            real_per_app_input,
        }
    }
    pub fn apply_layout(&mut self, layout: &UnitSetupLayout, pos: Position) {
        self.app_per_real_selector_label
            .set_rect(layout.app_per_real_selector_label.with_added_pos(pos));
        self.app_per_real_unit_selector
            .set_rect(layout.app_per_real_unit_selector.with_added_pos(pos));
        self.app_per_real_input_label
            .set_rect(layout.app_per_real_input_label.with_added_pos(pos));
        self.app_per_real_input
            .set_rect(layout.app_per_real_input.with_added_pos(pos));
        self.real_per_app_selector_label
            .set_rect(layout.real_per_app_selector_label.with_added_pos(pos));
        self.real_per_app_unit_selector
            .set_rect(layout.real_per_app_unit_selector.with_added_pos(pos));
        self.real_per_app_input_label
            .set_rect(layout.real_per_app_input_label.with_added_pos(pos));
        self.real_per_app_input
            .set_rect(layout.real_per_app_input.with_added_pos(pos));
    }
    fn app_per_real_change_handler(ui: &RcUi) {
        let mut _p = ui.borrow_mut();
        let p = &mut _p.as_mut().unwrap().unit_setup;
        if let Ok(app_per_real) = p.app_per_real_input.value().parse::<f64>() {
            let app_per_real_unit = p.app_per_real_unit_selector.value().try_into().unwrap();
            let real_per_app_unit = p.real_per_app_unit_selector.value().try_into().unwrap();
            let app_per_real = length_from_unit(app_per_real, app_per_real_unit);
            let real_per_app = 1.0 / convert_units(app_per_real, real_per_app_unit);
            p.real_per_app_input.set_value(&friendly_ftoa(real_per_app));

            drop(_p);
            OutputTabs::update(ui);
        }
    }
    fn real_per_app_change_handler(ui: &RcUi) {
        let mut _p = ui.borrow_mut();
        let p = &mut _p.as_mut().unwrap().unit_setup;
        if let Ok(real_per_app) = p.real_per_app_input.value().parse::<f64>() {
            let real_per_app_unit = p.real_per_app_unit_selector.value().try_into().unwrap();
            let app_per_real_unit = p.app_per_real_unit_selector.value().try_into().unwrap();
            let real_per_app = length_from_unit(real_per_app, real_per_app_unit);
            let app_per_real = 1.0 / convert_units(real_per_app, app_per_real_unit);
            p.app_per_real_input.set_value(&friendly_ftoa(app_per_real));

            drop(_p);
            OutputTabs::update(ui);
        }
    }
}
impl LayoutGen<'_> for UnitSetup {
    type Layout = UnitSetupLayout;
    type Arguments = ();

    fn generate_layout(&self, _: Self::Arguments) -> Self::Layout {
        const NUM_LINES: i32 = 2;

        let height_l1;
        let mut width_l1 = GROUP_H_PADDING * 2;

        let app_per_real_selector_label = Rect(
            Position(GROUP_H_PADDING, GROUP_V_PADDING),
            self.app_per_real_selector_label.measure_label().repack(),
        );
        height_l1 = app_per_real_selector_label.h() + ADDED_HEIGHT;
        width_l1 += app_per_real_selector_label.w();

        let app_per_real_unit_selector = Rect(
            app_per_real_selector_label.to_right(5),
            Size(105, height_l1),
        );
        width_l1 += app_per_real_unit_selector.w();

        let app_per_real_input_label = Rect(
            app_per_real_unit_selector.to_right(5),
            self.app_per_real_input_label.measure_label().repack(),
        );
        width_l1 += app_per_real_input_label.w();

        let app_per_real_input = Rect(app_per_real_input_label.to_right(5), Size(70, height_l1));
        width_l1 += app_per_real_input.w();

        let height_l2;
        let mut width_l2 = GROUP_H_PADDING * 2;

        let real_per_app_selector_label = Rect(
            app_per_real_selector_label.to_bottom(LINE_V_PADDING),
            self.real_per_app_selector_label.measure_label().repack(),
        );
        height_l2 = real_per_app_selector_label.h() + ADDED_HEIGHT;
        width_l2 += real_per_app_selector_label.w();

        let real_per_app_unit_selector = Rect(
            real_per_app_selector_label.to_right(5),
            Size(105, height_l2),
        );
        width_l2 += real_per_app_unit_selector.w();

        let real_per_app_input_label = Rect(
            real_per_app_unit_selector.to_right(0),
            self.real_per_app_input_label.measure_label().repack(),
        );
        width_l2 += real_per_app_input_label.w();

        let real_per_app_input = Rect(real_per_app_input_label.to_right(5), Size(70, height_l2));
        width_l2 += real_per_app_input.w();

        // Snatch the equivalent code from monitor_properties.rs if this pane
        // gets more than two lines.
        let total_width = max(width_l1, width_l2);
        let total_height =
            height_l1 + height_l2 + LINE_V_PADDING * (NUM_LINES - 1) + GROUP_V_PADDING * 2;
        let total_size = Size(total_width, total_height);
        UnitSetupLayout {
            total_size,
            app_per_real_selector_label,
            app_per_real_unit_selector,
            app_per_real_input_label,
            app_per_real_input,
            real_per_app_selector_label,
            real_per_app_unit_selector,
            real_per_app_input_label,
            real_per_app_input,
        }
    }
}
make_layout!(pub UnitSetupLayout, has
    app_per_real_selector_label,
    app_per_real_unit_selector,
    app_per_real_input_label,
    app_per_real_input,
    real_per_app_selector_label,
    real_per_app_unit_selector,
    real_per_app_input_label,
    real_per_app_input,
);
