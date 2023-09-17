use log::{error, trace};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    /// Position of the top left corner in meters
    pub pos: Vec2,
    // Size in meters
    pub size: Vec2,
    /// Velocity in meters per second
    pub vel: Vec2,
}

#[derive(Debug)]
pub struct Simulation {
    size: Vec2,
    rectangles: Vec<Rectangle>,
}

#[derive(Debug)]
pub enum PhysicsError {
    Overlap,
}

impl Simulation {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            rectangles: Vec::new(),
        }
    }

    pub fn get_rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }

    pub fn add_rectangle(&mut self, rectangle: Rectangle) -> Result<(), PhysicsError> {
        // check that it doesn't overlap with any other rectangle
        for other in self.rectangles.iter() {
            if Self::overlap(&rectangle, other) {
                error!(
                    "Rectangle {:?} overlaps with rectangle {:?}",
                    rectangle, other,
                );
                return Err(PhysicsError::Overlap);
            }
        }
        self.rectangles.push(rectangle);
        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) {
        for rectangle in self.rectangles.iter_mut() {
            let next = Vec2 {
                x: rectangle.pos.x + rectangle.vel.x * delta_time,
                y: rectangle.pos.y + rectangle.vel.y * delta_time,
            };

            // bounce on walls
            if next.x < 0.0 || next.x + rectangle.size.x > self.size.x {
                trace!("Rectangle {rectangle:?} bounced on a wall.");
                rectangle.vel.x *= -1.0;
            }
            if next.y < 0.0 || next.y + rectangle.size.y > self.size.y {
                trace!("Rectangle {rectangle:?} bounced on a wall.");
                rectangle.vel.y *= -1.0;
            }
            rectangle.pos.x += rectangle.vel.x * delta_time;
            rectangle.pos.y += rectangle.vel.y * delta_time;
        }
    }

    fn overlap(a: &Rectangle, b: &Rectangle) -> bool {
        a.pos.x < b.pos.x + b.size.x
            && a.pos.x + a.size.x > b.pos.x
            && a.pos.y < b.pos.y + b.size.y
            && a.pos.y + a.size.y > b.pos.y
    }
}
