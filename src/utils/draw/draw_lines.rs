use rand::Rng;

use crate::pre_load::{color::Color, graph::Point};
use std::io::Write;

pub fn draw_straight_line(dest: &mut dyn Write, point1: Point, point2: Point, color: &Color) {
    let off_x = point2.x - point1.x;
    let off_y = point2.y - point1.y;
    let off_z = point2.z - point1.z;
    let mut k_x: i64 = 1;
    if off_x == 0.0 {
        k_x = 0;
    }
    let dist2 = off_x * off_x + off_y * off_y + off_z * off_z;
    let dist: f64 = (dist2 as f64).sqrt();
    let mut k_y: f64 = (off_y as f64) / dist;
    let mut k_z: f64 = (off_z as f64) / dist;
    if off_x != 0.0 {
        k_y = (off_y as f64) * 1.0 / (off_x as f64);
        k_z = (off_z as f64) * 1.0 / (off_x as f64);
    }
    let mut num_cpt: i64 = 10;
    if off_x != 0.0 {
        num_cpt = (dist / (off_x as f64)) as i64 * 10;
    }
    let num_sep: f64 = 1.0 / (num_cpt as f64);
    if off_x != 0.0 {
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb={}*t,{:.5}*t,{:.5}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" {} {} 25\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_x,k_y,k_z,color.r,color.g,color.b,num_sep,num_cpt).unwrap();
    } else {
        write!(dest,"execute if score @p Timer matches {} run particleex parameter minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0 0 0 {:.5} \"x={}*t;y={:.5}*t;z={:.5}*t;\" {} 25\r\n",point1.x,point1.x,point1.y,point1.z,color.r,color.g,color.b,color.a,dist,k_x,k_y,k_z,num_sep).unwrap();
    }
}

/**
 * 画抛物线
 */
pub fn draw_parabola(dest: &mut dyn Write, point1: Point, point2: Point, color: &Color) {
    let mut rng = rand::thread_rng();
    let off_x = point2.x - point1.x;
    let off_y = point2.y - point1.y;
    let off_z = point2.z - point1.z;
    let k_y2 = rng.gen_range(-0.2..-0.1);
    let k_z: f64 = off_z / off_x;
    if off_x != 0.0 {
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb=t,{:.3}*t*(t-{}),{:.3}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" 0.05 20 25\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_y2,off_x-off_y/(k_y2*off_x),k_z,color.r,color.g,color.b).unwrap();
    }
}

pub fn draw_arc(
    dest: &mut dyn Write,
    point1: Point,
    point2: Point,
    point_center: Point,
    color: &Color,
) {
    if point1.y!=point2.y{
        draw_parabola(dest, point1, point2, color);
        return;
    }
    
}
