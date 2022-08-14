use std::fmt;

pub enum MainMenuOption {
    NewGame,
    ResumeGame,
    ShowRankings,
    Exit,
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
