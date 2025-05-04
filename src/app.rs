use std::path::PathBuf;
use std::fs;
use std::io;
use std::io::Seek;
use walkdir::WalkDir;
use notify::{Watcher, RecursiveMode, Event, Result as NotifyResult};
use crate::log_parser::{LogParser, LogEntry};
use crate::ui::{UI, UIEvent};

pub struct LogViewer {
    directory: PathBuf,
    files: Vec<PathBuf>,
    current_file: Option<PathBuf>,
    log_entries: Vec<LogEntry>,
    ui: UI,
    parser: LogParser,
    is_tailing: bool,
    is_file_list_focused: bool,
    last_file_size: u64,
}

impl LogViewer {
    pub fn new(directory: PathBuf) -> Result<Self, io::Error> {
        let ui = UI::new()?;
        let parser = LogParser::new();
        
        let mut viewer = Self {
            directory,
            files: Vec::new(),
            current_file: None,
            log_entries: Vec::new(),
            ui,
            parser,
            is_tailing: true,
            is_file_list_focused: true,
            last_file_size: 0,
        };

        viewer.load_files()?;
        Ok(viewer)
    }

    fn load_files(&mut self) -> io::Result<()> {
        self.files = WalkDir::new(&self.directory)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect();
        Ok(())
    }

    fn load_log_file(&mut self, file: &PathBuf) -> io::Result<()> {
        let content = fs::read_to_string(file)?;
        self.log_entries = self.parser.parse(&content);
        self.current_file = Some(file.clone());
        self.last_file_size = file.metadata()?.len();
        
        // Reset UI state for the new file
        self.ui.log_list_state.select(None);
        self.ui.clear_expanded_entries();
        self.ui.reset_scroll();
        self.is_tailing = true;
        
        // If we have entries, select the last one by default
        if !self.log_entries.is_empty() {
            self.ui.log_list_state.select(Some(self.log_entries.len() - 1));
        }
        
        Ok(())
    }

    fn handle_file_list_navigation(&mut self, up: bool) -> io::Result<()> {
        let selected = self.ui.file_list_state.selected();
        let new_selected = match selected {
            Some(selected) => {
                if up {
                    if selected > 0 { selected - 1 } else { selected }
                } else {
                    if selected < self.files.len() - 1 { selected + 1 } else { selected }
                }
            }
            None => if !self.files.is_empty() { 0 } else { 0 },
        };

        if new_selected != selected.unwrap_or(0) {
            self.ui.file_list_state.select(Some(new_selected));
            let file = self.files[new_selected].clone();
            self.load_log_file(&file)?;
        }
        Ok(())
    }

    fn handle_log_list_navigation(&mut self, up: bool) -> io::Result<()> {
        let selected = self.ui.log_list_state.selected();
        
        // Calculate total number of visible lines including expanded entries
        let total_visible_lines = self.log_entries.iter().enumerate()
            .map(|(i, entry)| {
                if self.ui.expanded_entries.contains(&i) {
                    entry.lines.len()
                } else {
                    1
                }
            })
            .sum::<usize>();

        let new_selected = match selected {
            Some(selected) => {
                if up {
                    if selected > 0 { selected - 1 } else { selected }
                } else {
                    if selected < total_visible_lines - 1 { selected + 1 } else { selected }
                }
            }
            None => if !self.log_entries.is_empty() { 0 } else { 0 },
        };

        if new_selected != selected.unwrap_or(0) {
            self.ui.log_list_state.select(Some(new_selected));
            // Only disable tail mode if we're not at the last entry
            if new_selected < total_visible_lines - 1 {
                self.is_tailing = false;
            } else {
                self.is_tailing = true;
            }
        }
        Ok(())
    }

