use crate::vga::{column::Column, row::Row};

#[derive(Default, Debug, Clone, Copy)]
pub struct Cursor {
    row: Row,
    column: Column,
}

// TODO! Setting the cursor in asm to column 0 on line 2 (position 0x0050 = 80 in decimal)
/*
out 0x3D4, 14      ; 14 tells the framebuffer to expect the highest 8 bits of the position
out 0x3D5, 0x00    ; sending the highest 8 bits of 0x0050
out 0x3D4, 15      ; 15 tells the framebuffer to expect the lowest 8 bits of the position
out 0x3D5, 0x50    ; sending the lowest 8 bits of 0x0050
*/

impl Cursor {
    pub const MIN_ROW: u8 = Row::MIN;
    pub const MAX_ROW: u8 = Row::MAX;
    pub const MIN_COLUMN: u8 = Column::MIN;
    pub const MAX_COLUMN: u8 = Column::MAX;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(row: Row, column: Column) -> Result<Self, ()> {
        Ok(Self { row, column })
    }

    pub fn row(&self) -> Row {
        self.row
    }

    pub fn column(&self) -> Column {
        self.column
    }

    /// Get the current row as a u8
    pub fn row_u8(&self) -> u8 {
        self.row.value()
    }

    /// Get the current column as a u8
    pub fn column_u8(&self) -> u8 {
        self.column.value()
    }

    /// Set the cursor position
    pub fn set_position(&mut self, row: u8, column: u8) -> Result<(), ()> {
        self.row = Row::try_from(row)?;
        self.column = Column::try_from(column)?;
        Ok(())
    }

    /// Move cursor to the start of the current line
    pub fn carriage_return(&mut self) {
        self.column = Column::default();
    }

    /// Move cursor to the next line, same column
    pub fn line_feed(&mut self) {
        if self.row_u8() < Row::MAX - 1 {
            self.row = Row::try_from(self.row_u8() + 1).unwrap();
        }
    }

    /// Move cursor to next line and beginning (like \n)
    pub fn newline(&mut self) {
        self.carriage_return();
        self.line_feed();
    }

    /// Advance cursor by one position, wrapping to next line if needed
    pub fn advance(&mut self) {
        if self.column_u8() < Column::MAX - 1 {
            self.column = Column::try_from(self.column_u8() + 1).unwrap();
        } else {
            // Wrap to next line
            self.carriage_return();
            self.line_feed();
        }
    }

    /// Move cursor back one position
    pub fn backspace(&mut self) {
        if self.column_u8() > 0 {
            self.column = Column::try_from(self.column_u8() - 1).unwrap();
        } else if self.row_u8() > 0 {
            // Move to end of previous line
            self.row = Row::try_from(self.row_u8() - 1).unwrap();
            self.column = Column::try_from(Column::MAX - 1).unwrap();
        }
    }

    /// Move cursor up one row
    pub fn move_up(&mut self) {
        if self.row_u8() > 0 {
            self.row = Row::try_from(self.row_u8() - 1).unwrap();
        }
    }

    /// Move cursor down one row
    pub fn move_down(&mut self) {
        if self.row_u8() < Row::MAX - 1 {
            self.row = Row::try_from(self.row_u8() + 1).unwrap();
        }
    }

    /// Move cursor left one column
    pub fn move_left(&mut self) {
        if self.column_u8() > 0 {
            self.column = Column::try_from(self.column_u8() - 1).unwrap();
        }
    }

    /// Move cursor right one column
    pub fn move_right(&mut self) {
        if self.column_u8() < Column::MAX - 1 {
            self.column = Column::try_from(self.column_u8() + 1).unwrap();
        }
    }

    /// Reset cursor to (0, 0)
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Check if cursor is at the start of a line
    pub fn is_at_line_start(&self) -> bool {
        self.column_u8() == 0
    }

    /// Check if cursor is at the end of a line
    pub fn is_at_line_end(&self) -> bool {
        self.column_u8() == Column::MAX - 1
    }

    /// Check if cursor is on the last line
    pub fn is_on_last_line(&self) -> bool {
        self.row_u8() == Row::MAX - 1
    }
}
