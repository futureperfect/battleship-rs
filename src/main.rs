use piston_window::*;

use piston_window::types::Color;

const BACKGROUND_COLOR: Color = [0.0, 0.1, 0.8, 0.7];
const SHIP_COLOR: Color = [0.7; 4];

fn main() {
    let (width, height) = (400, 400);

    let mut window: PistonWindow = WindowSettings::new("Battleship", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let key = match key {
                Key::Up => "UP",
                Key::Down => "DOWN",
                Key::Left => "LEFT",
                Key::Right => "RIGHT",
                _ => "¯\\_(ツ)_/¯",
            };

            println!("Pressed: {}", key);
        }

        window.draw_2d(&event, |context, graphics| {
            clear(BACKGROUND_COLOR, graphics);
            let ship = [50_f64, 50_f64, 100_f64, 20_f64];
            rectangle(SHIP_COLOR, ship, context.transform, graphics);
        });
    }
}
