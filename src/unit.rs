use std::fmt::Display;
use serde::Serialize;

/// Units as defined in [DLMS/COSEM][dlms] or [IEC 62056][iec]
///
/// This type only implements the units relevant for (and used by) power meters.
///
/// Specification of the units taken from this [pdf][dlmspdf] ([archive.org][dlmsarchive]).
/// See table on page 47.
///
/// [dlms]: https://www.dlms.com/dlms-cosem
/// [iec]: https://en.wikipedia.org/wiki/IEC_62056
/// [dlmspdf]: https://www.dlms.com/files/Blue-Book-Ed-122-Excerpt.pdf
/// [dlmsarchive]: https://web.archive.org/web/20211130052659/https://www.dlms.com/files/Blue-Book-Ed-122-Excerpt.pdf
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum Unit {
    /// active power `[W]`
    #[serde(alias = "W")]
    Watt,
    /// active energy `[Wh]`
    #[serde(alias = "Wh")]
    WattHour,
    /// voltage `[V]`
    Volt,
    /// current `[A]`
    Ampere,
    /// (phase) angle `[°]`
    Degree,
    /// frequency `[Hz]`
    Hertz,
}

impl Unit {
    /// Returns a string describing the unit (e.g. `"W"` for `Unit::Watt`)
    pub fn as_str(&self) -> &'static str {
        match self {
            Unit::Watt => "W",
            Unit::WattHour => "Wh",
            Unit::Volt => "V",
            Unit::Ampere => "A",
            Unit::Degree => "°",
            Unit::Hertz => "Hz",
        }
    }

    /// Creates a `Unit` instance from a DLMN/COSEM unit number.
    ///
    /// Returns `None` if the given unit number doesn't match one of the supported units.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            8 => Some(Unit::Degree),
            27 => Some(Unit::Watt),
            30 => Some(Unit::WattHour),
            33 => Some(Unit::Ampere),
            35 => Some(Unit::Volt),
            44 => Some(Unit::Hertz),
            _ => None,
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}