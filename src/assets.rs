use crate::render::{draw_block, draw_rectangle};
use piston_window::{types::Color, Context, G2d};

const PLAYER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const BALL_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum MovementDirection {
    Up,
    Down,
}

pub struct Paddle {
    _position_x: f64,
    _position_y: f64,
    _paddle_size: i32,
}

impl Paddle {
    pub fn new(_position_x: f64, _position_y: f64, _paddle_size: i32) -> Self {     // ①
        Self { _position_x, _position_y, _paddle_size }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {    // ②
        draw_rectangle(
            PLAYER_COLOR,
            self._position_x as f64,
            self._position_y as f64,
            1,
            self._paddle_size,
            con,
            g,
        );
    }

    pub fn slide(&mut self, dir: Option<MovementDirection>, min_y: f64, max_y: f64) {   // ③
        let mut new_y: Option<f64> = None;
        if let Some(dir) = dir {
            if dir == MovementDirection::Up {
                let next_y = self._position_y - 1.0;
                if next_y > min_y {
                    new_y = Some(next_y);
                }
            } else if dir == MovementDirection::Down {
                let next_y = self._position_y + 1.0;
                if next_y + (self._paddle_size as f64) < max_y {
                    new_y = Some(next_y);
                }
            }
        }

        if let Some(new_y) = new_y {
            self._position_y = new_y
        }
    }

    pub fn get_position_y(&self) -> f64 {
        self._position_y
    }

    pub fn get_position_x(&self) -> f64 {
        self._position_x
    }

    pub fn get_size(&self) -> i32 {
        self._paddle_size
    }
}



pub struct Ball {
    _x: f64,
    _y: f64,
    _velocity_x: f64,
    _velocity_y: f64,
}

impl Ball {
    pub fn new(_x: f64, _y: f64, _velocity_x: f64, _velocity_y: f64) -> Self {
        Self { _x, _y, _velocity_x, _velocity_y }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self._x, self._y, con, g);
    }

    pub fn set_position(&mut self, _x: f64, _y: f64) {    // ①
        self._x = _x;
        self._y = _y;
    }

    pub fn get_next_location(&self, delta_time: f64) -> (f64, f64) {    // ②
        let distance_x = self._velocity_x * delta_time;
        let distance_y = self._velocity_y * delta_time;
        let new_x = self._x + distance_x;
        let new_y = self._y + distance_y;
        (new_x, new_y)
    }

    pub fn flip_velocity_y(&mut self) {  // ③
        self._velocity_y *= -1.0;
    }

    pub fn flip_velocity_x(&mut self) {  
        self._velocity_x *= -1.0;
    }

    pub fn increase_y(&mut self, factor: f64) {     // ④ 
        self._velocity_y += factor;
    }

    pub fn get_velocity_x(&self) -> f64 {   // ⑤
        self._velocity_x
    }

    pub fn set_velocity(&mut self, _velocity_x: f64, _velocity_y: f64) {  // ⑥
        self._velocity_x = _velocity_x;
        self._velocity_y = _velocity_y;
    }
}
