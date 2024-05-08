use std::io;

use crate::{
    cli::Cli,
    tui::{self, ColorScheme, EvtxTable, PALETTES},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";

pub struct App {
    evtx_table: EvtxTable,
    exit: bool,
    state: TableState,
    scroll_state: ScrollbarState,
    colors: ColorScheme,
}

impl App {
    pub fn new(cli: Cli) -> Self {
        let evtx_table = EvtxTable::try_from(cli.evtx_file.path().path()).unwrap();
        let table_len = evtx_table.len();
        Self {
            evtx_table,
            exit: Default::default(),
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new(table_len - 1),
            colors: ColorScheme::new(&PALETTES[0]),
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let rects =
            Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(frame.size());
        let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rects[0]);
        frame.render_widget(Clear, rects[0]);
        self.render_table(frame, cols[0]);
        self.render_scrollbar(frame, cols[0]);
        self.render_content(frame, cols[1]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(self.evtx_table.table(), area, &mut self.state);
    }
    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        match self.state.selected() {
            Some(i) => match self.evtx_table.content(i) {
                Some(value) => match serde_json::to_string_pretty(value) {
                    Ok(content) => frame.render_widget(Paragraph::new(content), area),
                    Err(why) => frame.render_widget(Paragraph::new(format!("{why}")), area),
                },
                None => frame.render_widget(Clear, area),
            },
            None => frame.render_widget(Clear, area),
        }
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(&Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        )
    }

    fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Line::from(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg())
                    .bg(self.colors.buffer_bg()),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color())),
            );
        frame.render_widget(info_footer, area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('g') => self.set_selected(0),
            KeyCode::Char('G') => self.set_selected(self.evtx_table.len() - 1),
            KeyCode::Down => self.next(1),
            KeyCode::Up => self.previous(1),
            KeyCode::PageDown => self.next(10),
            KeyCode::PageUp => self.previous(10),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn set_selected(&mut self, idx: usize) {
        self.state.select(Some(idx));
        self.scroll_state = self.scroll_state.position(idx);
    }

    fn next(&mut self, steps: usize) {
        assert_ne!(steps, 0);
        if !self.evtx_table.is_empty() {
            let i = match self.state.selected() {
                Some(i) => usize::min(i + steps, self.evtx_table.len() - 1),
                None => 0,
            };
            self.set_selected(i);
        }
    }

    fn previous(&mut self, steps: usize) {
        assert_ne!(steps, 0);
        let i = match self.state.selected() {
            Some(i) => if i < steps { 0 } else {i - steps}
            None => 0,
        };
        self.set_selected(i);
    }
}
