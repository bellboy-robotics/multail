use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    text::{Span, Spans},
    Terminal,
};
use std::path::PathBuf;
use crate::log_parser::{LogEntry, LogLevel};

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub file_list_state: ListState,
    pub log_list_state: ListState,
    expanded_entries: std::collections::HashSet<usize>,
    log_scroll_offset: u16,
}

impl UI {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            file_list_state: ListState::default(),
            log_list_state: ListState::default(),
            expanded_entries: std::collections::HashSet::new(),
            log_scroll_offset: 0,
        })
    }

    pub fn draw(
        &mut self,
        files: &[PathBuf],
        log_entries: &[LogEntry],
        _current_file: &Option<PathBuf>,
        is_file_list_focused: bool,
    ) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            // File list
            let file_items: Vec<ListItem> = files
                .iter()
                .map(|f| ListItem::new(f.file_name().unwrap().to_string_lossy().to_string()))
                .collect();
            let file_list = List::new(file_items)
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .highlight_style(if is_file_list_focused {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                });
            f.render_stateful_widget(file_list, chunks[0], &mut self.file_list_state);

            // Log entries
            let log_items: Vec<ListItem> = log_entries
                .iter()
                .enumerate()
                .flat_map(|(i, entry)| {
                    let is_expanded = self.expanded_entries.contains(&i);
                    let style = match entry.level {
                        LogLevel::Debug => Style::default().fg(Color::DarkGray),
                        LogLevel::Info => Style::default().fg(Color::White),
                        LogLevel::Warn => Style::default().fg(Color::Yellow),
                        LogLevel::Error => Style::default().fg(Color::Red),
                    };
                    if is_expanded {
                        entry.lines.iter().map(|line| {
                            let line = if self.log_scroll_offset > 0 {
                                line.chars().skip(self.log_scroll_offset as usize).collect::<String>()
                            } else {
                                line.clone()
                            };
                            ListItem::new(Spans::from(vec![Span::styled(line, style)]))
                        }).collect::<Vec<_>>()
                    } else {
                        let mut line = entry.lines[0].clone();
                        if self.log_scroll_offset > 0 {
                            line = line.chars().skip(self.log_scroll_offset as usize).collect::<String>();
                        }
                        let mut spans = vec![Span::styled(line, style)];
                        if entry.lines.len() > 1 {
                            spans.push(Span::styled(" ⋯", Style::default().fg(Color::Cyan)));
                        }
                        vec![ListItem::new(Spans::from(spans))]
                    }
                })
                .collect();

            // Add scroll indicator to the title
            let scroll_indicator = if self.log_scroll_offset > 0 {
                format!(" Logs (← {})", self.log_scroll_offset)
            } else {
                " Logs".to_string()
            };

            let log_list = List::new(log_items)
                .block(Block::default().borders(Borders::ALL).title(scroll_indicator))
                .highlight_style(if !is_file_list_focused {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                });
            f.render_stateful_widget(log_list, chunks[1], &mut self.log_list_state);
        })?;
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<Option<UIEvent>, io::Error> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Some(UIEvent::Quit)),
                    KeyCode::Up => return Ok(Some(UIEvent::Up)),
                    KeyCode::Down => return Ok(Some(UIEvent::Down)),
                    KeyCode::Left => return Ok(Some(UIEvent::Left)),
                    KeyCode::Right => return Ok(Some(UIEvent::Right)),
                    KeyCode::Char(' ') => return Ok(Some(UIEvent::ToggleExpand)),
                    KeyCode::Char('t') => return Ok(Some(UIEvent::ToggleTail)),
                    KeyCode::Char('h') => return Ok(Some(UIEvent::ScrollLeft)),
                    KeyCode::Char('l') => return Ok(Some(UIEvent::ScrollRight)),
                    _ => {}
                }
            }
        }
        Ok(None)
    }

    pub fn cleanup(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        self.terminal.backend_mut().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn toggle_expand(&mut self, index: usize) {
        if self.expanded_entries.contains(&index) {
            self.expanded_entries.remove(&index);
        } else {
            self.expanded_entries.insert(index);
        }
    }

    pub fn clear_expanded_entries(&mut self) {
        self.expanded_entries.clear();
    }

    pub fn scroll_log_left(&mut self) {
        if self.log_scroll_offset > 0 {
            self.log_scroll_offset = self.log_scroll_offset.saturating_sub(10);
        }
    }

    pub fn scroll_log_right(&mut self) {
        self.log_scroll_offset = self.log_scroll_offset.saturating_add(10);
    }

    pub fn reset_scroll(&mut self) {
        self.log_scroll_offset = 0;
    }
}

pub enum UIEvent {
    Quit,
    Up,
    Down,
    Left,
    Right,
    ToggleExpand,
    ToggleTail,
    ScrollLeft,
    ScrollRight,
} 