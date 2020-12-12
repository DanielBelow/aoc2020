use parse_display::{Display as PDisplay, FromStr as PFromStr};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub dir: Direction,
    pub x_pos: i64,
    pub y_pos: i64,
}

#[derive(PDisplay, PFromStr, Copy, Clone)]
pub enum NavigationAction {
    #[display("N{0}")]
    North(i64),

    #[display("S{0}")]
    South(i64),

    #[display("E{0}")]
    East(i64),

    #[display("W{0}")]
    West(i64),

    #[display("L{0}")]
    Left(i64),

    #[display("R{0}")]
    Right(i64),

    #[display("F{0}")]
    Forward(i64),
}

pub trait Ship {
    fn perform_action(&mut self, action: &NavigationAction);
    fn get_distance(&self) -> i64;
}

#[derive(Copy, Clone)]
pub struct SimpleShip {
    position: Position,
}

impl SimpleShip {
    pub fn new() -> Self {
        let position = Position {
            dir: Direction::East,
            x_pos: 0,
            y_pos: 0,
        };

        Self { position }
    }

    fn turn(&mut self, steps: i64) {
        let turned = ((self.position.dir as i64 + steps) + 4) % 4;
        if let Some(new_dir) = FromPrimitive::from_i64(turned) {
            self.position.dir = new_dir;
        }
    }

    fn move_forward(&mut self, steps: i64) {
        match self.position.dir {
            Direction::North => self.position.y_pos += steps,
            Direction::South => self.position.y_pos -= steps,
            Direction::East => self.position.x_pos += steps,
            Direction::West => self.position.x_pos -= steps,
        };
    }
}

impl Ship for SimpleShip {
    fn perform_action(&mut self, action: &NavigationAction) {
        match *action {
            NavigationAction::North(n) => self.position.y_pos += n,
            NavigationAction::South(n) => self.position.y_pos -= n,
            NavigationAction::East(n) => self.position.x_pos += n,
            NavigationAction::West(n) => self.position.x_pos -= n,
            NavigationAction::Left(n) => self.turn(-n / 90),
            NavigationAction::Right(n) => self.turn(n / 90),
            NavigationAction::Forward(n) => self.move_forward(n),
        };
    }

    fn get_distance(&self) -> i64 {
        self.position.x_pos.abs() + self.position.y_pos.abs()
    }
}

#[derive(Copy, Clone)]
struct Waypoint {
    x_pos: i64,
    y_pos: i64,
}

#[derive(Copy, Clone)]
pub struct ShipWithWaypoint {
    position: Position,
    waypoint: Waypoint,
}

impl Waypoint {
    fn rotate(&mut self, degrees: i64, around: &Position) {
        let cur_x = self.x_pos - around.x_pos;
        let cur_y = self.y_pos - around.y_pos;

        let (s, c) = (degrees as f64).to_radians().sin_cos();

        let cur_x = cur_x as f64;
        let cur_y = cur_y as f64;

        let new_x = (c * cur_x - s * cur_y).round() as i64;
        let new_y = (s * cur_x + c * cur_y).round() as i64;

        self.x_pos = new_x + around.x_pos;
        self.y_pos = new_y + around.y_pos;
    }
}

impl ShipWithWaypoint {
    pub fn new() -> Self {
        let position = Position {
            dir: Direction::East,
            x_pos: 0,
            y_pos: 0,
        };

        let waypoint = Waypoint {
            x_pos: 10,
            y_pos: 1,
        };

        Self { position, waypoint }
    }

    fn move_forward(&mut self, steps: i64) {
        let x_steps = self.waypoint.x_pos - self.position.x_pos;
        let y_steps = self.waypoint.y_pos - self.position.y_pos;

        let to_move_x = steps * x_steps;
        let to_move_y = steps * y_steps;

        self.position.x_pos += to_move_x;
        self.position.y_pos += to_move_y;

        let new_waypoint = Waypoint {
            x_pos: to_move_x + self.waypoint.x_pos,
            y_pos: to_move_y + self.waypoint.y_pos,
        };

        self.waypoint = new_waypoint;
    }
}

impl Ship for ShipWithWaypoint {
    fn perform_action(&mut self, action: &NavigationAction) {
        match *action {
            NavigationAction::North(n) => self.waypoint.y_pos += n,
            NavigationAction::South(n) => self.waypoint.y_pos -= n,
            NavigationAction::East(n) => self.waypoint.x_pos += n,
            NavigationAction::West(n) => self.waypoint.x_pos -= n,
            NavigationAction::Left(n) => self.waypoint.rotate(n, &self.position),
            NavigationAction::Right(n) => self.waypoint.rotate(-n, &self.position),
            NavigationAction::Forward(n) => self.move_forward(n),
        };
    }

    fn get_distance(&self) -> i64 {
        self.position.x_pos.abs() + self.position.y_pos.abs()
    }
}
