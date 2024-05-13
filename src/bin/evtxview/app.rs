use std::{io, ops::Neg, time::Duration};

use crate::{
    cli::Cli,
    tui::{self, ColorScheme, EvtxTable, PALETTES},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use tui_textarea::TextArea;

// (→) next color | (←) previous color
const INFO_TEXT: &str = r#"(Esc) quit | (↑) move up | (↓) move down | (E) Exclude by Event id" | (e) include by Event id | (U) exclude by User | (u) include by User | (R) Reset filter | (o) change Orientation | (+/-) in/decrease table size | (/|?) search forard/backward"#;

#[derive(Clone, Copy)]
enum SearchOrder {
    Forward,
    Backward,
}

impl Neg for SearchOrder {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            SearchOrder::Forward => SearchOrder::Backward,
            SearchOrder::Backward => SearchOrder::Forward,
        }
    }
}

#[derive(Default, Clone, Copy)]
enum AppMode {
    #[default]
    Normal,
    SearchField(SearchOrder),
}

pub struct App<'t> {
    evtx_table: EvtxTable,
    exit: bool,
    state: TableState,
    table_scroll_state: ScrollbarState,
    details_scroll_state: ScrollbarState,
    colors: ColorScheme,
    table_view_port: Rect,
    orientation: Direction,
    table_percentage: u16,
    app_mode: AppMode,
    search_field: TextArea<'t>,
    search_order: SearchOrder,
}

impl<'t> App<'t> {
    pub fn new(cli: Cli) -> Self {
        let paths: Vec<_> = cli.evtx_file.iter().map(|p| p.path().path()).collect();
        let evtx_table = EvtxTable::try_from(paths).unwrap();
        let table_len = evtx_table.len();
        let table_scroll_state = if table_len == 0 { 0 } else { table_len - 1 };

        let search_field = TextArea::default();

        Self {
            evtx_table,
            exit: Default::default(),
            state: TableState::default().with_selected(0),
            table_scroll_state: ScrollbarState::new(table_scroll_state),
            details_scroll_state: ScrollbarState::new(0),
            colors: ColorScheme::new(&PALETTES[0]),
            table_view_port: Rect::new(0, 0, 0, 0),
            orientation: Direction::Horizontal,
            table_percentage: 50,
            app_mode: Default::default(),
            search_order: SearchOrder::Forward,
            search_field,
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
        let contents_line;
        let textfield_line;
        let help_line;

        match self.app_mode {
            AppMode::Normal => {
                let lines = Layout::vertical([Constraint::Min(5), Constraint::Length(3)])
                    .split(frame.size());
                contents_line = lines[0];
                textfield_line = None;
                help_line = lines[1];
            }
            AppMode::SearchField(_) => {
                let lines = Layout::vertical([
                    Constraint::Min(5),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame.size());
                contents_line = lines[0];
                textfield_line = Some(lines[1]);
                help_line = lines[2];
            }
        }

        let cols = Layout::new(
            self.orientation,
            vec![
                Constraint::Percentage(self.table_percentage),
                Constraint::Percentage(100 - self.table_percentage),
            ],
        )
        .split(contents_line);

        let table_scroll_area = cols[0].inner(&margins);
        let table_contents_area = table_scroll_area.inner(&margins);
        self.table_view_port = table_contents_area;

        frame.render_widget(Clear, contents_line);
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
        if let Some(textfield_line) = textfield_line {
            self.render_textfield(frame, textfield_line);
        }
        self.render_footer(frame, help_line);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        self.evtx_table.render(frame, area, &mut self.state)
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

    fn render_textfield(&mut self, frame: &mut Frame, area: Rect) {
        let block = self.bordered_block();
        let layout = Layout::horizontal(vec![Constraint::Length(19), Constraint::Min(0)])
            .split(block.inner(area));
        frame.render_widget(block, area);
        let display_text = match self.app_mode {
            AppMode::Normal => "",
            AppMode::SearchField(SearchOrder::Forward) => "Search (forward):",
            AppMode::SearchField(SearchOrder::Backward) => "Search (backward):"
        };
        frame.render_widget(Text::raw(display_text), layout[0]);
        frame.render_widget(self.search_field.widget(), layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        self.evtx_table.update();
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.evtx_table.update();

        match self.app_mode {
            AppMode::Normal => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                KeyCode::Char('g') => self.set_selected(0),
                KeyCode::Char('G') => self.set_selected(usize::max(self.evtx_table.len(), 1) - 1),
                KeyCode::Down => self.next(1),
                KeyCode::Up => self.previous(1),
                KeyCode::PageDown => self.next((self.table_view_port.height / 2).into()),
                KeyCode::PageUp => self.previous((self.table_view_port.height / 2).into()),
                KeyCode::Char('E') => self.exclude_event_id(),
                KeyCode::Char('e') => self.include_event_id(),
                KeyCode::Char('U') => self.exclude_user(),
                KeyCode::Char('u') => self.include_user(),
                KeyCode::Char('R') => self.reset_filter(),
                KeyCode::Char('o') => self.change_orientation(),
                KeyCode::Char('+') => self.increase_table_size(),
                KeyCode::Char('-') => self.decrease_table_size(),
                KeyCode::Char('/') => self.app_mode = AppMode::SearchField(SearchOrder::Forward),
                KeyCode::Char('?') => self.app_mode = AppMode::SearchField(SearchOrder::Backward),
                KeyCode::Char('n') => self.goto_next_finding(self.search_order),
                KeyCode::Char('N') => self.goto_next_finding(self.search_order.neg()),
                _ => {}
            },
            AppMode::SearchField(order) => match key_event.code {
                KeyCode::Enter => {
                    self.app_mode = AppMode::Normal;
                    self.search_order = order;
                    self.goto_next_finding(order);
                }

                KeyCode::Esc => {
                    self.app_mode = AppMode::Normal;
                }

                _ => {
                    self.search_field.input(key_event);
                }
            },
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn goto_next_finding(&mut self, order: SearchOrder) {
        if !self.evtx_table.is_empty() {
            if let Some(i) = self.state.selected() {
                if let Some(search_string) = self.search_field.lines().first() {
                    if let Some(index) = match order {
                        SearchOrder::Forward => self.evtx_table.find_next(i, search_string),
                        SearchOrder::Backward => self.evtx_table.find_previous(i, search_string),
                    } {
                        self.set_selected(index);
                    }
                }
            }
        }
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

    fn exclude_user(&mut self) {
        if !self.evtx_table.is_empty() {
            if let Some(i) = self.state.selected() {
                self.evtx_table.exclude_user(i)
            }
        }
    }

    fn include_user(&mut self) {
        if !self.evtx_table.is_empty() {
            if let Some(i) = self.state.selected() {
                self.evtx_table.include_user(i)
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
