extern crate piston_window;

mod render;
mod game;
mod assets;

use render::to_coord;
use game::Game;
use piston_window::{clear, types::Color, Button, PistonWindow, PressEvent, ReleaseEvent, Size, UpdateEvent,WindowSettings,};


const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (scree_width, scree_height) = (50, 55);
    let title: &str = "Pingy Pong";

    let mut window: PistonWindow = WindowSettings::new(
        title.to_string(),
        Size {
            width: to_coord(scree_width as f64),
            height: to_coord(scree_height as f64),
        },
    ).exit_on_esc(true).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("font").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    let mut game: Game = Game::new_game(scree_width, scree_height);
    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args() { game.key_pressed(key); }
        if let Some(Button::Keyboard(_)) = event.release_args() { game.key_released(); }

        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}