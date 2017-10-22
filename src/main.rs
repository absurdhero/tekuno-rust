extern crate piston_window;

use piston_window::*;

// Implementing a foreign trait for a foreign type not allowed by Rust:
//impl std::ops::Mul for piston_window::math::Matrix2d {
//    // The multiplication of rational numbers is a closed operation.
//    type Output = Self;
//
//    fn mul(self, rhs: Self) -> Self {
//        return math::multiply(self, rhs);
//    }
//}

pub struct App {
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, event: &Event, window: &mut PistonWindow, args: &RenderArgs) {
        window.draw_2d(event, |context, graphics| {
            clear([1.0; 4], graphics);

            let rect_size = [100.0, 100.0];
            let rect_midpoint = [rect_size[0] / 2.0, rect_size[1] / 2.0];

            use math::*;

            // translate to the middle of the viewport
            let t = multiply(context.transform, translate([
                args.width as f64 / 2.0 - rect_midpoint[0],
                args.height as f64 / 2.0 - rect_midpoint[1]]));
            // rotate around the midpoint of the square
            let t = multiply(t, translate(rect_midpoint));
            let t = multiply(t, rotate_radians(self.rotation));
            let t = multiply(t, translate(math::mul_scalar(rect_midpoint, -1.0)));

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, rect_size[0], rect_size[1]],
                      t,
                      graphics);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();


    // Create a new game and run it.
    let mut app = App {
        rotation: 0.0
    };

    while let Some(event) = window.next() {

        if let Some(r) = event.render_args() {
            app.render(&event, &mut window, &r);
        }

        if let Some(u) = event.update_args() {
            app.update(&u);
        }
    }
}