use piston_window::*;
use rand::prelude::*;
use std::time::*;

extern crate spatial;
use spatial::linear_quadtree::*;
use spatial::core::*;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 1024;

const VEL_MULT: f32 = 50.0;
const CURSOR_RADIUS: f64 = 10.0;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    pub position: Point2D,
    pub velocity: [f32; 2]
}

impl Spatial2D for Point {
    fn x(&self) -> f32 {
        self.position.x()
    }

    fn y(&self) -> f32 {
        self.position.y()
    }
}

impl Point {
    fn new(x: f32, y: f32, vel: [f32; 2]) -> Point {
        Point {
            position: Point2D::new(x, y),
            velocity: vel
        }
    }
}

fn main() {

    let mut keys = Vec::new();

    let mut window: PistonWindow =
        WindowSettings::new("Quadtree demo!", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .exit_on_esc(true).build().unwrap();

    let mut quadtree = LinearQuadtree::<Point>::new(Bounds::new(
        0., SCREEN_WIDTH as f32, 0., SCREEN_HEIGHT as f32));

    let mut rng = rand::thread_rng();

    let mut cursor = [0.0, 0.0];
    let mut dt: f32 = 0.008;

    let circle = Ellipse::new([0., 0., 1., 0.5]);
    let mut run = true;

    while let Some(event) = window.next() {

        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                keys.push(quadtree.insert(Point::new(
                    cursor[0] as f32, cursor[1] as f32,
                    [rng.gen::<f32>() * VEL_MULT,
                        rng.gen::<f32>() * VEL_MULT]
                )));
            } else if button == MouseButton::Right {
                if let Some(key) = keys.pop() {
                    quadtree.remove(key);
                }
            }

        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == piston_window::Key::Space {
                run = !run;
            }
            if key == piston_window::Key::Return {
                for bound in quadtree.bounds() {
                    println!("Bound: {:?}", bound);
                }
                println!("----------------------------------")
            }
        }

        if let Some(update) = event.update_args() {
            dt = update.dt as f32;
            //println!("{}", dt);
        }

        event.mouse_cursor(|pos| cursor = pos);

        //let bounds = quadtree.bounds;

//        if run {
//            for point in quadtree.values_mut() {
//                point.position.x += point.velocity[0] * dt;
//                point.position.y += point.velocity[1] * dt;
//
//                if point.position.x >= bounds.x_max || point.position.x <= bounds.x_min {
//                    point.velocity[0] = -point.velocity[0]
//                }
//
//                if point.position.y >= bounds.y_max || point.position.y <= bounds.y_min {
//                    point.velocity[1] = -point.velocity[1]
//                }
//            }
//            quadtree.rebuild_tree();
//        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            // draw all points
            for point in quadtree.values() {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [(point.x() - 2.5) as f64, (point.y()-2.5) as f64, 5.0, 5.0],
                          context.transform,
                          graphics);
            }

            // draw bounds
            for bound in quadtree.bounds() {
                draw_bounds(bound,
                             [0., 0., 0., 1.],
                            context.transform,
                            graphics
                );
            }

            for bound in quadtree.bounds_no_branch() {
                draw_bounds(bound,
                            [1., 0., 0., 1.],
                            context.transform,
                            graphics
                );
            }

            // draw cursor
            circle.draw([cursor[0] - CURSOR_RADIUS / 2.,
                            cursor[1] - CURSOR_RADIUS / 2.,
                            CURSOR_RADIUS ,
                            CURSOR_RADIUS],
                        &DrawState::new_alpha(),
                        context.transform,
                        graphics,)
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
    line(color, 0.5, [x_min, y_min, x_max, y_min], transform, g);
    line(color, 0.5, [x_min, y_max, x_max, y_max], transform, g);
    line(color, 0.5, [x_min, y_min, x_min, y_max], transform, g);
    line(color, 0.5, [x_max, y_min, x_max, y_max], transform, g);
}