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

macro_rules! impl_from_for_row {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<Row> for $t {
                #[inline]
                fn from(column: Row) -> Self {
                    column.0 as $t
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
    #[inline]
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

    pub fn advance_truncated(&mut self) -> bool {
        if self.0 < Self::MAX - 1 {
            self.0 += 1;
            false
        } else {
            true
        }
    }

    /// Returns if the counter 'wrapped arround' or not
    pub fn advance_wrap(&mut self) -> bool {
        if self.0 < Self::MAX - 1 {
            self.0 += 1;
            false
        } else {
            self.0 = 0;
            true
        }
    }
}

impl_try_from_for_row!(u8, i8, u16, i16, u32, i32, u64, i64);
impl_from_for_row!(u8, i8, u16, i16, u32, i32, u64, i64);
