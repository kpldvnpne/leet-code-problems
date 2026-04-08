pub struct Solution;

fn main() {}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => { "North".into() },
            Direction::East => { "East".into() },
            Direction::South => { "South".into() },
            Direction::West => { "West".into() }
        }
    }
}

struct Robot {
    _width: u32,
    _height: u32,
    _pos_x: u32,
    _pos_y: u32,
    _dir: Direction,
    _move: u32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Self {
            _width: width as u32,
            _height: height as u32,
            _pos_x: 0 as u32,
            _pos_y: 0 as u32,
            _dir: Direction::East,
            _move: 0
        }
    }

    fn step(&mut self, num: i32) {
        self._move += num as u32;
    }

    fn actual_step(&mut self, num: u32) {
        let one_round = (self._width - 1 + self._height - 1) * 2;

        if num >= one_round && num % one_round == 0 {
            self._dir = match (self._pos_x, self._pos_y) {
                (0, 0) => Direction::South,
                (x, 0) if x == self._width - 1 => Direction::East,
                (x, y) if x == self._width - 1 && y == self._height - 1 => Direction::North,
                (0, y) if y == self._height - 1 => Direction::West,
                _ => self._dir.clone(), // Keep the current direction if not at a corner
            };
            return;
        }

        let mut steps = num as u32 % one_round;

        while steps != 0 {
            match self._dir {
                Direction::East => {
                    let max_steps = self._width - 1 - self._pos_x;
                    if steps > max_steps {
                        self._pos_x += max_steps;
                        steps -= max_steps;
                        self._dir = Direction::North;
                    } else {
                        self._pos_x += steps;
                        steps = 0;
                    }
                },
                Direction::North => {
                    let max_steps = self._height - 1 - self._pos_y;
                    if steps > max_steps {
                        self._pos_y += max_steps;
                        steps -= max_steps;
                        self._dir = Direction::West;
                    } else {
                        self._pos_y += steps;
                        steps = 0;
                    }
                },
                Direction::West => {
                    let max_steps = self._pos_x;
                    if steps > max_steps {
                        self._pos_x -= max_steps;
                        steps -= max_steps;
                        self._dir = Direction::South;
                    } else {
                        self._pos_x -= steps;
                        steps = 0;
                    }
                },
                Direction::South => {
                    let max_steps = self._pos_y;
                    if steps > max_steps {
                        self._pos_y -= max_steps;
                        steps -= max_steps;
                        self._dir = Direction::East;
                    } else {
                        self._pos_y -= steps;
                        steps = 0;
                    }
                },
            }
        }
    }

    fn get_pos(&mut self) -> Vec<i32> {
        if self._move > 0 {
            self.actual_step(self._move);
            self._move = 0;
        }
        return vec![self._pos_x as i32, self._pos_y as i32];
    }

    fn get_dir(&mut self) -> String {
        if self._move > 0 {
            self.actual_step(self._move);
            self._move = 0;
        }
        return self._dir.to_string();
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
