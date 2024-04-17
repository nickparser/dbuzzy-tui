use std::borrow::Cow;

use crate::{buffer::Buffer, editor::Workspace};

pub(super) fn select_line(ws: &mut Workspace) {
    let buf = ws.cur_mut().buf_mut();
    let (idx, ofs) = buf.pos();

    if ofs == buf.line_len_chars(idx) - 1 {
        let pos = super::shift_down(1, buf);
        buf.set_pos(pos);
    }

    let idx = buf.index();
    let start = buf.line_byte(idx);

    let ofs = buf.line_len_chars(idx) - 1;
    let end = start + ofs;

    if buf.selection().is_none() {
        buf.new_selection(start);
    }

    buf.update_selection(end);
    buf.set_offset(ofs);
}

pub(super) fn selected_text(buf: &Buffer) -> Option<Cow<str>> {
    let selection = buf.selection()?;
    let (start, mut end) = selection.range();

    if selection.head() > selection.anchor() {
        let len_chars = buf.len_chars();
        end = (end + 1).min(len_chars);
    }

    let slice = buf.text().slice(start..end);

    let text = match slice.as_str() {
        Some(s) => Cow::from(s),
        None => Cow::from(slice.to_string()),
    };

    Some(text)
}

#[cfg(test)]
mod tests {
    use ropey::Rope;

    use crate::document::Document;

    use super::*;

    #[test]
    fn test_select_line() {
        let mut ws = Workspace::default();
        ws.add_doc(Document::default());

        let doc = ws.cur_mut();
        let buf = doc.buf_mut();

        let text = Rope::from("test\ntest");

        buf.set_pos((0, 2));
        buf.set_text(text);

        select_line(&mut ws);

        assert_eq!(ws.cur().buf().pos(), (0, 4));
        assert_eq!(ws.cur().buf().selection().map(|s| s.range()), Some((0, 4)));

        select_line(&mut ws);

        assert_eq!(ws.cur().buf().pos(), (1, 3));
        assert_eq!(ws.cur().buf().selection().map(|s| s.range()), Some((0, 8)));
    }
}
