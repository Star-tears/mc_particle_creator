use rand::Rng;

use crate::pre_load::graph::Point;

pub struct Ve {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Ve {
    /**
     * p1指向p2的向量
     */
    pub fn new(p1: Point, p2: Point) -> Ve {
        Ve {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        }
    }
    pub fn from(p: Point) -> Ve {
        Ve {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl Ve {
    pub fn dot_product(&self, v2: &Ve) -> f64 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
    pub fn get_module(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn add(&self, v2: &Ve) -> Ve {
        Ve {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
        }
    }
    pub fn sub(&self, v2: &Ve) -> Ve {
        Ve {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z,
        }
    }
    pub fn normalized(&mut self) {
        let module = self.get_module();
        self.x /= module;
        self.y /= module;
        self.z /= module;
    }
    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn multi_f64(&self, k: f64) -> Ve {
        Ve {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
    pub fn get_cos_alpha(&self, v2: &Ve) -> f64 {
        self.dot_product(v2) / (self.get_module() * v2.get_module())
    }
}
