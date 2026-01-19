use core::convert::TryFrom;

macro_rules! impl_try_from_for_column {
    ($($t:ty),* $(,)?) => {
        $(
            impl TryFrom<$t> for Column {
                type Error = ();

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if value >= Column::MIN as $t && value < Column::MAX as $t {
                        Ok(Column(value as u8))
                    } else {
                        Err(())
                    }
                }
            }
        )*
    };
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Column(u8);

impl Column {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 80;

    /// Get the column value
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Create a new Column (checked)
    pub const fn new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value < Self::MAX {
            Some(Column(value))
        } else {
            None
        }
    }

    pub fn advance_wrap(&mut self) -> bool {
        if self.0 < Self::MAX - 1 {
            self.0 += 1;
            false // didn't wrap
        } else {
            self.0 = 0;
            true // wrapped
        }
    }
}

impl_try_from_for_column!(u8, i8, u16, i16, u32, i32, u64, i64);
