// 
// Custom Vector Class(es) (as in Mathematical-Vectors)
// For practice and getting the hang of all the math involved
//

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        return Self {x, y}
    }

    pub fn get_magnitude(&self) -> f64 {
        return f64::sqrt((self.x * self.x) + (self.y * self.y));
    }

    pub fn normalize(&self) -> Vec2 {
        let mag: f64 = self.get_magnitude();

        return Vec2::new(self.x / mag, self.y / mag);
    }

    pub fn  normalize_self(&mut self) -> &mut Self {
        let mag: f64 = self.get_magnitude();

        self.x /= mag;
        self.y /= mag;

        return self;
    }

    pub fn add(&self, operand: Vec2) -> Vec2 {
        return Vec2::new(self.x + operand.x, self.y + operand.y);
    }

    pub fn add_self(&mut self, operand: Vec2) -> &mut Self {
        self.x += operand.x;
        self.y += operand.y;

        return self;
    }

    pub fn scale_by(&self, scaling_factor: f64) -> Vec2 {
        return Vec2::new(self.x * scaling_factor, self.y * scaling_factor);
    }

    pub fn scale_self_by(&mut self, scaling_factor: f64) -> &mut Self {
        self.x *= scaling_factor;
        self.y *= scaling_factor;

        return self;
    }

    pub fn scale_to(&self, scale_to: f64) -> Vec2 {
        let mag = self.get_magnitude();

        let x = (self.x / mag) * scale_to;
        let y = (self.y / mag) * scale_to;

        return Vec2::new(x, y);
    }

    pub fn scale_self_to(&mut self, scale_to: f64) -> &mut Self {
        let mag = self.get_magnitude();

        self.x = (self.x / mag) * scale_to;
        self.y = (self.y / mag) * scale_to;

        return self;
    }
}