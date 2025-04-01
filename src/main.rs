mod introduction;
mod snake;
mod board;

extern crate pancurses;

use pancurses::*;
use std::{thread, time::Duration};
use rand::Rng;
use introduction::*;
use crate::board::*;
use crate::snake::Snake;
use crate::snake::Coordinate;

#[derive(PartialEq)]
struct Direction {
    x:i32,
    y:i32,
}

const DOWN:Direction = Direction{y:1 , x:0};
const UP:Direction = Direction{y:-1 , x:0};
const LEFT:Direction = Direction{y:0 , x:-1};
const RIGHT:Direction = Direction{y:0 , x:1};


fn main() {
    let mut rng = rand::rng();
    let main_window = initscr();
    start_color();
    use_default_colors();
    curs_set(0); // cursor is invisible
    noecho();

    main_window.refresh();

    let config = show_menu(&main_window);
    let window = config.board;
    let mut direction = RIGHT; // start direction

    let mut food_coordinate: Coordinate = print_random_food_on_board(&window, &mut rng);
    let mut snake = Snake::new(1, config.snake_limit_len, "*".to_string()); // deref coercing
    let mut speed;

    let mut final_text: Vec<&str> = vec![];

    'main: loop {
        if snake.has_reached_capacity() {
            break 'main;
        }

        if snake.snake_hit_itself() {
            window.clear();
            final_text.push("Game Over....🐍");
            final_text.push("q: Quit");
            final_text.push("n: Try again");
            print_to_center(&window, &final_text);

            window.refresh();
            'end_game:loop {
                match window.getch() {
                    Some(input) => {
                        if input == Input::Character("q".chars().nth(0).unwrap()) {
                            endwin();
                            break 'main;
                        }
                        if input == Input::Character("n".chars().nth(0).unwrap()) {
                            snake.reset();
                            direction = RIGHT;
                            final_text =  vec![];
                            window.clear();
                            window.refresh();
                            break 'end_game;
                        }
                    },
                    None => ()
                }
            }
        }

        if food_is_eaten(snake.head(), &food_coordinate) {
            snake.increase_len();
            food_coordinate = print_random_food_on_board(&window, &mut rng);
        }

        // clean tail of snake
        while snake.capacity_is_exceed() {
            let tail: Coordinate = snake.remove_tail();
            clean_symbol(&window, tail);
        }

        print_head_snake_on_board(&window, &snake);

        // change direction after user click arrow on keyboard
        match window.getch() {
            Some(Input::KeyDown) => {
                if direction != UP { // exclude opposite direction for each current direction
                    direction = DOWN;
                }
            },
            Some(Input::KeyUp) => {
                if direction != DOWN {
                    direction = UP;
                }
            },
            Some(Input::KeyLeft) => {
                if direction != RIGHT {
                    direction = LEFT;
                }
            },
            Some(Input::KeyRight) => {
                if direction != LEFT {
                    direction = RIGHT;
                }
            }
            Some(_input) => (),
            None => ()
        }

        // y
        // |
        // |
        // |
        // |__________ x
        match true {
            _ if snake_hit_right_wall(snake.head().x + direction.x, window.get_max_x()) => {
                snake.next_step(Coordinate { x: 0, y: snake.head().y + direction.y });
            },
            _ if snake_hit_bottom_wall(snake.head().y + direction.y, window.get_max_x()) => {
                snake.next_step(Coordinate { x: snake.head().x + direction.x, y: 0 });
            },
            _ if snake_hit_left_wall(&snake) => {
                snake.next_step(Coordinate { x: window.get_max_x(), y: snake.head().y + direction.y });
            },
            _ if snake_hit_top_wall(&snake) => {
                snake.next_step(Coordinate { x: snake.head().x + direction.x, y: window.get_max_x() / 2 });
            },
            _ => { // snake just moving properly
                snake.next_step(Coordinate { x: snake.head().x + direction.x, y: snake.head().y + direction.y });
            }
        }

        // speed definer - depends on already eaten food - consequently speed will increase
        speed = 200 - (snake.len() * config.speed_coeff);

        thread::sleep(Duration::from_millis(speed as u64));
    }
}


fn food_is_eaten(snake_head: &Coordinate, food: &Coordinate) -> bool {
    snake_head.x == food.x && snake_head.y == food.y
}
// helpers
fn snake_hit_right_wall(x: i32, window_width:i32) -> bool {
    x > 0 && (x % window_width) == 0
}
fn snake_hit_bottom_wall(y: i32, window_width:i32) -> bool {
    y > 0 && y % (window_width / 2) == 0
}

fn snake_hit_left_wall(snake: &Snake) -> bool {
    snake.head().x < 0
}
fn snake_hit_top_wall(snake: &Snake) -> bool {
    snake.head().y < 0
}