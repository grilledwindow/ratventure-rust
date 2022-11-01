use std::io;

use crossterm::{Result, execute};
use crossterm::event::{read, Event, KeyCode::*};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::*;

pub mod menu;

use menu::main_menu::{ Direction, Menu, MainMenuOption};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    const MAIN_MENU_OPTIONS: [MainMenuOption; 4] = [
        MainMenuOption::NewGame,
        MainMenuOption::ResumeGame,
        MainMenuOption::ShowRankings,
        MainMenuOption::Exit,
    ];

    let mut main_menu = Menu::new("Ratventure", &MAIN_MENU_OPTIONS);

    execute!(stdout, EnterAlternateScreen, Hide)?;
    main_menu.display_full(&mut stdout)?;

    loop {
        let event = read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                Enter => if main_menu.is_exit_selected() { break },
                Up | Char('k') | Char('w') => main_menu.move_selection(&mut stdout, Direction::Up)?,
                Down | Char('j') | Char('s') => main_menu.move_selection(&mut stdout, Direction::Down)?, 
                _ => continue,
            };
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)
}
