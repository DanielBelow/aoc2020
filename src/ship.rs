use num_complex::Complex;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

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
pub struct Simple {
    position: Complex<i64>,
    direction: Complex<i64>,
}

impl Simple {
    pub const fn new() -> Self {
        Self {
            position: Complex::new(0, 0),
            direction: Complex::new(1, 0),
        }
    }

    fn turn_left(&mut self, steps: i64) {
        for _ in 0..steps {
            self.direction *= Complex::new(0, 1);
        }
    }

    fn turn_right(&mut self, steps: i64) {
        for _ in 0..steps {
            self.direction *= Complex::new(0, -1);
        }
    }

    fn move_forward(&mut self, steps: i64) {
        self.position += self.direction.scale(steps);
    }
}

impl Ship for Simple {
    fn perform_action(&mut self, action: &NavigationAction) {
        match *action {
            NavigationAction::North(n) => self.position.im += n,
            NavigationAction::South(n) => self.position.im -= n,
            NavigationAction::East(n) => self.position.re += n,
            NavigationAction::West(n) => self.position.re -= n,
            NavigationAction::Left(n) => self.turn_left(n / 90),
            NavigationAction::Right(n) => self.turn_right(n / 90),
            NavigationAction::Forward(n) => self.move_forward(n),
        };
    }

    fn get_distance(&self) -> i64 {
        self.position.l1_norm()
    }
}

#[derive(Copy, Clone)]
struct Waypoint {
    position: Complex<i64>,
}

#[derive(Copy, Clone)]
pub struct WithWaypoint {
    position: Complex<i64>,
    waypoint: Waypoint,
}

impl Waypoint {
    #[allow(clippy::cast_precision_loss)]
    fn rotate(&mut self, degrees: i64, around: &Complex<i64>) {
        let cur_pos = self.position - around;

        let (s, c) = (degrees as f64).to_radians().sin_cos();

        let cur_x = cur_pos.re as f64;
        let cur_y = cur_pos.im as f64;

        let new_x = c.mul_add(cur_x, -(s * cur_y)).round() as i64;
        let new_y = s.mul_add(cur_x, c * cur_y).round() as i64;

        let new_pos = Complex::new(new_x, new_y);

        self.position = new_pos + around;
    }
}

impl WithWaypoint {
    pub const fn new() -> Self {
        Self {
            position: Complex::new(0, 0),
            waypoint: Waypoint {
                position: Complex::new(10, 1),
            },
        }
    }

    fn move_forward(&mut self, steps: i64) {
        let p_steps = self.waypoint.position - self.position;

        let to_move = p_steps.scale(steps);

        self.position += to_move;
        self.waypoint.position += to_move;
    }
}

impl Ship for WithWaypoint {
    fn perform_action(&mut self, action: &NavigationAction) {
        match *action {
            NavigationAction::North(n) => self.waypoint.position.im += n,
            NavigationAction::South(n) => self.waypoint.position.im -= n,
            NavigationAction::East(n) => self.waypoint.position.re += n,
            NavigationAction::West(n) => self.waypoint.position.re -= n,
            NavigationAction::Left(n) => self.waypoint.rotate(n, &self.position),
            NavigationAction::Right(n) => self.waypoint.rotate(-n, &self.position),
            NavigationAction::Forward(n) => self.move_forward(n),
        };
    }

    fn get_distance(&self) -> i64 {
        self.position.l1_norm()
    }
}
