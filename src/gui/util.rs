use crate::layout::{Position, Rect, Size};
use fltk::{draw, window::WidgetExt, WindowExt};
use std::{borrow::Cow, convert::TryFrom, num::FpCategory};
use uom::{
    si::{
        f64::Length,
        length::{centimeter, foot, inch, meter},
    },
    Conversion,
};

pub static DEGREE_SIGN: &str = "°";
pub static NAN_FTOA: &str = "<error>";
pub static INFINITY_FTOA: &str = "∞";

/// Converts a string which can only either be empty or parsable to a float into an `Option<f64>`.
pub fn float_from_restricted_string(src: &str) -> Option<f64> {
    let src = src.trim();
    if src.is_empty() {
        None
    } else {
        Some(src.parse().unwrap())
    }
}

/// Converts a float to a string in a friendly representation.
pub fn friendly_ftoa(val: f64) -> Cow<'static, str> {
    match val.classify() {
        FpCategory::Nan => Cow::Borrowed(NAN_FTOA),
        FpCategory::Infinite => {
            let sign = if val.is_sign_negative() { "-" } else { "" };
            Cow::Owned(format!("{}{}", sign, INFINITY_FTOA))
        }
        _ => Cow::Owned(friendly_ftoa_base(val)),
    }
}
fn friendly_ftoa_base(val: f64) -> String {
    let mut formatted = format!("{:.3}", val);
    let mut chars_to_pop = 0;
    for c in formatted.chars().rev() {
        if c == '0' {
            chars_to_pop += 1;
        } else {
            if c == '.' {
                chars_to_pop += 1;
            }
            break;
        }
    }
    formatted.truncate(formatted.len() - chars_to_pop);
    formatted
}

pub fn length_from_unit(val: f64, unit: Unit) -> Length {
    match unit {
        Unit::Meters => Length::new::<meter>(val),
        Unit::Centimeters => Length::new::<centimeter>(val),
        Unit::Feet => Length::new::<foot>(val),
        Unit::Inches => Length::new::<inch>(val),
    }
}

pub fn convert_units(val: Length, b: Unit) -> f64 {
    match b {
        Unit::Meters => val.get::<meter>(),
        Unit::Centimeters => val.get::<centimeter>(),
        Unit::Feet => val.get::<foot>(),
        Unit::Inches => val.get::<inch>(),
    }
}

/// By what number a value of unit A needs to multiplied to yield unit B.
pub fn conversion_rate(a: Unit, b: Unit) -> f64 {
    let u2f = |unit| match unit {
        Unit::Meters => <meter as Conversion<f64>>::coefficient(),
        Unit::Centimeters => <centimeter as Conversion<f64>>::coefficient(),
        Unit::Feet => <foot as Conversion<f64>>::coefficient(),
        Unit::Inches => <inch as Conversion<f64>>::coefficient(),
    };
    let (a, b) = (u2f(a), u2f(b));
    b / a
}

#[repr(i32)]
pub enum Unit {
    Meters = 0,
    Centimeters = 1,
    Feet = 2,
    Inches = 3,
}
impl From<Unit> for i32 {
    fn from(val: Unit) -> Self {
        val as i32
    }
}
impl TryFrom<i32> for Unit {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let ok = match value {
            0 => Self::Meters,
            1 => Self::Centimeters,
            2 => Self::Feet,
            3 => Self::Inches,
            _ => return Err(()),
        };
        Ok(ok)
    }
}

/// Extension trait for automatically adjusting the width of a label to the width of the contained text.
pub trait AutoLabelExt: WidgetExt + Sized {
    /// Sets the label and width of the widget. *Assumes that the current font and font size of the `draw` subsystem is the one used for the widget, for performance.*
    fn set_auto_label(&mut self, label: &str) {
        let width = draw::width(label).ceil() as i32;
        self.set_size(width, self.h());
        self.set_label(label);
    }
    /// Sets the label and width of the widget with chain-call support. *Assumes that the current font and font size of the `draw` subsystem is the one used for the widget, for performance.*
    fn with_auto_label(mut self, label: &str) -> Self {
        self.set_auto_label(label);
        self
    }
}
impl<T: WidgetExt> AutoLabelExt for T {}

pub trait PosExt: WidgetExt {
    fn set_rect(&mut self, Rect(Position(x, y), Size(w, h)): Rect) {
        self.set_size(w, h);
        self.set_pos(x, y);
    }
    fn rect(&self) -> Rect {
        Rect(Position(self.x(), self.y()), Size(self.w(), self.h()))
    }
    fn set_center_screen(&self)
    where
        Self: WindowExt + Clone,
    {
        self.clone().center_screen();
    }
}
impl<T: WidgetExt + ?Sized> PosExt for T {}

pub trait Repack<T>: Sized {
    fn repack(self) -> T;
}
impl<T> Repack<[T; 2]> for (T, T) {
    fn repack(self) -> [T; 2] {
        [self.0, self.1]
    }
}
impl<T> Repack<(T, T)> for [T; 2] {
    fn repack(self) -> (T, T) {
        let [a, b] = self;
        (a, b)
    }
}
