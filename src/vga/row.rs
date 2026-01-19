use core::convert::TryFrom;

macro_rules! impl_try_from_for_row {
    ($($t:ty),* $(,)?) => {
        $(
            impl TryFrom<$t> for Row {
                type Error = ();

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if value >= Row::MIN as $t && value < Row::MAX as $t {
                        Ok(Row(value as u8))
                    } else {
                        Err(())
                    }
                }
            }
        )*
    };
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Row(u8);

impl Row {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 25;

    /// Get the row value
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Create a new Row (checked)
    pub const fn new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value < Self::MAX {
            Some(Row(value))
        } else {
            None
        }
    }

    pub fn increment(&mut self) -> Result<(), ()> {
        if self.0 < Self::MAX - 1 {
            self.0 += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl_try_from_for_row!(u8, i8, u16, i16, u32, i32, u64, i64);
