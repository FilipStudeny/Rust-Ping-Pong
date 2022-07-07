use crate::render::{draw_rectangle, draw_text};
use crate::assets::{Ball, Paddle, MovementDirection};
use piston_window::{types::Color, Context, G2d, Glyphs, Key};

const BORDER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const GAMEOVER_COLOR: Color = [0.80, 0.0, 0.0, 0.5];

const TEXT_AREA_HEIGHT: f64 = 5.0;
const GAME_SPEED: f64 = 0.02;
const RESTART_TIME: f64 = 1.0;



pub struct Game {
    player: Paddle,
    enemy: Paddle,
    ball: Ball,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    ai_response_time: f64,
    ai_update_time: f64,

    active_key: Option<Key>,
    score: i32,

    ball_owner: String,
    BALL_COLOR: Color,
    BALL_COLOR2: Color,
}

impl Game {
    pub fn new_game(width: i32, height: i32) -> Self {
        Self {
            player: Paddle::new(width as f64 - 3.0, TEXT_AREA_HEIGHT + 5.0, 5),
            enemy: Paddle::new(3.0, TEXT_AREA_HEIGHT + 9.0, 5),
            waiting_time: 0.0,
            ai_response_time: 0.01,
            ai_update_time: 0.0,
            ball: Ball::new(6.0, TEXT_AREA_HEIGHT + 4.0, 100.0, 0.0),
            width,
            height,
            game_over: false,
            active_key: None,
            score: 0,
            ball_owner: ("Enemy").to_string(),
            BALL_COLOR: [1.0, 1.0, 1.0, 1.0],
            BALL_COLOR2: [0.0, 0.5, 0.8, 1.0],
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        self.active_key = Some(key);
    }

    pub fn key_released(&mut self) {
        self.active_key = None;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, cache: &mut Glyphs) {
        self.player.draw(con, g);
        self.enemy.draw(con, g);

        // Draw ball
        if !self.game_over {
            self.ball.draw(con, g);
        }

        draw_rectangle(BORDER_COLOR, 0.0, TEXT_AREA_HEIGHT, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0.0, (self.height - 1) as f64, self.width, 1, con, g,);
        draw_rectangle(BORDER_COLOR, 0.0, TEXT_AREA_HEIGHT, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, (self.width - 1) as f64, TEXT_AREA_HEIGHT, 1, self.height, con, g,);

        draw_text(std::format!("SCORE: {}", self.score).as_str(), 5.0, con, g, cache,);
        draw_text(std::format!("BALL HIT BY: {}", self.ball_owner).as_str(), 20.0, con, g, cache,);

        if self.game_over {
            draw_rectangle( GAMEOVER_COLOR, 0.0, TEXT_AREA_HEIGHT, self.width, self.height, con, g,);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.waiting_time > GAME_SPEED {
            self.update_ball(delta_time);
            self.update_player(self.get_dir());
            self.update_ai(delta_time);
            self.waiting_time = 0.0;
        }
    }

    fn update_ball(&mut self, delta_time: f64) {
        let (next_x, next_y) = self.ball.get_next_location(delta_time);

        if next_x > self.width as f64 || next_x < 0.0 {
            // Horizontal wall hit
            self.game_over = true;
            if next_x > self.player.get_position_x() + 1_f64 {
                // GAME OVER

                self.game_over = true;
            } else {
                self.ball.set_velocity(100.0, 0.0);
                self.ball.set_position(6.0, 6.0 + TEXT_AREA_HEIGHT);
                self.score += 1;
            }
        }

        if self.game_over {
            return;
        }

        if next_y > (self.height - 1) as f64 || next_y < TEXT_AREA_HEIGHT + 1.0 {
            // Vertical wall hit
            // change y velocity
            self.ball.flip_velocity_y();
        }

        // Collision Detection
        // Player collision
        if next_x.floor() >= (self.player.get_position_x() - 1.0)
            && next_y >= self.player.get_position_y()
            && next_y <= self.player.get_position_y() + self.player.get_size() as f64
        {
            let paddle_center = self.player.get_position_y() + (self.player.get_size() / 2) as f64;
            let d = paddle_center as f64 - next_y;
            self.ball.flip_velocity_x();
            self.ball.increase_y(d * -20.0);
            self.ball_owner = String::from("Player");
        }

        // AI collision
        if next_x.ceil() <= (self.enemy.get_position_x() + 1.0)
            && next_y >= self.enemy.get_position_y()
            && next_y <= self.enemy.get_position_y() + self.enemy.get_size() as f64
        {
            let paddle_center = self.enemy.get_position_y() + (self.enemy.get_size() / 2) as f64;
            let d = paddle_center as f64 - next_y;
            self.ball.flip_velocity_x();
            self.ball.increase_y(d * -20.0);
            self.ball_owner = String::from("Enemy");

        }

        self.ball.set_position(next_x, next_y);
    }

    fn update_player(&mut self, dir: Option<MovementDirection>) {
        let min_y = TEXT_AREA_HEIGHT;
        let max_y = self.height as f64;
        self.player.slide(dir, min_y, max_y);
    }

    fn get_dir(&self) -> Option<MovementDirection> {
        match self.active_key {
            Some(Key::Up) => Some(MovementDirection::Up),
            Some(Key::Down) => Some(MovementDirection::Down),
            _ => None,
        }
    }

    fn update_ai(&mut self, delta_time: f64) {
        self.ai_update_time += delta_time;
        if self.ai_update_time < self.ai_response_time {
            return;
        }
        self.ai_update_time = 0.0;

        let (_, next_y) = self.ball.get_next_location(delta_time);

        let mut dir: Option<MovementDirection> = None;
        if self.ball.get_velocity_x() < 0.0 {
            if next_y < self.enemy.get_position_y() {
                dir = Some(MovementDirection::Up);
            } else if next_y > self.enemy.get_position_y() + self.enemy.get_size() as f64 {
                dir = Some(MovementDirection::Down);
            }
        }

        let min_y = TEXT_AREA_HEIGHT;
        let max_y = self.height as f64;
        self.enemy.slide(dir, min_y, max_y);
    }

    fn restart(&mut self) {
        //HELLOOOOO
        if self.ball_owner == "Enemy" && self.score > 0{
            self.score -= 1;
        }
        
        self.ball_owner = ("Enemy").to_string();

        self.waiting_time = 0.0;
        self.ball.set_velocity(100.0, 0.0);
        self.ball.set_position(6.0, (self.height / 2) as f64);
        self.game_over = false;
    }
}