    fn handle_navigation(&mut self, event: UIEvent) -> io::Result<()> {
        match event {
            UIEvent::Up => {
                if self.is_file_list_focused {
                    self.handle_file_list_navigation(true)?;
                } else {
                    self.handle_log_list_navigation(true)?;
                }
            }
            UIEvent::Down => {
                if self.is_file_list_focused {
                    self.handle_file_list_navigation(false)?;
                } else {
                    self.handle_log_list_navigation(false)?;
                }
            }
            UIEvent::Left => {
                if self.is_file_list_focused {
                    // Do nothing in file list
                } else if self.ui.is_at_beginning() {
                    // If at the beginning of the lines, switch to file list
                    self.is_file_list_focused = true;
                } else {
                    // Otherwise scroll left
                    self.ui.scroll_log_left();
                }
            }
            UIEvent::Right => {
                if self.is_file_list_focused {
                    self.is_file_list_focused = false;
                } else {
                    self.ui.scroll_log_right();
                }
            }
            UIEvent::SwitchToFileList => {
                self.is_file_list_focused = true;
            }
            UIEvent::SwitchToLogView => {
                self.is_file_list_focused = false;
            }
            UIEvent::ToggleExpand => {
                if !self.is_file_list_focused {
                    if let Some(selected_line) = self.ui.log_list_state.selected() {
                        // Map the selected line index back to the log entry index
                        let mut entry_index = 0;
                        let mut line_count = 0;
                        for (i, entry) in self.log_entries.iter().enumerate() {
                            let is_expanded = self.ui.expanded_entries.contains(&i);
                            let lines = if is_expanded { entry.lines.len() } else { 1 };
                            if selected_line < line_count + lines {
                                entry_index = i;
                                break;
                            }
                            line_count += lines;
                        }
                        self.ui.toggle_expand(entry_index);
                    }
                }
            }
            UIEvent::ToggleTail => {
                if !self.is_file_list_focused {
                    self.is_tailing = !self.is_tailing;
                    if self.is_tailing && !self.log_entries.is_empty() {
                        self.ui.log_list_state.select(Some(self.log_entries.len() - 1));
                    }
                }
            }
            UIEvent::ScrollLeft => {
                if !self.is_file_list_focused {
                    self.ui.scroll_log_left();
                }
            }
            UIEvent::ScrollRight => {
                if !self.is_file_list_focused {
                    self.ui.scroll_log_right();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_file_update(&mut self) -> io::Result<()> {
        if let Some(file) = &self.current_file {
            let current_size = file.metadata()?.len();
            if current_size > self.last_file_size {
                // Read only the new content
                let mut file = fs::File::open(file)?;
                file.seek(io::SeekFrom::Start(self.last_file_size))?;
                let mut new_content = String::new();
                io::Read::read_to_string(&mut file, &mut new_content)?;
                
                // Parse new entries
                let new_entries = self.parser.parse(&new_content);
                self.log_entries.extend(new_entries);
                
                // Update last file size
                self.last_file_size = current_size;
                
                // If we're in tail mode, select the last entry
                if self.is_tailing && !self.log_entries.is_empty() {
                    self.ui.log_list_state.select(Some(self.log_entries.len() - 1));
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Set up file watcher
        let mut watcher: notify::RecommendedWatcher = notify::recommended_watcher(|res: NotifyResult<Event>| {
            if let Ok(event) = res {
                if let notify::EventKind::Modify(_) = event.kind {
                    // The file has been modified, we'll handle it in the main loop
                }
            }
        })?;

        if let Some(file) = &self.current_file {
            watcher.watch(file, RecursiveMode::NonRecursive)?;
        }

        // Select first file by default if available
        if !self.files.is_empty() {
            self.ui.file_list_state.select(Some(0));
            let file = self.files[0].clone();
            self.load_log_file(&file)?;
        }

        loop {
            // Check for file updates
            self.handle_file_update()?;
            
            self.ui.draw(&self.files, &self.log_entries, &self.current_file, self.is_file_list_focused)?;

            if let Some(event) = self.ui.handle_events()? {
                match event {
                    UIEvent::Quit => break,
                    event => self.handle_navigation(event)?,
                }
            }
        }

        self.ui.cleanup()?;
        Ok(())
    }
} 