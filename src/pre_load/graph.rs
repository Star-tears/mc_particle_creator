use std::collections::BTreeSet;

#[derive(Debug)]
pub struct TickNode {
    pub tick_id: i64,
    pub pitch_set: BTreeSet<i64>,
}

#[derive(Clone, Debug)]
pub struct PointGroup {
    pub x: f64,
    pub z_list: Vec<f64>,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct Edge {
    pub point1: Point,
    pub point2: Point,
}

impl Point {
    pub fn dist_to(&self, another_point: &Point) -> f64 {
        let off_x = (self.x - another_point.x) as f64;
        let off_y = (self.y - another_point.y) as f64;
        let off_z = (self.z - another_point.z) as f64;
        (off_x * off_x + off_y * off_y + off_z * off_z).sqrt()
    }
}

impl Point {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn points_dist() {
        let p1 = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p2 = Point { x: 1.0, y: 2.0, z: 3.0 };
        let dis = p1.dist_to(&p2);
        assert!((3.74165738677394138558 - dis).abs().le(&1e-10));
    }
}
