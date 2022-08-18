use std::{fmt, io::{Write, Stdout}};

use crossterm::Result;
use crossterm::{QueueableCommand, cursor::*};
use crossterm::style::*;

pub trait MenuOption {

}

pub enum MainMenuOption {
    NewGame,
    ResumeGame,
    ShowRankings,
    Exit,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl MenuOption for MainMenuOption {

}

impl fmt::Display for MainMenuOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NewGame => write!(f, "new game"),
            Self::ResumeGame => write!(f, "resume game"),
            Self::ShowRankings => write!(f, "show rankings"),
            Self::Exit => write!(f, "exit"),
        }
    }
}

// TODO: make coordinates configurable
pub struct Menu<'a, T> where 
    T: MenuOption + fmt::Display {
    title: &'a str,
    options: &'a [T],
    selected_option: u16,
}

impl<'a, T> Menu<'a, T> where
    T: MenuOption + fmt::Display {
    pub fn new(title: &'a str, options: &'a [T]) -> Self {
        Menu { title, options, selected_option: 0 }
    }

    pub fn display_full(&self, stdout: &mut Stdout) -> Result<()> {
        self.display_title(stdout)?;
        self.display_options(stdout)
    }

    pub fn display_title(&self, stdout: &mut Stdout) -> Result<()> {
        stdout
            .queue(MoveTo(1, 0))?
            .queue(Print(self.title))?
            .flush()
    }

    pub fn display_options(&self, stdout: &mut Stdout) -> Result<()> {
        let mut i = 1;
        for option in self.options {
            stdout
                .queue(MoveTo(1, i))?
                .queue(Print(format!("[ ] {}" , option)))?;
            i += 1;
        }

        let terminal_offset = 1;
        let checkbox_offset = 1;
        let column = terminal_offset + checkbox_offset;
        self.select_option(stdout, column, self.selected_option + 1)?;

        stdout.flush()
    }

    pub fn move_selection(&mut self, stdout: &mut Stdout, direction: Direction) -> Result<()> {
        let terminal_offset = 1;
        let checkbox_offset = 1;
        let column = terminal_offset + checkbox_offset;

        self.deselect_option(stdout, column, self.selected_option + 1)?;

        self.selected_option = match direction {
            Direction::Up => self.move_selection_up(),
            Direction::Down => self.move_selection_down(),
            _ => self.selected_option,
        };

        self.select_option(stdout, column, self.selected_option + 1)?;
        
        stdout.flush()
    }

    fn move_selection_up(&mut self) -> u16 {
        if self.selected_option == 0 {
            (self.options.len() - 1) as u16
        } else {
            self.selected_option - 1
        }
    }

    fn move_selection_down(&mut self) -> u16 {
        if self.selected_option == (self.options.len() - 1) as u16 {
            0
        } else {
            self.selected_option + 1
        }
    }

    // Deselet old option, e.g. "[x]" -> "[ ]"
    fn deselect_option(&'a self, stdout: &'a mut Stdout, column: u16, row: u16) -> Result<&'a mut Stdout> {
        stdout
            .queue(MoveTo(column, row))?
            .queue(Print(" "))
    }

    // Select new option, e.g. "[ ]" -> "[x]"
    fn select_option(&'a self, stdout: &'a mut Stdout, column: u16, row: u16) -> Result<&'a mut Stdout> {
        stdout
            .queue(MoveTo(column, row))?
            .queue(Print("x"))
    }
}
