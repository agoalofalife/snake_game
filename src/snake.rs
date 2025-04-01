use std::collections::LinkedList;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    body: LinkedList<Coordinate>,
    len:i32,
    capacity:i32,
    sign:String,
}
impl Snake {
    pub fn new(len:i32, capacity:i32, sign:String) -> Snake {
        Snake {
            body: LinkedList::from([Coordinate { x: 1, y: 1 }]), // init point
            len,
            capacity,
            sign
        }
    }

    pub fn reset(&mut self) {
        self.len = 1;
        self.body = LinkedList::from([Coordinate { x: 1, y: 1 }]);
    }
    pub fn snake_hit_itself(&self) -> bool {
        let head:&Coordinate = self.body.back().unwrap();

        for (i, snake_part) in self.body.iter().enumerate() {
            if i != self.body.len() - 1 && head.x == snake_part.x && head.y == snake_part.y {
                return true;
            }
        }
        false
    }
    pub fn len(&self) -> i32 {
        self.len
    }
    pub fn has_reached_capacity(&self) -> bool {
        self.capacity == self.len
    }
    pub fn next_step(&mut self, coordinate: Coordinate) {
        self.body.push_back(coordinate);
    }

    pub fn capacity_is_exceed(&self) -> bool {
        self.body.len() as i32 > self.len
    }

    pub fn head(&self) -> &Coordinate {
        self.body.back().expect("Something went wrong")
    }

    pub fn remove_tail(&mut self) -> Coordinate {
        self.body.pop_front().unwrap()
    }

    pub fn increase_len(&mut self) {
        self.len += 1
    }

    pub fn sign(&self) -> String {
        self.sign.clone()
    }
}