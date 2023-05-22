use crate::custom_vec::*;
use crate::logger::*;
use winit::dpi::PhysicalSize;

pub struct Ball {
    pos: Vec2,
    dir: Vec2,
    speed: f64,
    r: f64,
}

impl Ball {
    pub fn new(pos_x: f64, pos_y: f64, dir_x: f64, dir_y: f64, speed: f64, radius: f64) -> Self {
        Logger::log_info(LogLevel::High, "\tBall - Creating Ball");
        return Self {
            pos: Vec2::new(pos_x, pos_y),
            dir: Vec2::new(dir_x, dir_y),
            speed: speed,
            r: radius,
        }
    }

    pub fn update(&mut self) -> &mut Self {
        self.pos.add_self(self.dir.scale_to(self.speed));

        return self
    }

    pub fn change_dir(&mut self, dir_x: f64, dir_y: f64) -> &mut Self {
        self.dir.x = dir_x;
        self.dir.y = dir_y;

        return self;
    }

    pub fn change_dir_vec(&mut self, new_dir: Vec2) -> &mut Self {
        self.dir.x = new_dir.x;
        self.dir.y = new_dir.y;

        return self;
    }

    pub fn as_physical_size(&self) -> PhysicalSize<f64> {
        return PhysicalSize::<f64>::new(self.pos.x, self.pos.y);
    }
}