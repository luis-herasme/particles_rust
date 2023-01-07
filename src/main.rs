extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod particle;
mod vector;

use std::f32::consts::PI;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use piston::{MouseCursorEvent, PressEvent};

use crate::particle::Particle;
use crate::vector::Vector;
use rand::Rng;

pub const WINDOW_WITH: f64 = 1800.0;
pub const WINDOW_HEIGHT: f64 = 1000.0;
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.9];

pub struct App {
    gl: GlGraphics,
    particles: Vec<Particle>,
    mouse_position: Vector,
    mouse_click: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let particles = &mut self.particles;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Draws the particles
            for particle in particles {
                let x = particle.position.x;
                let y = particle.position.y;

                let vel = (particle.velocity.mag() / 150.0) as f32;
                let color: [f32; 4] = [vel, 0.0, 1.0 - vel, 1.0];
                let transform = c.transform.trans(x, y);

                circle_arc(
                    color,
                    particle.radius,
                    0.0,
                    (2.0 * PI).into(),
                    [0.0, 0.0, 0.0, 0.0],
                    transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for particle in &mut self.particles {
            particle.update_border_collision();
            particle.update(args.dt);
        }

        // When the mouse is clicked, it applies a force to a particle
        if self.mouse_click {
            for particle in &mut self.particles {
                let difference = self.mouse_position.sub(&particle.position);
                if difference.mag() < particle.radius {
                    particle.velocity.add(&Vector { x: 300.0, y: 0.0 });
                }
            }
        }

        // Simulates the gravity
        {
            let particles = &mut self.particles;

            for i0 in 0..particles.len() {
                for i1 in 0..particles.len() {
                    if i0 != i1 {
                        let p1 = &particles[i0];
                        let p2 = &particles[i1];

                        let difference = p1.position.sub(&p2.position);
                        let difference_normal = difference.normalize();
                        let distance = difference.mag();

                        let m1 = 2.0 * 3.14 * p1.radius.powi(2);
                        let m2 = 2.0 * 3.14 * p2.radius.powi(2);

                        let force_mag = m1 * m2 / distance.powi(2);

                        let f1 = difference_normal.mult(force_mag);
                        let f2 = f1.mult(-1.0);

                        let p1 = &mut particles[i0];
                        p1.acceleration.add(&f2.mult(1.0 / m1));

                        let p2 = &mut particles[i1];
                        p2.acceleration.add(&f1.mult(1.0 / m2));
                    }
                }
            }
        }

        // Simulates the collision
        let particles = &mut self.particles;
        for i0 in 0..particles.len() {
            for i1 in 0..particles.len() {
                if i0 != i1 {
                    let p1 = &particles[i0];
                    let p2 = &particles[i1];

                    let m1 = 2.0 * 3.14 * p1.radius.powi(2);
                    let m2 = 2.0 * 3.14 * p2.radius.powi(2);

                    let p1v = particles[i0].velocity.clone();
                    let p2v = particles[i1].velocity.clone();

                    let mut v1 = p1v.mult((m1 - m2) / (m1 + m2));
                    v1.add(&p2v.mult(2.0 * m2 / (m1 + m2)));

                    let mut v2 = p2v.mult((m2 - m1) / (m1 + m2));
                    v2.add(&p1v.mult(2.0 * m1 / (m1 + m2)));

                    let difference = p2.position.sub(&p1.position);
                    let distance = difference.mag();

                    let collision_distance = p1.radius + p2.radius;

                    let p1 = &mut particles[i0];

                    let distance_normal = difference.normalize();

                    if distance <= collision_distance {
                        let space = distance_normal.mult(collision_distance - distance);
                        let space_f1 = space.mult(-0.51);
                        let space_f2 = space.mult(0.51);

                        p1.position.add(&space_f1);
                        p1.velocity.x = v1.x * 0.99;
                        p1.velocity.y = v1.y * 0.99;

                        let p2 = &mut particles[i1];
                        p2.position.add(&space_f2);
                        p2.velocity.x = v2.x * 0.99;
                        p2.velocity.y = v2.y * 0.99;
                    }
                }
            }
        }
        self.mouse_click = false;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Particles", [WINDOW_WITH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut particles: Vec<Particle> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let x: f64 = rng.gen::<f64>() * WINDOW_WITH;
        let y: f64 = rng.gen::<f64>() * WINDOW_HEIGHT;

        let vx: f64 = rng.gen::<f64>() * 100.0 - 50.0;
        let vy: f64 = rng.gen::<f64>() * 100.0 - 50.0;

        let radius: f64 = 5.0 + rng.gen::<f64>() * 10.0;

        particles.push(Particle {
            position: Vector { x, y },
            velocity: Vector { x: vx, y: vy },
            acceleration: Vector { x: 0., y: 0. },
            radius,
        });
    }

    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles,
        mouse_click: false,
        mouse_position: Vector { x: 0.0, y: 0.0 },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_position.x = args[0];
            app.mouse_position.y = args[1];
        }

        if let Some(press) = e.press_args() {
            match press {
                Button::Mouse(key) => match key {
                    _ => {
                        app.mouse_click = true;
                    }
                },
                event => println!("Unknown Event {:?}", event),
            }
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
