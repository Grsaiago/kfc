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

macro_rules! impl_from_for_column {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<Column> for $t {
                #[inline]
                fn from(column: Column) -> Self {
                    column.0 as $t
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
    #[inline]
    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl_try_from_for_column!(u8, i8, u16, i16, u32, i32, u64, i64);

impl_from_for_column!(u8, i8, u16, i16, u32, i32, u64, i64);
