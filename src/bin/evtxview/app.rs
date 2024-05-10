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

// (→) next color | (←) previous color 
const INFO_TEXT: &str = r#"(Esc) quit | (↑) move up | (↓) move down | (x) eXclude by event id" | (i) Include by event id | (R) Reset filter | (o) change Orientation | (+/-) in/decrease table size"#;

pub struct App {
    evtx_table: EvtxTable,
    exit: bool,
    state: TableState,
    table_scroll_state: ScrollbarState,
    details_scroll_state: ScrollbarState,
    colors: ColorScheme,
    table_view_port: Rect,
    orientation: Direction,
    table_percentage: u16,
}

impl App {
    pub fn new(cli: Cli) -> Self {
        let evtx_table = EvtxTable::try_from(cli.evtx_file.path().path()).unwrap();
        let table_len = evtx_table.len();
        Self {
            evtx_table,
            exit: Default::default(),
            state: TableState::default().with_selected(0),
            table_scroll_state: ScrollbarState::new(table_len - 1),
            details_scroll_state: ScrollbarState::new(0),
            colors: ColorScheme::new(&PALETTES[0]),
            table_view_port: Rect::new(0, 0, 0, 0),
            orientation: Direction::Horizontal,
            table_percentage: 50
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
        let margins = Margin::new(0, 0);
        let rects = Layout::vertical([
            Constraint::Min(5),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(frame.size());

        let cols = Layout::new(
            self.orientation,
            vec![Constraint::Percentage(self.table_percentage), Constraint::Percentage(100-self.table_percentage)],
        )
        .split(rects[0]);

        let table_scroll_area = cols[0].inner(&margins);
        let table_contents_area = table_scroll_area.inner(&margins);
        self.table_view_port = table_contents_area;

        frame.render_widget(Clear, rects[0]);
        self.render_table(frame, self.table_view_port);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(None),
            table_scroll_area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.table_scroll_state,
        );

        let details_scroll_area = cols[1].inner(&margins);
        let details_contents_area = details_scroll_area.inner(&margins);
        self.render_content(frame, details_contents_area);
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            details_scroll_area,
            &mut self.details_scroll_state,
        );
        self.render_sparkline(frame, rects[1]);
        self.render_footer(frame, rects[2]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(self.colors.footer_border_color()));
        frame.render_stateful_widget(self.evtx_table.table().block(block), area, &mut self.state);
    }
    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        match self.state.selected() {
            Some(i) => match self.evtx_table.content(i) {
                Some(value) => frame.render_widget(
                    Paragraph::new(&value[..])
                        .wrap(Wrap { trim: false })
                        .block(self.bordered_block()),
                    area,
                ),
                None => frame.render_widget(Clear, area),
            },
            None => frame.render_widget(Clear, area),
        }
    }

    fn render_sparkline(&mut self, frame: &mut Frame, area: Rect) {
        let spark_line = Sparkline::default()
            .data(self.evtx_table.sparkline_data())
            .block(self.bordered_block());
        frame.render_widget(spark_line, area)
    }

    fn bordered_block(&self) -> Block {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(self.colors.footer_border_color()))
    }

    fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Line::from(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg())
                    .bg(self.colors.buffer_bg()),
            )
            .centered()
            .block(self.bordered_block());
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
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            KeyCode::Char('g') => self.set_selected(0),
            KeyCode::Char('G') => self.set_selected(self.evtx_table.len() - 1),
            KeyCode::Down => self.next(1),
            KeyCode::Up => self.previous(1),
            KeyCode::PageDown => self.next((self.table_view_port.height / 2).into()),
            KeyCode::PageUp => self.previous((self.table_view_port.height / 2).into()),
            KeyCode::Char('x') => self.exclude_event_id(),
            KeyCode::Char('i') => self.include_event_id(),
            KeyCode::Char('R') => self.reset_filter(),
            KeyCode::Char('o') => self.change_orientation(),
            KeyCode::Char('+') => self.increase_table_size(),
            KeyCode::Char('-') => self.decrease_table_size(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increase_table_size(&mut self) {
        // leave some space
        if self.table_percentage < 97 {
            self.table_percentage += 1;
        }
    }

    fn decrease_table_size(&mut self) {
        // leave some space
        if self.table_percentage > 3 {
            self.table_percentage -= 1;
        }
    }

    fn change_orientation(&mut self) {
        self.orientation = match self.orientation {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }

    fn exclude_event_id(&mut self) {
        if !self.evtx_table.is_empty() {
            if let Some(i) = self.state.selected() {
                self.evtx_table.exclude_event_id(i)
            }
        }
    }

    fn include_event_id(&mut self) {
        if !self.evtx_table.is_empty() {
            if let Some(i) = self.state.selected() {
                self.evtx_table.include_event_id(i)
            }
        }
    }

    fn reset_filter(&mut self) {
        self.evtx_table.reset_filter();
    }

    fn set_selected(&mut self, idx: usize) {
        self.state.select(Some(idx));
        self.table_scroll_state = self.table_scroll_state.position(idx);
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
            Some(i) => {
                if i < steps {
                    0
                } else {
                    i - steps
                }
            }
            None => 0,
        };
        self.set_selected(i);
    }
}
