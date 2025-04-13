use pancurses::{Input, Window};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::introduction::LevelConfig;
use crate::snake::{Coordinate, Snake};

#[derive(PartialEq)]
struct Direction {
    x:i32,
    y:i32,
}

const DOWN:Direction = Direction{y:1 , x:0};
const UP:Direction = Direction{y:-1 , x:0};
const LEFT:Direction = Direction{y:0 , x:-1};
const RIGHT:Direction = Direction{y:0 , x:1};

pub struct Board<'a> {
    window: &'a Window,
    messages:Vec<&'a str>,
    curr_direction: Direction,
    food_coordinate:Coordinate,
    pub snake: Snake,
    speed:i32,
    thread_rng: ThreadRng
}

impl <'a> Board<'a> {
    pub fn new(config: &LevelConfig) -> Board {
        Board {
            window: &config.board,
            messages:vec![],
            curr_direction: RIGHT,
            food_coordinate: Coordinate::default(),
            snake:Snake::new(1, config.snake_limit_len, "*".to_string()),
            speed:0,
            thread_rng:rand::rng()
        }
    }
    pub fn add_text(&mut self, text: &'a str) {
        self.messages.push(text);
    }
    pub fn generate_food(&mut self) {
        self.food_coordinate =  Coordinate{x: self.thread_rng.random_range(0..self.window.get_max_x()), y: self.thread_rng.random_range(0..self.window.get_max_y() / 2)};
        self.window.mvprintw(self.food_coordinate.y, self.food_coordinate.x, "*");
        self.window.refresh();
    }

    pub fn remove_snake_tail(&mut self) {
        let tail: Coordinate = self.snake.remove_tail();
        self.window.mvprintw(tail.y, tail.x, " ");
    }
    pub fn print_snake_head(&self) {
        self.window.mvprintw(self.snake.head().y , self.snake.head().x, self.snake.sign());
        self.window.refresh();
    }

    pub fn catch_push_on_keyboard(&mut self ) {
        match self.window.getch() {
            Some(Input::KeyDown) => {
                if self.curr_direction != UP { // exclude opposite direction for each current direction
                    self.curr_direction = DOWN;
                }
            },
            Some(Input::KeyUp) => {
                if self.curr_direction != DOWN {
                    self.curr_direction = UP;
                }
            },
            Some(Input::KeyLeft) => {
                if self.curr_direction != RIGHT {
                    self.curr_direction = LEFT;
                }
            },
            Some(Input::KeyRight) => {
                if self.curr_direction != LEFT {
                    self.curr_direction = RIGHT;
                }
            }
            Some(_input) => (),
            None => ()
        }
    }

    pub fn add_next_step_for_snake(&mut self) {
        match true {
            _ if self.snake_hit_right_wall() => {
                self.next_step_from_left_wall();
            },
            _ if self.snake_hit_bottom_wall() => {
                self.next_step_from_top_wall()
            },
            _ if self.snake_hit_left_wall() => {
                self.next_step_from_right_wall();
            },
            _ if self.snake_hit_top_wall() => {
                self.next_step_from_bottom_wall();
            },
            _ => { // snake just moving properly
                self.next_step_to_curr_direction();
            }
        }
    }

    fn next_step_from_left_wall(&mut self) {
        self.snake.next_step(Coordinate { x: 0, y: self.snake.head().y + self.curr_direction.y });
    }
    fn next_step_from_top_wall(&mut self) {
        self.snake.next_step(Coordinate { x:self.snake.head().x + self.curr_direction.x, y: 0 });
    }
    fn next_step_from_right_wall(&mut self) {
        self.snake.next_step(Coordinate { x: self.window.get_max_x(), y: self.snake.head().y + self.curr_direction.y });
    }

    fn next_step_from_bottom_wall(&mut self) {
        self.snake.next_step(Coordinate { x: self.snake.head().x + self.curr_direction.x, y: self.window.get_max_x() / 2 });
    }
    fn next_step_to_curr_direction(&mut self) {
        self.snake.next_step(Coordinate { x: self.snake.head().x + self.curr_direction.x, y: self.snake.head().y + self.curr_direction.y });
    }
    fn snake_hit_right_wall(&self) -> bool {
        let next_coordinate = self.snake.head().x + self.curr_direction.x;
        next_coordinate > 0 && (next_coordinate % self.window.get_max_x()) == 0
    }

    fn snake_hit_bottom_wall(&self) -> bool {
        let next_coordinate = self.snake.head().y + self.curr_direction.y;
        next_coordinate > 0 && next_coordinate % ( self.window.get_max_x() / 2) == 0
    }

    fn snake_hit_left_wall(&self) -> bool {
        self.snake.head().x < 0
    }

    fn snake_hit_top_wall(&self) -> bool {
        self.snake.head().y < 0
    }

    pub fn print_exit_dialog(&mut self) -> Result<bool, ()>{
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
                        self.curr_direction = RIGHT;
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
    pub fn increase_speed_as_need(&mut self, speed_coeff:i32) {
        self.speed = 200 - (self.snake.len() * speed_coeff);
    }

    pub fn snake_delay(&self ) -> u64 {
        self.speed as u64
    }
    pub fn food_is_eaten(&self) -> bool {
        self.snake.head().x == self.food_coordinate.x && self.snake.head().y == self.food_coordinate.y
    }
}

pub fn print_to_center(window: &Window, text_slices: &Vec<&str>) {
    let mut line:i32 = window.get_max_y() / 2;

    for text in text_slices {
        window.mvprintw(line,  (window.get_max_x() / 2) - (text.len() as i32 / 2), text);
        line += 2;
    }
    window.refresh();
}