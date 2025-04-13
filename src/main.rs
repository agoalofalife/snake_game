mod introduction;
mod snake;
mod board;

extern crate pancurses;

use pancurses::*;
use std::{thread, time::Duration};
use introduction::*;
use crate::board::*;

fn main() {
    let main_window = create_main_window();
    let config:LevelConfig = show_menu(&main_window);

    let mut board = Board::new(&config);
    board.generate_food();

    'main: loop {
        if board.snake.has_reached_capacity() {
            board.add_text("You are win! 🥇");
            match board.print_exit_dialog() {
                Ok(repeat_game) => {
                    if repeat_game == false {
                        endwin();
                        break 'main;
                    }
                },
                _ => (),
            }
        }

        if board.snake.snake_hit_itself() {
            board.add_text(  "Game Over....🐍");
            match board.print_exit_dialog() {
                Ok(repeat_game) => {
                    if repeat_game == false {
                        endwin();
                        break 'main;
                    }
                },
                _ => (),
            }
        }

        if board.food_is_eaten() {
            board.snake.increase_len();
            board.generate_food();
        }

        // clean tail of snake
        while board.snake.capacity_is_exceed() {
            board.remove_snake_tail();
        }
        board.print_snake_head();

        // change direction after user click arrow on keyboard
        board.catch_push_on_keyboard();


        // y
        // |
        // |
        // |
        // |__________ x
        board.add_next_step_for_snake();

        board.increase_speed_as_need(config.speed_coeff);
        thread::sleep(Duration::from_millis(board.snake_delay()));
    }
}
// helpers


fn create_main_window() -> Window {
    let main_window = initscr();
    start_color();
    use_default_colors();
    curs_set(0); // cursor is invisible
    noecho();
    main_window.refresh();
    main_window
}