#![forbid(rust_2018_idioms)]
#![cfg_attr(not(windows), forbid(unsafe_code))]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fltk::{
    app::{App, Scheme},
    image::PngImage,
    input::FloatInput,
    menu::{Choice, MenuExt, MenuFlag},
    window::{WidgetExt, Window},
    GroupExt, InputExt, Shortcut, WindowExt,
};
use fpvsetup::MonitorDimensions;
use native_dialog::{MessageDialog, MessageType};
use std::{
    cell::{Cell, RefCell},
    cmp::max,
    convert::TryFrom,
    panic::{self, PanicInfo},
    process,
    rc::Rc,
    thread,
};

#[macro_use]
mod layout;
mod focused;
mod monitor_properties;
mod monitors;
mod output_tabs;
mod portal_like;
mod unit_setup;
mod util;
use {
    focused::*, layout::*, monitor_properties::*, monitors::*, output_tabs::*, portal_like::*,
    unit_setup::*, util::*,
};

/// The horizontal padding of the widget group as a whole.
const GROUP_H_PADDING: i32 = 7;
/// The vertical padding of the widget group as a whole.
const GROUP_V_PADDING: i32 = 7;
/// How much height to add over the automatically calculated label height.
const ADDED_HEIGHT: i32 = 4;
/// The vertical padding between lines.
const LINE_V_PADDING: i32 = 10;

const OPTIMAL_SCHEME: Scheme = {
    if cfg!(target_os = "macos") {
        Scheme::Plastic
    } else {
        Scheme::Gtk
    }
};

static ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon/icon.png"));

fn main() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| panic_hook(info, &default_hook)));
    let monitor_dimensions = find_any_monitor_dimensions().ok();
    let mut app = App::default();
    app.set_scheme(OPTIMAL_SCHEME);
    let mut window = Window::default().with_label("FPVSetup");
    let icon = PngImage::from_data(ICON);
    if let Ok(icon) = icon {
        window.set_icon(Some(icon));
    }
    let Size(width, height) = build_ui(monitor_dimensions);
    window.end();
    window.set_size(width, height);
    // this is why you shouldn't have a struct as a builder of itself
    window = window.center_screen();
    window.show();
    if let Err(error) = app.run() {
        eprintln!("Fatal error: {:?}", error);
    }
}
#[derive(Clone)]
pub struct Ui {
    monitor_properties: MonitorProperties,
    unit_setup: UnitSetup,
    output_tabs: OutputTabs,
}
pub type RcUi = Rc<RefCell<Option<Ui>>>;
impl Ui {
    #[allow(clippy::new_without_default)] // Not using it
    pub fn new(monitor_dimensions: Option<MonitorDimensions>) -> Self {
        let whole_ui = Rc::new(RefCell::new(None));
        let monitor_properties = MonitorProperties::new(&whole_ui, monitor_dimensions);
        let unit_setup = UnitSetup::new(&whole_ui);
        let output_tabs = OutputTabs::new(&whole_ui);
        let built = Self {
            monitor_properties,
            unit_setup,
            output_tabs,
        };
        *whole_ui.borrow_mut() = Some(built.clone());
        if monitor_dimensions.is_some() {
            MonitorProperties::width_or_height_change_handler(&whole_ui);
        }
        built
    }
    pub fn apply_layout(
        &mut self,
        layout: &UiLayout,
        monitor_properties_layout: &MonitorPropertiesLayout,
        unit_setup_layout: &UnitSetupLayout,
        output_tabs_layout: &OutputTabsLayout,
        portal_like_layout: &PortalLikeLayout,
        focused_layout: &FocusedLayout,
    ) {
        self.monitor_properties
            .apply_layout(monitor_properties_layout, layout.monitor_properties.pos());
        self.unit_setup
            .apply_layout(unit_setup_layout, layout.unit_setup.pos());
        self.output_tabs.apply_layout(
            output_tabs_layout,
            portal_like_layout,
            focused_layout,
            layout.output_tabs.pos(),
        );
    }
}
impl<'a> LayoutGen<'a> for Ui {
    type Layout = UiLayout;
    type Arguments = (
        &'a MonitorPropertiesLayout,
        &'a UnitSetupLayout,
        &'a OutputTabsLayout,
    );

