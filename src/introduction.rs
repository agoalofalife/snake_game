use std::thread;
use std::time::Duration;
use pancurses::{init_pair, napms, newwin, Input, Window, COLOR_BLACK, COLOR_PAIR, COLOR_WHITE};

const SNAKE_LOGO: [&str; 5] = [
    "  ____  _   _    _    _  __ _____ ",
    " / ___|| \\ | |  / \\  | |/ /| ____|",
    " \\___ \\|  \\| | / _ \\ | ' / |  _|  ",
    "  ___) | |\\  |/ ___ \\| . \\ | |___ ",
    " |____/|_| \\_/_/   \\_\\_|\\_\\|_____|",
];

const SELECT: [&str; 5] = [
    "EASY",
    " ",
    "MEDIUM",
    "",
    "HARD",
];

pub struct LevelConfig {
    pub speed_coeff:i32,
    pub snake_limit_len:i32,
    pub board:Window
}

impl LevelConfig {
    fn easy(main_window: &Window) -> LevelConfig {
        LevelConfig {
            snake_limit_len: 10,
            speed_coeff: 15,
            board: create_board(main_window, 20)
        }
    }
    fn medium(main_window: &Window) -> LevelConfig {
        LevelConfig {
            snake_limit_len: 30,
            speed_coeff: 10,
            board: create_board(main_window, 40)
        }
    }

    fn hard(main_window: &Window) -> LevelConfig {
        LevelConfig {
            snake_limit_len: 50,
            speed_coeff: 5,
            board: create_board(main_window, 60)
        }
    }
}

fn create_board(main_window:&Window, size: i32) -> Window {
    let window = newwin(
        size / 2,
        size,
        (main_window.get_max_y() - size) / 2,
        (main_window.get_max_x() - size) / 2,
    );

    window.bkgd(COLOR_PAIR(1));
    window.refresh();
    window.keypad(true); // enable arrows on keyboard
    window.nodelay(true);
    window
}

pub fn print_logo(window: &Window) {
    for (i, s) in SNAKE_LOGO.iter().enumerate() {
        window.mvaddstr(i as i32 + 1, 8, s);
        window.refresh();
        napms(100);
    }
}

pub fn show_menu(main_window: &Window) -> LevelConfig {
    init_pair(1, COLOR_BLACK, COLOR_WHITE); // set background types
    init_pair(2, COLOR_BLACK, COLOR_BLACK);
    let menu_window = create_board(main_window, 50); // create new window for menu

    print_logo(&menu_window);
    print_select(&menu_window);

    let y_line_coordinates:[i32; 3] = [11, 13, 15]; // y coordinates for draw underline
    let mut line_y = 300;
    // use module operation in order to find position and use cycle movement

    loop {
        match menu_window.getch() {
            Some(Input::KeyDown) => {
                clean_line(&menu_window, y_line_coordinates[line_y % 3], 50);
                line_y += 1;
            },
            Some(Input::KeyUp) => {
                clean_line(&menu_window, y_line_coordinates[line_y % 3], 50);
                line_y -= 1;
            },
            Some(input) => {
                if input == Input::Character("\n".chars().nth(0).unwrap()) {
                    menu_window.bkgd(COLOR_PAIR(2));
                    menu_window.clear();
                    menu_window.refresh();
                    menu_window.delwin();

                    let config = match line_y % 3 {
                      0 => LevelConfig::easy(main_window),
                      1 => LevelConfig::medium(main_window),
                      2 => LevelConfig::hard(main_window),
                      _ => {
                          panic!("Unknown level config found")
                      },
                    };
                    return config;
                }
            },
            None => ()
        }

        draw_underline(&menu_window, y_line_coordinates, line_y);
    }
}
fn print_select(menu_window: &Window) {
    for (i , s) in SELECT.iter().enumerate() {
        menu_window.mvaddstr(i as i32 + 10, 20, s);
        menu_window.refresh();
        napms(100);
    }
}
fn draw_underline(window: &Window, y_coordinates:[i32; 3], curr_y:usize) {
    for i in 0..8 { // length of line from 0 point to 8
        window.mvaddstr(y_coordinates[curr_y % 3], 20 + i, "-");
        thread::sleep(Duration::from_millis(50)); // animation
        window.refresh();
    }
}
fn clean_line(window: &Window, line_num:i32, width:i32) {
    for x in 0..=width{
        window.mvaddstr(line_num, x, " ");
        window.refresh();
    }
}