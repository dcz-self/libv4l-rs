use std::{fmt, str};

#[derive(Default, Copy, Clone, Eq)]
/// Four character code representing a pixelformat
pub struct FourCC {
    pub repr: [u8; 4],
}

impl FourCC {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    /// Returns a pixelformat as four character code
    ///
    /// # Arguments
    ///
    /// * `repr` - Four characters as raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use v4l::format::FourCC;
    /// let fourcc = FourCC::new(b"YUYV");
    /// ```
    pub fn new(repr: &[u8; 4]) -> FourCC {
        FourCC { repr: *repr }
    }

    /// Returns the string representation of a four character code
    ///
    /// # Example
    ///
    /// ```
    /// use v4l::format::FourCC;
    /// let fourcc = FourCC::new(b"YUYV");
    /// let str = fourcc.str().unwrap();
    /// ```
    pub fn str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.repr)
    }
}

impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = str::from_utf8(&self.repr);
        if let Ok(string) = string {
            write!(f, "FourCC(")?;
            string.fmt(f)?;
            write!(f, ")")?;
        } else {
            write!(f, "FourCC({:02x} {:02x} {:02x} {:02x})", self.repr[0], self.repr[1], self.repr[2], self.repr[3])?;
        }
        Ok(())
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = str::from_utf8(&self.repr);
        if let Ok(string) = string {
            write!(f, "{}", string)?;
        }
        Ok(())
    }
}

impl PartialEq for FourCC {
    fn eq(&self, other: &FourCC) -> bool {
        self.repr.iter().zip(other.repr.iter()).all(|(a, b)| a == b)
    }
}

impl From<u32> for FourCC {
    fn from(code: u32) -> Self {
        FourCC::new(&code.to_le_bytes())
    }
}

impl From<FourCC> for u32 {
    fn from(fourcc: FourCC) -> Self {
        Self::from_le_bytes(fourcc.repr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_fourcc_string() {
        assert_eq!(format!("{:?}", FourCC::new(b"MJPG")), "FourCC(\"MJPG\")");
    }

    #[test]
    fn debug_fourcc_nonascii() {
        assert_eq!(format!("{:?}", FourCC::new(&[0x01, 0xff, 0x20, 0xcd])), "FourCC(01 ff 20 cd)");
    }
}