    fn generate_layout(
        &self,
        (monitor_properties_layout, unit_setup_layout, output_tabs_layout): Self::Arguments,
    ) -> Self::Layout {
        let mut height = 0;
        let monitor_properties = Rect(Position(0, 0), monitor_properties_layout.total_size);
        height += monitor_properties.h();
        let unit_setup = Rect(
            monitor_properties.to_bottom(0),
            unit_setup_layout.total_size,
        );
        height += unit_setup.h();
        let output_tabs = Rect(unit_setup.to_bottom(0), output_tabs_layout.total_size);
        height += output_tabs.h();
        let width = [monitor_properties.w(), unit_setup.w(), output_tabs.w()]
            .iter()
            .copied()
            .max()
            .unwrap();
        let total_size = Size(width, height);
        UiLayout {
            total_size,
            monitor_properties,
            unit_setup,
            output_tabs,
        }
    }
}
make_layout!(pub UiLayout, has monitor_properties, unit_setup, output_tabs);

fn build_ui(monitor_dimensions: Option<MonitorDimensions>) -> Size {
    let mut ui = Ui::new(monitor_dimensions);
    let monitor_properties_layout = ui.monitor_properties.generate_layout(());
    let unit_setup_layout = ui.unit_setup.generate_layout(());
    let portal_like_layout = ui.output_tabs.portal_like.generate_layout(());
    let focused_layout = ui.output_tabs.focused.generate_layout(());
    let fill_width = max(
        monitor_properties_layout.total_size.w(),
        unit_setup_layout.total_size.w(),
    );
    let output_tabs_layout =
        ui.output_tabs
            .generate_layout((&portal_like_layout, &focused_layout, fill_width));
    let ui_layout = ui.generate_layout((
        &monitor_properties_layout,
        &unit_setup_layout,
        &output_tabs_layout,
    ));
    ui.apply_layout(
        &ui_layout,
        &monitor_properties_layout,
        &unit_setup_layout,
        &output_tabs_layout,
        &portal_like_layout,
        &focused_layout,
    );
    ui_layout.total_size
}

fn build_unit_selector(
    input_field: &FloatInput,
    default: Option<Unit>,
    number: Number,
    invert: bool,
) -> Choice {
    let mut selector = Choice::default();
    let prev_rc = Rc::new(Cell::new(0));
    let mut counter = 0;
    let mut add_entry = |singular, plural| {
        let prev_c = Rc::clone(&prev_rc);
        let input_c = input_field.clone();
        let index = counter;
        let label = match number {
            Singular => singular,
            Plural => plural,
        };
        selector.add(label, Shortcut::empty(), MenuFlag::Normal, move || {
            if let Some(old_val) = float_from_restricted_string(&input_c.value()) {
                let prev = Unit::try_from(prev_c.get()).unwrap();
                let new = Unit::try_from(index).unwrap();
                let mul = if !invert {
                    conversion_rate(prev, new)
                } else {
                    conversion_rate(new, prev)
                };
                let new_val = old_val * mul;
                input_c.set_value(&friendly_ftoa(new_val));
            }
            prev_c.set(index); // Remember current state for later
        });
        counter += 1;
    };
    add_entry("meter", "meters");
    add_entry("centimeter", "centimeters");
    add_entry("foot", "feet");
    add_entry("inch", "inches");
    if let Some(default) = default {
        selector.set_value(default.into());
    }
    selector
}
#[derive(Copy, Clone)]
enum Number {
    Singular,
    Plural,
}
use Number::*;

fn panic_hook(info: &PanicInfo<'_>, default_hook: &dyn Fn(&PanicInfo<'_>)) {
    default_hook(info);
    let current_thread = thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>");
    let message = format!("Thread '{}' {}", thread_name, info);
    MessageDialog::new()
        .set_title("FPVSetup — fatal error")
        .set_text(&message)
        .set_type(MessageType::Error)
        .show_alert()
        .unwrap();
    process::abort();
}
