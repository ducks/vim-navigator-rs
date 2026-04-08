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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    // -- VimNavigator: Normal mode --

    #[test]
    fn normal_mode_j_moves_down() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Char('j'))), NavAction::MoveDown);
    }

    #[test]
    fn normal_mode_k_moves_up() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Char('k'))), NavAction::MoveUp);
    }

    #[test]
    fn normal_mode_arrows() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Down)), NavAction::MoveDown);
        assert_eq!(nav.handle_key(key(KeyCode::Up)), NavAction::MoveUp);
    }

    #[test]
    fn normal_mode_g_moves_top() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Char('g'))), NavAction::MoveTop);
    }

    #[test]
    fn normal_mode_shift_g_moves_bottom() {
        let mut nav = VimNavigator::new();
        assert_eq!(
            nav.handle_key(key(KeyCode::Char('G'))),
            NavAction::MoveBottom
        );
    }

    #[test]
    fn normal_mode_q_quits() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Char('q'))), NavAction::Quit);
    }

    #[test]
    fn normal_mode_unknown_key_is_none() {
        let mut nav = VimNavigator::new();
        assert_eq!(nav.handle_key(key(KeyCode::Char('z'))), NavAction::None);
    }

    // -- VimNavigator: Mode transitions --

    #[test]
    fn colon_enters_command_mode() {
        let mut nav = VimNavigator::new();
        let action = nav.handle_key(key(KeyCode::Char(':')));
        assert_eq!(action, NavAction::ModeChange(InputMode::Command));
        assert_eq!(nav.mode, InputMode::Command);
    }

    #[test]
    fn slash_enters_search_mode() {
        let mut nav = VimNavigator::new();
        let action = nav.handle_key(key(KeyCode::Char('/')));
        assert_eq!(action, NavAction::ModeChange(InputMode::Search));
        assert_eq!(nav.mode, InputMode::Search);
    }

    #[test]
    fn i_enters_insert_mode() {
        let mut nav = VimNavigator::new();
        let action = nav.handle_key(key(KeyCode::Char('i')));
        assert_eq!(action, NavAction::ModeChange(InputMode::Insert));
        assert_eq!(nav.mode, InputMode::Insert);
    }

    #[test]
    fn insert_mode_returns_none() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char('i')));
        assert_eq!(nav.handle_key(key(KeyCode::Char('a'))), NavAction::None);
        assert_eq!(nav.handle_key(key(KeyCode::Char('b'))), NavAction::None);
    }

    #[test]
    fn exit_insert_mode() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char('i')));
        assert_eq!(nav.mode, InputMode::Insert);
        nav.exit_insert_mode();
        assert_eq!(nav.mode, InputMode::Normal);
    }

    // -- VimNavigator: Command mode --

    #[test]
    fn command_mode_builds_buffer_and_submits() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char(':')));
        nav.handle_key(key(KeyCode::Char('w')));
        nav.handle_key(key(KeyCode::Char('q')));
        assert_eq!(nav.command_buffer, "wq");

        let action = nav.handle_key(key(KeyCode::Enter));
        assert_eq!(action, NavAction::Command("wq".into()));
        assert_eq!(nav.mode, InputMode::Normal);
        assert!(nav.command_buffer.is_empty());
    }

    #[test]
    fn command_mode_backspace() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char(':')));
        nav.handle_key(key(KeyCode::Char('a')));
        nav.handle_key(key(KeyCode::Char('b')));
        nav.handle_key(key(KeyCode::Backspace));
        assert_eq!(nav.command_buffer, "a");
    }

    #[test]
    fn command_mode_esc_cancels() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char(':')));
        nav.handle_key(key(KeyCode::Char('q')));
        let action = nav.handle_key(key(KeyCode::Esc));
        assert_eq!(action, NavAction::ModeChange(InputMode::Normal));
        assert_eq!(nav.mode, InputMode::Normal);
        assert!(nav.command_buffer.is_empty());
    }

    // -- VimNavigator: Search mode --

    #[test]
    fn search_mode_submits_and_stores_last_search() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char('/')));
        nav.handle_key(key(KeyCode::Char('f')));
        nav.handle_key(key(KeyCode::Char('o')));
        nav.handle_key(key(KeyCode::Char('o')));

        let action = nav.handle_key(key(KeyCode::Enter));
        assert_eq!(action, NavAction::Search("foo".into()));
        assert_eq!(nav.get_last_search(), "foo");
        assert_eq!(nav.mode, InputMode::Normal);
    }

    #[test]
    fn search_next_and_prev_in_normal_mode() {
        let mut nav = VimNavigator::new();
        assert_eq!(
            nav.handle_key(key(KeyCode::Char('n'))),
            NavAction::SearchNext
        );
        assert_eq!(
            nav.handle_key(key(KeyCode::Char('N'))),
            NavAction::SearchPrev
        );
    }

    #[test]
    fn search_mode_esc_cancels() {
        let mut nav = VimNavigator::new();
        nav.handle_key(key(KeyCode::Char('/')));
        nav.handle_key(key(KeyCode::Char('x')));
        nav.handle_key(key(KeyCode::Esc));
        assert_eq!(nav.mode, InputMode::Normal);
        assert!(nav.search_buffer.is_empty());
        assert!(nav.last_search.is_empty()); // not committed
    }

    // -- ListNavigator --

    #[test]
    fn list_move_down_increments() {
        let mut list = ListNavigator::new();
        list.move_down(5);
        assert_eq!(list.selected(), 1);
    }

    #[test]
    fn list_move_down_clamps_at_end() {
        let mut list = ListNavigator::new();
        list.move_down(3);
        list.move_down(3);
        list.move_down(3);
        list.move_down(3);
        assert_eq!(list.selected(), 2);
    }

    #[test]
    fn list_move_up_decrements() {
        let mut list = ListNavigator::new();
        list.move_down(5);
        list.move_down(5);
        list.move_up();
        assert_eq!(list.selected(), 1);
    }

    #[test]
    fn list_move_up_clamps_at_zero() {
        let mut list = ListNavigator::new();
        list.move_up();
        assert_eq!(list.selected(), 0);
    }

    #[test]
    fn list_move_top_and_bottom() {
        let mut list = ListNavigator::new();
        list.move_bottom(10);
        assert_eq!(list.selected(), 9);
        list.move_top();
        assert_eq!(list.selected(), 0);
    }

    #[test]
    fn list_empty_list_safe() {
        let mut list = ListNavigator::new();
        list.move_down(0);
        assert_eq!(list.selected(), 0);
        list.move_bottom(0);
        assert_eq!(list.selected(), 0);
    }

    #[test]
    fn list_reset() {
        let mut list = ListNavigator::new();
        list.move_down(10);
        list.move_down(10);
        list.reset();
        assert_eq!(list.selected(), 0);
    }

    #[test]
    fn default_impls() {
        let nav = VimNavigator::default();
        assert_eq!(nav.mode, InputMode::Normal);

        let list = ListNavigator::default();
        assert_eq!(list.selected(), 0);
    }
}
