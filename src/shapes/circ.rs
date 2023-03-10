use super::area::Area;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![
                (self.x, self.y),
                (self.x + self.radius, self.y),
                (self.x + self.radius, self.y + self.radius),
                (self.x, self.y + self.radius),
            ].into();
        }
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        return distance <= self.radius;
    }
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        return distance <= self.radius;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
}