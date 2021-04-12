//! Library and GUI tool for calculating optimal first-person 3D view parameters from monitor size and distance.
//!
//! The "portal-like" mode will perform the necessary trigonometry and output the configuration for the camera which will allow the screen to appear as if it is a portal into the rendered 3D world, improving perception of depth and realism.
//!
//! The "focused" mode will output an FOV for the camera which will represent an accurate scale of objects at a given distance in the 3D world from the camera.
//!
//! # License
//! The code itself is dual-licensed under the MIT or Apache 2.0 licenses, at your option.
//!
//! The icon is the eye emoji from [Twemoji], licensed under [CC-BY 4.0].
//!
//! [Twemoji]: https://twemoji.twitter.com/ " "
//! [CC-BY 4.0]: https://creativecommons.org/licenses/by/4.0/ " "

#![no_std]
#![forbid(unsafe_code, rust_2018_idioms)]
#![warn(missing_docs)]

mod aspect;
pub use aspect::*;

use core::fmt::{self, Debug, Formatter};
use uom::{
    si::{
        f64::{Angle, Length},
        length::meter,
    },
    typenum::*,
};

/// Measurements of the monitor dimensions and position.
#[derive(Copy, Clone, Debug)]
pub struct MonitorConfiguration {
    /// The dimensions of the monitor. Those contain the width and height, diagonal length and aspect ratio of the monitor.
    pub dimensions: MonitorDimensions,
    /// The distance at which the viewer is said to be located from the monitor's surface.
    pub distance: Length,
}
impl MonitorConfiguration {
    /// Calculates the viewing angle from the viewpoint towards the monitor.
    ///
    /// More exactly, this is the angle at the viewpoint vertex of a triangle constructed from the screen width as a line segment and two line segments between two verticies of the screen width line segment and the viewpoint vertex.
    pub fn fov(self) -> Angle {
        // Opposite catet, which is half the width of the screen
        let opposite = self.dimensions.width_and_height()[0] / 2.0;
        // Adjacent catet, the distance to the screen
        let adjacent = self.distance;
        // Find the angle by the ratio of the opposite catet to the adjacent
        // catet — exactly what the arctangent function does
        let half_angle = (opposite / adjacent).atan();
        // Double the angle because we split the viewer-to-screen triangle
        // into two right-angled triangles and got the desired angle of
        // one of them, hence we get the full angle by multiplying by two
        half_angle * 2.0
    }
    /// Calculates an FOV for the monitor as the starting point such that a given distance (either relative to the eye or the monitor) will be represented with accurate scale.
    pub fn monitor_fov_for_distance(self, distance: Length, relative_to_monitor: bool) -> Angle {
        let distance_from_eye = if relative_to_monitor {
            distance + self.distance
        } else {
            distance
        };
        // Eye-to-monitor FOV divided by 2 will give us a right-angled triangle to work with
        let base_half_angle = self.fov() / 2.0;
        // The tangent of an angle is the opposite catet divided by the adjacent catet...
        let half_width_over_distance = base_half_angle.tan();
        // ...so multiplying that by the adjacent catet will give us the opposite catet
        let half_width = half_width_over_distance * distance_from_eye;
        // Now we get the tangent of the (half of the) monitor-relative FOV we're trying to find
        let final_angle_tangent = half_width / distance;
        // The arctangent will give us the angle from a known tangent...
        let half_final_angle = final_angle_tangent.atan();
        // ...so we can multiply it by 2 to get the final monitor-relative FOV
        half_final_angle * 2.0
    }
}

/// The dimensions of a monitor.
#[derive(Copy, Clone)]
pub enum MonitorDimensions {
    /// Dimensions expressed directly as the width and the height.
    #[allow(missing_docs)] // Field names are self-explanatory
    WidthAndHeight { width: Length, height: Length },
    /// Dimensions expressed indirectly as the diagonal and the aspect ratio.
    DiagonalAndAspect {
        /// The length of the (imaginary) line drawn between two opposite (top left and bottom right, for example) corners of the monitor.
        diagonal: Length,
        /// The aspect ratio, i.e. `width / height`.
        aspect: f64,
    },
}
impl MonitorDimensions {
    /// Returns the width and height of the monitor, calculating them indirectly if necessary.
    pub fn width_and_height(self) -> [Length; 2] {
        match self {
            Self::WidthAndHeight { width, height } => [width, height],
            Self::DiagonalAndAspect { diagonal, aspect } => {
                Self::diagonal_and_aspect_to_width_and_height(diagonal, aspect)
            }
        }
    }
    /// Returns the aspect ratio of the monitor, calculating it if necessary.
    pub fn aspect(self) -> f64 {
        match self {
            Self::WidthAndHeight { width, height } => width.get::<meter>() / height.get::<meter>(),
            Self::DiagonalAndAspect { aspect, .. } => aspect,
        }
    }
    /// Returns the length of the diagonal of the monitor, calculating it if necessary.
    pub fn diagonal(self) -> Length {
        match self {
            Self::WidthAndHeight { width, height } => {
                // Pythagorean theorem, reinterpret width and height as catet
                // lengths and calculate hypotenuse
                (width.powi(P2::new()) + height.powi(P2::new())).sqrt()
            }
            Self::DiagonalAndAspect { diagonal, .. } => diagonal,
        }
    }
    fn diagonal_and_aspect_to_width_and_height(diagonal: Length, aspect: f64) -> [Length; 2] {
        // An explanation of the math can be found here: https://math.stackexchange.com/a/63690.
        // The only difference between the formula used there and the calculations performed here
        // is that we have an aspect ratio rather than m and n forming the fraction m/n, meaning
        // that all uses of m are substituted by the aspect ratio and all uses of n are substituted
        // by 1.
        let height = diagonal / (aspect.powi(2) + 1.0).sqrt();
        let width = height * aspect;
        [width, height]
    }
    /// Re-represents the dimensions as the `WidthAndHeight` variant.
    pub fn as_width_and_height(self) -> Self {
        let [width, height] = self.width_and_height();
        Self::WidthAndHeight { width, height }
    }
    /// Re-represents the dimensions as the `DiagonalAndAspect` variant.
    pub fn as_diagonal_and_aspect(self) -> Self {
        Self::DiagonalAndAspect {
            diagonal: self.diagonal(),
            aspect: self.aspect(),
        }
    }
}
impl Debug for MonitorDimensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [width, height] = self.width_and_height();
        #[derive(Debug)]
        enum StoredAs {
            WidthAndHeight,
            DiagonalAndAspect,
        }
        let stored_as = match self {
            Self::WidthAndHeight { .. } => StoredAs::WidthAndHeight,
            Self::DiagonalAndAspect { .. } => StoredAs::DiagonalAndAspect,
        };
        f.debug_struct("MonitorDimensions")
            .field("width", &width)
            .field("height", &height)
            .field("aspect", &self.aspect())
            .field("diagonal", &self.diagonal())
            .field("stored_as", &stored_as)
            .finish()
    }
}
