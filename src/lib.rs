use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Command,
    Insert,
    Search,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavAction {
    None,
    ModeChange(InputMode),
    Command(String),
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    Search(String),
    SearchNext,
    SearchPrev,
    Quit,
}

pub struct VimNavigator {
    pub mode: InputMode,
    pub command_buffer: String,
    pub search_buffer: String,
    pub last_search: String,
}

impl VimNavigator {
    pub fn new() -> Self {
        Self {
            mode: InputMode::Normal,
            command_buffer: String::new(),
            search_buffer: String::new(),
            last_search: String::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> NavAction {
        match self.mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Command => self.handle_command_mode(key),
            InputMode::Insert => NavAction::None,
            InputMode::Search => self.handle_search_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) -> NavAction {
        match key.code {
            KeyCode::Char('q') => NavAction::Quit,
            KeyCode::Char(':') => {
                self.mode = InputMode::Command;
                self.command_buffer.clear();
                NavAction::ModeChange(InputMode::Command)
            }
            KeyCode::Char('/') => {
                self.mode = InputMode::Search;
                self.search_buffer.clear();
                NavAction::ModeChange(InputMode::Search)
            }
            KeyCode::Char('n') => NavAction::SearchNext,
            KeyCode::Char('N') => NavAction::SearchPrev,
            KeyCode::Char('i') => {
                self.mode = InputMode::Insert;
                NavAction::ModeChange(InputMode::Insert)
            }
            KeyCode::Char('j') | KeyCode::Down => NavAction::MoveDown,
            KeyCode::Char('k') | KeyCode::Up => NavAction::MoveUp,
            KeyCode::Char('g') => NavAction::MoveTop,
            KeyCode::Char('G') => NavAction::MoveBottom,
            KeyCode::Esc => {
                self.mode = InputMode::Normal;
                NavAction::ModeChange(InputMode::Normal)
            }
            _ => NavAction::None,
        }
    }

    fn handle_command_mode(&mut self, key: KeyEvent) -> NavAction {
        match key.code {
            KeyCode::Esc => {
                self.mode = InputMode::Normal;
                self.command_buffer.clear();
                NavAction::ModeChange(InputMode::Normal)
            }
            KeyCode::Enter => {
                let cmd = self.command_buffer.clone();
                self.mode = InputMode::Normal;
                self.command_buffer.clear();
                NavAction::Command(cmd)
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
                NavAction::None
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
                NavAction::None
            }
            _ => NavAction::None,
        }
    }

    fn handle_search_mode(&mut self, key: KeyEvent) -> NavAction {
        match key.code {
            KeyCode::Esc => {
                self.mode = InputMode::Normal;
                self.search_buffer.clear();
                NavAction::ModeChange(InputMode::Normal)
            }
            KeyCode::Enter => {
                let search = self.search_buffer.clone();
                self.last_search = search.clone();
                self.mode = InputMode::Normal;
                self.search_buffer.clear();
                NavAction::Search(search)
            }
            KeyCode::Backspace => {
                self.search_buffer.pop();
                NavAction::None
            }
            KeyCode::Char(c) => {
                self.search_buffer.push(c);
                NavAction::None
            }
            _ => NavAction::None,
        }
    }

    pub fn exit_insert_mode(&mut self) {
        self.mode = InputMode::Normal;
    }

    pub fn get_last_search(&self) -> &str {
        &self.last_search
    }
}

impl Default for VimNavigator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ListNavigator {
    pub selected_index: usize,
}

impl ListNavigator {
    pub fn new() -> Self {
        Self { selected_index: 0 }
    }

    pub fn move_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn move_down(&mut self, list_len: usize) {
        if list_len > 0 {
            self.selected_index = (self.selected_index + 1).min(list_len - 1);
        }
    }

    pub fn move_top(&mut self) {
        self.selected_index = 0;
    }

    pub fn move_bottom(&mut self, list_len: usize) {
        if list_len > 0 {
            self.selected_index = list_len - 1;
        }
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
    }

    pub fn selected(&self) -> usize {
        self.selected_index
    }
}

impl Default for ListNavigator {
    fn default() -> Self {
        Self::new()
    }
}
