macro_rules! generate_common_aspect_ratios {
    ($($n:literal : $d:literal),+ $(,)?) => {
        [$(
            ($n as f64 / $d as f64, [$n as f64, $d as f64]),
        )+]
    };
}

/// A list of commonly used monitor aspect ratios.
///
/// Values are given in the form of a tuple `(ratio, [numerator, denominator])`, all three numbers being `f64`s.
pub static COMMON_ASPECT_RATIOS: &[(f64, [f64; 2])] = &generate_common_aspect_ratios! [
    // Common
    16:9,
    // Not very common
    16:10,
    4:3,
    // Considerably less common
    5:4,
    3:2,
    // Ultrawide gamer ratios
    17:9,
    21:9,
    32:9,
    // Honestly not common at all
    1:1,
    4:1,
];
/// Finds a common aspect ratio for the given single-number ratio, considering the ratio close enough if the difference is less than the given rounding.
pub fn find_common_aspect_ratio(ratio: f64, rounding: f64) -> Option<[f64; 2]> {
    COMMON_ASPECT_RATIOS
        .iter()
        .copied()
        .filter(|(r, _)| (ratio - r).abs() < rounding)
        .map(|(_, [n, d])| [n, d])
        .next()
}
