use ropey::Rope;

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub offset: usize,
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Self {
            index: pos.0,
            offset: pos.1,
        }
    }
}

impl From<&Position> for (usize, usize) {
    fn from(pos: &Position) -> Self {
        (pos.index, pos.offset)
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub(super) text: Rope,
    pub(super) pos: Position,
    pub(super) mode: CursorMode,

    vscroll: usize,
}

impl Buffer {
    pub fn byte_pos(&self) -> usize {
        let (index, offset) = Into::into(&self.pos);
        offset + self.text.line_to_byte(index)
    }

    pub fn cursor_pos(&self, pos: usize) -> Position {
        let index = self.text.byte_to_line(pos);
        let start = self.text.line_to_byte(index);
        let offset = pos - start;
        (index, offset).into()
    }

    pub const fn vscroll(&self) -> usize {
        self.vscroll
    }

    pub fn update_vscroll(&mut self, max: usize) {
        let index = self.pos.index;
        let upper_bound = self.vscroll + max - 1;

        if index < self.vscroll {
            self.vscroll = index;
        } else if index > upper_bound {
            self.vscroll = index - max + 1;
        }
    }

    pub fn len_bytes(&self, index: usize) -> usize {
        self.text.line(index).len_bytes()
    }

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn is_insert(&self) -> bool {
        self.mode == CursorMode::Insert
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            text: Rope::default(),
            pos: Position::default(),
            vscroll: 0,
            mode: CursorMode::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CursorMode {
    Insert,
    Normal,
    Visual,
}

impl std::fmt::Display for CursorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Insert => write!(f, "insert"),
            Self::Normal => write!(f, "normal"),
            Self::Visual => write!(f, "visual"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::set_cursor;

    use super::*;

    #[test]
    fn test_cursor_macro() {
        let mut buf = Buffer::default();

        set_cursor!(buf, index += 5);
        assert_eq!((5, 0), Into::into(&buf.pos));

        set_cursor!(buf, offset += 10);
        assert_eq!((5, 10), Into::into(&buf.pos));

        set_cursor!(buf, (15, 20).into());
        assert_eq!((15, 20), Into::into(&buf.pos));
    }

    #[test]
    fn test_cursor_pos() {
        let mut buf = Buffer::default();
        buf.text = Rope::from_str("text\n\ntext");

        set_cursor!(buf, buf.cursor_pos(10));
        assert_eq!((2, 4), Into::into(&buf.pos));
    }
}
