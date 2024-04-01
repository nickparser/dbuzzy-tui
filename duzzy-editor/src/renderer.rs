use crossterm::cursor::SetCursorStyle;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};
use ropey::RopeSlice;

use crate::{
    buffer::Mode,
    editor::DuzzyEditor,
    selection::{SelectedRange, SelectionSpan, SpanIterator, SpanKind},
};

#[derive(Default)]
pub(super) struct Viewport {
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn update(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
}

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventOutcome {
    Render,
    Ignore,
    Exit,
}

impl Cursor {
    pub const fn style(&self) -> SetCursorStyle {
        match self.mode {
            Mode::Insert => SetCursorStyle::BlinkingBar,
            Mode::Normal | Mode::Visual => SetCursorStyle::BlinkingBlock,
        }
    }
}

pub struct Renderer<'a>(&'a DuzzyEditor);

impl<'a> From<SelectionSpan<'a>> for Span<'a> {
    fn from(span: SelectionSpan<'a>) -> Self {
        let mut style = Style::default();

        if span.kind == SpanKind::Selection {
            style = style.bg(Color::Gray);
        }

        ratatui::text::Span::styled(span.slice, style)
    }
}

impl<'a> Renderer<'a> {
    pub const fn new(editor: &'a DuzzyEditor) -> Self {
        Self(editor)
    }

    fn line(
        line_idx: usize,
        max_len: usize,
        line: RopeSlice<'_>,
        selection: Option<SelectedRange>,
    ) -> Line<'_> {
        let Some((start, end)) = selection else {
            return Line::raw(line);
        };

        if start == end {
            return Line::raw(line);
        }

        let overlaps = start <= line_idx + max_len && line_idx <= end;
        let selection_start = start.saturating_sub(line_idx).min(max_len);
        let selection_end = end.saturating_sub(line_idx).min(max_len);

        if overlaps {
            let range = (selection_start, selection_end);
            let spans = SpanIterator::new(line, range)
                .map(Into::<Span>::into)
                .collect::<Vec<_>>();

            return Line::from(spans);
        }

        Line::raw(line)
    }

    #[inline]
    pub fn text(&self) -> Option<Text> {
        let buf = self.0.workspace.curr().buf();

        let text = buf.text();
        let viewport = self.0.viewport();
        let selection = buf.selection().map(|s| s.range());

        let vscroll = buf.vscroll();
        let max_y = viewport.1.min(text.len_lines());

        let mut lines = Vec::with_capacity(max_y);
        for y in 0..max_y {
            let index = y + vscroll;
            let line = text.line(index);

            let line_idx = text.line_to_char(index);
            let max_len = viewport.0.min(line.len_chars());

            lines.push(Self::line(line_idx, max_len, line, selection));
        }

        Some(Text::from(lines))
    }
}

impl<'a> Widget for Renderer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.text() {
            Some(text) => {
                let inner = Paragraph::new(text);
                inner.render(area, buf);
            }
            None => log::warn!("nothing to render!"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_renderer_lines() {}
}
