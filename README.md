# vim-navigator

Vim-style modal editing and navigation patterns for Ratatui TUIs.

## Features

- Modal editing with Normal, Insert, Command, and Search modes
- Vim keybindings (j/k, g/G, :commands, /search, n/N)
- List navigation with automatic bounds checking
- Zero runtime dependencies beyond crossterm

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
vim-navigator = "20260220"
crossterm = "0.28"
```

Basic example:

```rust
use vim_navigator::{VimNavigator, ListNavigator, NavAction, InputMode};
use crossterm::event::{self, Event, KeyEvent};

struct App {
    vim_nav: VimNavigator,
    items: Vec<String>,
    items_nav: ListNavigator,
}

impl App {
    fn handle_input(&mut self, key: KeyEvent) -> bool {
        match self.vim_nav.handle_key(key) {
            NavAction::Quit => return true,
            NavAction::MoveDown => self.items_nav.move_down(self.items.len()),
            NavAction::MoveUp => self.items_nav.move_up(),
            NavAction::MoveTop => self.items_nav.move_top(),
            NavAction::MoveBottom => self.items_nav.move_bottom(self.items.len()),
            NavAction::Command(cmd) => self.execute_command(&cmd),
            NavAction::Search(query) => self.search(&query),
            NavAction::SearchNext => self.search_next(),
            NavAction::SearchPrev => self.search_prev(),
            NavAction::ModeChange(_) => {},
            _ => {}
        }
        false
    }

    fn execute_command(&mut self, cmd: &str) {
        match cmd {
            "q" | "quit" => std::process::exit(0),
            _ => {}
        }
    }
}
```

## Keybindings

### Normal Mode
- `j` / Down - Move down
- `k` / Up - Move up
- `g` - Jump to top
- `G` - Jump to bottom
- `:` - Enter command mode
- `/` - Enter search mode
- `i` - Enter insert mode
- `n` - Next search match
- `N` - Previous search match
- `q` - Quit (returns NavAction::Quit)

### Command Mode
- Type commands after `:`
- `Enter` - Execute command
- `Esc` - Cancel and return to normal mode
- `Backspace` - Delete character

### Search Mode
- `/` - Enter search mode (from normal mode)
- Type search query
- `Enter` - Execute search (returns `NavAction::Search(query)`)
- `Esc` - Cancel and return to normal mode
- `Backspace` - Delete character
- `n` - Next match (from normal mode, returns `NavAction::SearchNext`)
- `N` - Previous match (from normal mode, returns `NavAction::SearchPrev`)

### Insert Mode
- `Esc` - Return to normal mode
- All other keys return `NavAction::None` for app-specific handling

## License

MIT OR Apache-2.0
