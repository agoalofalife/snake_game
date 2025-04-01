use pancurses::Window;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::snake::{Coordinate, Snake};

pub fn print_head_snake_on_board(window: &Window, snake: &Snake) {
    window.mvprintw(snake.head().y , snake.head().x, snake.sign());
    window.refresh();
}

pub fn clean_symbol(window:&Window, coordinate: Coordinate) {
    window.mvprintw(coordinate.y, coordinate.x, " ");
}
pub fn print_random_food_on_board(window: &Window, rng: &mut ThreadRng) -> Coordinate {
    let coordinate = Coordinate{x: rng.random_range(0..window.get_max_x()), y: rng.random_range(0..window.get_max_y() / 2)};
    window.mvprintw(coordinate.y, coordinate.x, "*");
    window.refresh();
    coordinate
}

pub fn print_to_center(window: &Window, text_slices: &Vec<&str>) {
    let mut line:i32 = window.get_max_y() / 2;

    for text in text_slices {
        window.mvprintw(line,  (window.get_max_x() / 2) - (text.len() as i32 / 2), text);
        line += 2;
    }
}