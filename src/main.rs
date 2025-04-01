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


struct Board<'a> {
    window: &'a Window,
    messages:Vec<&'a str>,
    direction: Direction,
    food_coordinate:Coordinate,
    snake: Snake,
    speed:i32,
}
impl <'a> Board<'a>  {
    fn add_text(&mut self, text: &'a str) {
        self.messages.push(text);
    }
    fn generate_food(&mut self) {
        let mut rng = rand::rng();
        self.food_coordinate =  Coordinate{x: rng.random_range(0..self.window.get_max_x()), y: rng.random_range(0..self.window.get_max_y() / 2)};
        self.window.mvprintw(self.food_coordinate.y, self.food_coordinate.x, "*");
        self.window.refresh();
    }

    fn remove_snake_tail(&mut self) {
        let tail: Coordinate = self.snake.remove_tail();
        self.window.mvprintw(tail.y, tail.x, " ");
    }
    fn print_snake_head(&self) {
        self.window.mvprintw(self.snake.head().y , self.snake.head().x, self.snake.sign());
        self.window.refresh();
    }

    fn catch_push_on_keyboard(&mut self ) {
        match self.window.getch() {
            Some(Input::KeyDown) => {
                if self.direction != UP { // exclude opposite direction for each current direction
                    self.direction = DOWN;
                }
            },
            Some(Input::KeyUp) => {
                if self.direction != DOWN {
                    self.direction = UP;
                }
            },
            Some(Input::KeyLeft) => {
                if self.direction != RIGHT {
                    self.direction = LEFT;
                }
            },
            Some(Input::KeyRight) => {
                if self.direction != LEFT {
                    self.direction = RIGHT;
                }
            }
            Some(_input) => (),
            None => ()
        }
    }

    fn add_next_step_for_snake(&mut self) {
        match true {
            _ if snake_hit_right_wall(self.snake.head().x + self.direction.x, self.window.get_max_x()) => {
                self.snake.next_step(Coordinate { x: 0, y: self.snake.head().y + self.direction.y });
            },
            _ if snake_hit_bottom_wall(self.snake.head().y + self.direction.y, self.window.get_max_x()) => {
                self.snake.next_step(Coordinate { x:self.snake.head().x + self.direction.x, y: 0 });
            },
            _ if snake_hit_left_wall(&self.snake) => {
                self.snake.next_step(Coordinate { x: self.window.get_max_x(), y: self.snake.head().y + self.direction.y });
            },
            _ if snake_hit_top_wall(&self.snake) => {
                self.snake.next_step(Coordinate { x: self.snake.head().x + self.direction.x, y: self.window.get_max_x() / 2 });
            },
            _ => { // snake just moving properly
                self.snake.next_step(Coordinate { x: self.snake.head().x + self.direction.x, y: self.snake.head().y + self.direction.y });
            }
        }
    }
    fn print_exit_dialog(&mut self) -> Result<bool, ()>{
        self.window.clear();
        self.messages.push("q: Quit");
        self.messages.push( "n: Try again");

        print_to_center(&self.window, &self.messages);

        loop {
            match self.window.getch() {
                Some(input) => {
                    if input == Input::Character("q".chars().nth(0).unwrap()) {
                        return Ok(false);
                    }
                    if input == Input::Character("n".chars().nth(0).unwrap()) {
                        self.snake.reset();
                        self.direction = RIGHT;
                        self.messages =  vec![];
                        self.window.clear();
                        self.window.refresh();
                        self.generate_food();
                        return Ok(true);
                    }
                },
                None => ()
            }
        }
    }
    fn increase_speed_as_need(&mut self, speed_coeff:i32) {
        self.speed = 200 - (self.snake.len() * speed_coeff);
    }

    fn snake_delay(&self ) -> u64 {
        self.speed as u64
    }
    fn food_is_eaten(&self) -> bool {
        self.snake.head().x == self.food_coordinate.x && self.snake.head().y == self.food_coordinate.y
    }
}
fn main() {
    let mut rng = rand::rng();
    let main_window = initscr();
    start_color();
    use_default_colors();
    curs_set(0); // cursor is invisible
    noecho();
    main_window.refresh();

    let config = show_menu(&main_window);

    let mut board = Board {
        window: &config.board,
        messages:vec![],
        direction: RIGHT,
        food_coordinate: print_random_food_on_board(&config.board, &mut rng),
        snake:Snake::new(1, config.snake_limit_len, "*".to_string()),
        speed:0
    };

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