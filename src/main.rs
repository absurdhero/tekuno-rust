extern crate piston_window;

use piston_window::*;

#[allow(dead_code)]
pub struct Ship {
    // motion
    position: math::Vec2d,
    rotation: f64,
    velocity: math::Vec2d,

    // controls
    thrusters_on: bool,
    rotating_left: bool,
    rotating_right: bool,
}

impl Default for Ship {
    fn default() -> Ship {
        Ship {
            position: [0.0, 0.0],
            rotation: 0.0,
            velocity: [0.0, 0.0],
            thrusters_on: false,
            rotating_left: false,
            rotating_right: false,
        }
    }
}

impl Ship {
    fn rotation_speed(&self) -> f64 { 2.0 }
    fn radius(&self) -> f64 { 20.0 }
}

pub struct Player {
    ship: Ship
}

pub struct App {
    player: Player
}

impl App {
    fn render(&mut self, event: &Event, window: &mut PistonWindow, args: &RenderArgs) {
        window.draw_2d(event, |context, graphics| {
            clear([1.0; 4], graphics);

            let rect_size = [self.player.ship.radius() * 2.0, self.player.ship.radius() * 2.0];
            let rect_midpoint = [rect_size[0] / 2.0, rect_size[1] / 2.0];

            use math::*;

            // translate to the middle of the viewport
            let t = multiply(context.transform, translate([
                args.width as f64 / 2.0 - rect_midpoint[0],
                args.height as f64 / 2.0 - rect_midpoint[1]]));
            // rotate around the midpoint of the square
            let t = multiply(t, translate(rect_midpoint));
            let t = multiply(t, rotate_radians(self.player.ship.rotation));
            let t = multiply(t, translate(math::mul_scalar(rect_midpoint, -1.0)));

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, rect_size[0], rect_size[1]],
                      t,
                      graphics);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        let player_ship = &mut self.player.ship;

        if player_ship.rotating_left {
            player_ship.rotation -= player_ship.rotation_speed() * args.dt;
        }

        if player_ship.rotating_right {
            player_ship.rotation += player_ship.rotation_speed() * args.dt;
        }
    }

    fn key_down(&mut self, state: ButtonState, key: Key) {
        match key {
            Key::Up => self.player.ship.thrusters_on = state == ButtonState::Press,
            Key::Left => self.player.ship.rotating_left = state == ButtonState::Press,
            Key::Right => self.player.ship.rotating_right = state == ButtonState::Press,
            _ => println!("{:?}ed keyboard key '{:?}'", state, key)
        }
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Tekuno", [640, 480])
            .exit_on_esc(true).build().unwrap();


    let mut app = App {
        player: Player { ship: Default::default() }
    };

    while let Some(event) = window.next() {

        if let Some(r) = event.render_args() {
            app.render(&event, &mut window, &r);
        }

        if let Some(u) = event.update_args() {
            app.update(&&u);
        }

        if let Some(args) = event.button_args() {
            match args.button {
                Button::Keyboard(key) => app.key_down(args.state, key),
                _ => {}
            }
        };
    }
}