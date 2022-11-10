use crate::pre_load::{color::Color, graph::Point};
use std::io::Write;

pub fn draw_cube_flyup(dest: &mut dyn Write, point: Point, color: &Color) {
    write!(dest,"execute if score @p Timer matches {} run particleex conditional minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0.4 0 0.5 0.5 0.5 \"dis>0.7\" 0.1 60\r\n",point.x,point.x,point.y,point.z,color.r,color.g,color.b,color.a).unwrap();
}
