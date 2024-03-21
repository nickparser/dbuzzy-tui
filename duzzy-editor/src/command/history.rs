use crate::{
    doc_mut,
    editor::Workspace,
    history::{ChangeKind, History},
    set_cursor,
};

pub(super) fn undo(ws: &mut Workspace) {
    let (buf, history) = doc_mut!(ws);

    if let Some(pos) = history.undo(&mut buf.text) {
        set_cursor!(buf, buf.cursor_pos(pos));
    }
}

pub(super) fn redo(ws: &mut Workspace) {
    let (buf, history) = doc_mut!(ws);

    if let Some(pos) = history.redo(&mut buf.text) {
        set_cursor!(buf, buf.cursor_pos(pos));
    }
}

pub(super) fn insert_char(ch: char, pos: usize, history: &mut History) {
    history.push(ChangeKind::Insert, pos, |change| {
        change.on_char(ch, false).keep()
    });
}

pub(super) fn delete_char(ch: char, pos: usize, history: &mut History) {
    history.push(ChangeKind::Delete, pos, |change| {
        change.on_char(ch, false).keep()
    });
}

pub(super) fn delete_char_inplace(ch: char, pos: usize, history: &mut History) {
    history.push(ChangeKind::Delete, pos, |change| {
        change.on_char(ch, true).commit()
    });
}
