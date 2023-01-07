use crate::{vector::Vector, WINDOW_HEIGHT, WINDOW_WITH};

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub radius: f64,
}

impl Particle {
    pub fn update_border_collision(&mut self) {
        if self.position.x + self.radius > WINDOW_WITH.into() {
            self.position.x = WINDOW_WITH as f64 - self.radius;
            self.velocity.x = self.velocity.x * -1.0;
        }

        if self.position.y + self.radius > WINDOW_HEIGHT.into() {
            self.position.y = WINDOW_HEIGHT as f64 - self.radius;
            self.velocity.y = self.velocity.y * -1.0;
        }

        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x = self.velocity.x * -1.0;
        }

        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y = self.velocity.y * -1.0;
        }
    }

    pub fn mass(&self) -> f64 {
        2.0 * 3.14 * self.radius.powi(2)
    }

    pub fn apply_force(&mut self, force: &Vector) {
        let f = force.mult(1.0 / self.mass());
        self.acceleration.add(&f);
    }

    pub fn update(&mut self, dt: f64) {
        self.velocity.add(&self.acceleration);
        let dv = self.velocity.mult(dt);
        self.velocity.mult(0.5);
        self.position.add(&dv);
        self.acceleration.zero();
    }
}
