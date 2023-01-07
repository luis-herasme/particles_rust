#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&mut self, other: &Vector) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mult(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.mag();

        Vector {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
