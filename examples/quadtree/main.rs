use piston_window::*;

extern crate spatial;
use spatial::*;

const SCREEN_WIDTH: u32 = 512;
const SCREEN_HEIGHT: u32 = 512;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Quadtree demo!", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .exit_on_esc(true).build().unwrap();

    let mut quadtree = Quadtree::<Point2D>::new(Bounds::new(
        0., SCREEN_WIDTH as f32, 0., SCREEN_HEIGHT as f32));

    let mut cursor = [0.0, 0.0];

    while let Some(event) = window.next() {

        if let Some(Button::Mouse(button)) = event.press_args() {
            quadtree.insert(Point2D::new(cursor[0] as f32, cursor[1] as f32));
        }
        event.mouse_cursor(|pos| cursor = pos);

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            for point in &quadtree.container {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [point.x() as f64, point.y() as f64, 5.0, 5.0],
                          context.transform,
                          graphics);
            }

            for bound in quadtree.bounds() {
                draw_bounds(bound,
                            [0.0, 0.0, 0.0, 1.0],
                            context.transform,
                            graphics
                );
            }
        });
    }
}

pub fn draw_bounds<G>(bound: Bounds, color: types::Color, transform: math::Matrix2d, g: &mut G)
    where G: Graphics
{
    let Bounds {x_min, x_max, y_min, y_max} = bound;
    let x_min = x_min as f64;
    let x_max = x_max as f64;
    let y_min = y_min as f64;
    let y_max = y_max as f64;
    line(color, 1., [x_min, y_min, x_max, y_min], transform, g);
    line(color, 1., [x_min, y_max, x_max, y_max], transform, g);
    line(color, 1., [x_min, y_min, x_min, y_max], transform, g);
    line(color, 1., [x_max, y_min, x_max, y_max], transform, g);
}