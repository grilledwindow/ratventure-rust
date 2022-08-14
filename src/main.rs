pub mod menu;

use menu::option::MainMenuOption;

fn main() {
    load_main_menu();
}

fn load_main_menu() {
    const MAIN_MENU_OPTIONS: [MainMenuOption; 4] = [
        MainMenuOption::NewGame,
        MainMenuOption::ResumeGame,
        MainMenuOption::ShowRankings,
        MainMenuOption::Exit,
    ];

    println!("Ratventure");
    for option in &MAIN_MENU_OPTIONS {
        println!("[ ] {}", option);
    }
}
