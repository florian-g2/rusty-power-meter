use std::fmt::Display;
use sml_rs::parser::OctetStr;

/// A code as defined in [OBIS][obis]
///
/// See [here][obiscode] for a description of OBIS Codes.
///
/// [obis]: https://de.wikipedia.org/wiki/OBIS-Kennzahlen
/// [obiscode]: https://onemeter.com/docs/device/obis/
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObisCode {
    inner: [u8; 5],
}

impl Display for ObisCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}-{}:{}.{}.{}",
            self.inner[0], self.inner[1], self.inner[2], self.inner[3], self.inner[4]
        )
    }
}

impl Default for ObisCode {
    fn default() -> Self {
        Self { inner: [0; 5] }
    }
}

impl ObisCode {
    /// Parses an OBIS code from a string such as `&[1, 2, 3, 4, 5, 255]`.
    ///
    /// Panics when the input doesn't contain a valid octet string.
    ///
    /// This function is designed to be used in constant contexts, where it will
    /// fail to compile if the provided input isn't valid. For parsing OBIS codes
    /// at runtime, see `ObisCode::try_from`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sml_rs::application::ObisCode;
    /// const OBIS_CODE: ObisCode = ObisCode::from_octet_str(&[1, 2, 3, 4, 5, 255]);
    /// assert_eq!(&format!("{OBIS_CODE}"), "1-2:3.4.5");
    pub const fn from_octet_str(value: OctetStr<'static>) -> Self {
        match Self::try_from_octet_str(value) {
            Ok(x) => x,
            Err(e) => e.panic(),
        }
    }

    /// Parses an OBIS code from a string such as `"1-0:1.8.0"`.
    ///
    /// Panics when the input doesn't contain a valid string.
    ///
    /// This function is designed to be used in constant contexts, where it will
    /// fail to compile if the provided input isn't valid. For parsing OBIS codes
    /// at runtime, see `ObisCode::try_from`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sml_rs::application::ObisCode;
    /// const OBIS_CODE: ObisCode = ObisCode::from_str("1-2:3.4.5");
    /// assert_eq!(&format!("{OBIS_CODE}"), "1-2:3.4.5");
    /// ```
    pub const fn from_str(s: &'static str) -> Self {
        match Self::try_from_str(s) {
            Ok(x) => x,
            Err(e) => e.panic(),
        }
    }

    /// Views this Obis code as a slice of bytes.
    pub const fn as_bytes(&self) -> &[u8; 5] {
        &self.inner
    }

    const fn try_from_str(s: &str) -> Result<Self, ObisParseError> {
        const SEPARATORS: &[u8; 4] = b"-:..";
        let bytes = s.as_bytes();
        let mut vals = [0u8; 5];
        let mut idx = 0;
        let mut val_idx = 0;
        while idx < bytes.len() {
            match bytes[idx] {
                b'0'..=b'9' => {
                    let n = bytes[idx] - b'0';
                    let Some(val) = vals[val_idx].checked_mul(10) else {
                        return Err(ObisParseError::Overflow);
                    };
                    let Some(val) = val.checked_add(n) else {
                        return Err(ObisParseError::Overflow);
                    };
                    vals[val_idx] = val;
                }
                b if val_idx < SEPARATORS.len() && SEPARATORS[val_idx] == b => {
                    val_idx += 1;
                }
                _ => {
                    return Err(ObisParseError::UnexpectedSeparator);
                }
            }
            idx += 1;
        }

        Ok(ObisCode { inner: vals })
    }

    pub const fn try_from_octet_str(value: OctetStr<'_>) -> Result<Self, ObisParseError> {
        if value.len() != 6 {
            return Err(ObisParseError::InvalidLength);
        }
        // if value[5] != 255 {
        //     return Err(ObisParseError::InvalidLastByte);
        // }
        // doesn't look nice, but also works in const contexts
        let mut vals = [0u8; 5];
        let mut idx = 0;
        while idx < 5 {
            vals[idx] = value[idx];
            idx += 1;
        }
        Ok(ObisCode { inner: vals })
    }
}

/// The error type returned when parsing an [`ObisCode`] from another type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObisParseError {
    /// A value in the obis string contained number that's too large (>255)
    Overflow,
    /// An unexpected separator was parsed
    UnexpectedSeparator,
    /// Provided octet string has invalid length
    InvalidLength,
    /// Provided octet string's last byte doesn't equal 255
    InvalidLastByte,
}

impl ObisParseError {
    const fn panic(self) -> ! {
        match self {
            ObisParseError::Overflow => panic!("Overflow"),
            ObisParseError::UnexpectedSeparator => panic!("Unexpected separator"),
            ObisParseError::InvalidLength => panic!("Invalid input length. Expected 6 bytes."),
            ObisParseError::InvalidLastByte => {
                panic!("Invalid input. Expected the last byte to contain the value 255.")
            }
        }
    }
}

impl core::convert::TryFrom<&str> for ObisCode {
    type Error = ObisParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(value)
    }
}

impl core::convert::TryFrom<OctetStr<'_>> for ObisCode {
    type Error = ObisParseError;

    fn try_from(value: OctetStr<'_>) -> Result<Self, Self::Error> {
        Self::try_from_octet_str(value)
    }
}

impl core::convert::TryFrom<&[u8; 6]> for ObisCode {
    type Error = ObisParseError;

    fn try_from(value: &[u8; 6]) -> Result<Self, Self::Error> {
        Self::try_from_octet_str(value.as_slice())
    }
}
