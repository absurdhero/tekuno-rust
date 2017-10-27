extern crate piston_window;
extern crate sprite;
extern crate find_folder;

use std::rc::Rc;
use std::boxed::Box;
use piston_window::*;
use sprite::*;

pub struct Ship {
    sprite: Box<Sprite<G2dTexture>>,
    velocity: math::Vec2d,

    // controls
    thrusters_on: bool,
    rotating_left: bool,
    rotating_right: bool,
}

impl Ship {
    fn new(sprite: Box<Sprite<G2dTexture>>) -> Self {
        Ship {
            sprite,
            velocity: [0.0, 0.0],
            thrusters_on: false,
            rotating_left: false,
            rotating_right: false,
        }
    }

    fn update_position(&mut self, dt: f64) {
        if self.rotating_left {
            self.rotate(-dt);
        }

        if self.rotating_right {
            self.rotate(dt);
        }

        if self.thrusters_on {
            let theta = self.sprite.get_rotation().to_radians();
            self.velocity[0] += self.thrust() * theta.cos();
            self.velocity[1] += self.thrust() * theta.sin();
        }

        let pos = self.sprite.get_position();
        self.sprite.set_position(
            pos.0 + self.velocity[0], pos.1 + self.velocity[1]);
    }

    /// positive dt rotates right, negative rotates left
    fn rotate(&mut self, dt: f64) {
        let mut new_rotation = self.sprite.get_rotation() + self.rotation_speed() * dt;

        if new_rotation > 360.0 {
            new_rotation -= 360.0;
        }
        if new_rotation < 0.0 {
            new_rotation += 360.0;
        }

        self.sprite.set_rotation(new_rotation)
    }

    #[allow(dead_code)]
    fn radius(&self) -> f64 { 16.0 }
    fn rotation_speed(&self) -> f64 { 270.0 }
    fn thrust(&self) -> f64 { 0.02 }
}

pub struct Player {
    ship: Ship
}

pub struct App {
    player: Player,
}

impl App {
    fn new(window: &mut PistonWindow) -> Self {
        let images = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets/images").unwrap();
        let player_image_path = images.join("player.png");
        let player_tex = Rc::new(Texture::from_path(
            &mut window.factory,
            &player_image_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

        let mut player_sprite = Sprite::from_texture(player_tex.clone());
        player_sprite.set_position(
            window.draw_size().width as f64 / 2.0,
            window.draw_size().height as f64 / 2.0);

        return App {
            player: Player {
                ship: Ship::new(Box::new(player_sprite))
            },
       };
    }

    fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        window.draw_2d(event, |context, graphics| {
            clear([0.0; 4], graphics);

            self.player.ship.sprite.draw(context.transform, graphics);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        self.player.ship.update_position(args.dt);
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
        WindowSettings::new("Tekuno", [800, 600])
            .exit_on_esc(true).build().unwrap();


    let mut app = App::new(&mut window);

    while let Some(event) = window.next() {

        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
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