use crate::{buffer::Content, cursor::CursorMode};

pub(super) fn insert_char(content: &mut Content, ch: char) {
    let pos = content.cursor.position(&content.text);
    content.text.insert_char(pos, ch);

    super::movement::move_forward(content);
}

pub(super) fn new_line(content: &mut Content) {
    let pos = content.cursor.position(&content.text);
    content.text.insert_char(pos, '\n');

    super::movement::move_down(content);
    content.cursor.offset = 0;
}

pub(super) fn delete_char(content: &mut Content) {
    let pos = content.cursor.position(&content.text);
    if pos < content.text.len_chars() {
        content.text.remove(pos..pos + 1);
    }
}

pub(super) fn insert_mode_line_next(content: &mut Content) {
    let line_start_byte = content.text.line_to_byte(content.cursor.index + 1);
    content.text.insert_char(line_start_byte, '\n');

    super::movement::move_down(content);

    content.cursor.offset = 0;
    content.cursor.mode = CursorMode::Insert;
}

pub(super) fn insert_mode_line_prev(content: &mut Content) {
    super::movement::move_up(content);

    let line_start_byte = content.text.line_to_byte(content.cursor.index);
    content.text.insert_char(line_start_byte, '\n');

    content.cursor.offset = 0;
    content.cursor.mode = CursorMode::Insert;
}