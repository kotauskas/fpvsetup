use fpvsetup::MonitorDimensions;
use std::io::{self, Cursor, ErrorKind};
use uom::si::{f64::Length, length::centimeter};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::MonitorEdids;

pub fn find_any_monitor_dimensions() -> io::Result<MonitorDimensions> {
    #[cfg(windows)]
    {
        for edid in MonitorEdids::new()? {
            let edid = match edid {
                Ok(edid) => edid,
                Err(..) => continue,
            };
            let mut cursor = Cursor::new(edid);
            let parsed_edid = match edid::parse(&mut cursor) {
                Ok(p) => p,
                Err(..) => continue,
            };
            let edid::ImageSize { width, height } =
                if let Some(max_size) = parsed_edid.display.max_size {
                    max_size
                } else {
                    continue;
                };
            let width = Length::new::<centimeter>(width as _);
            let height = Length::new::<centimeter>(height as _);
            return Ok(MonitorDimensions::WidthAndHeight { width, height });
        }
        Err(io::Error::new(
            ErrorKind::NotFound,
            "no suitable EDID found",
        ))
    }
    #[cfg(not(windows))]
    {
        Err(io::Error::new(
            ErrorKind::Other,
            "not yet implemented on this platform",
        ))
    }
}
