use rand::Rng;

use crate::{
    pre_load::{color::Color, graph::Point},
    utils::math::{cal, ve::Ve},
};
use std::{f64::consts::PI, io::Write, vec};

pub fn draw_straight_line(dest: &mut dyn Write, point1: Point, point2: Point, color: &Color) {
    let off_x = point2.x - point1.x;
    let off_y = point2.y - point1.y;
    let off_z = point2.z - point1.z;
    let mut k_x: i64 = 1;
    if cal::eq_f64(off_x, 0.0) {
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
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb={}*t,{:.5}*t,{:.5}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" {} {} 50\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_x,k_y,k_z,color.r,color.g,color.b,num_sep,num_cpt).unwrap();
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
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb=t,{:.3}*t*(t-{}),{:.3}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" 0.05 20 50\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_y2,off_x-off_y/(k_y2*off_x),k_z,color.r,color.g,color.b).unwrap();
    }
}
/**
 * 画弧线，以point_center为圆心，point1指向point2的圆弧
 * direction_flag为true则为顺时针，否则为逆时针
 * point_center默认为圆心，暂未处理point_center不为圆心的情况
 */
pub fn draw_arc(
    dest: &mut dyn Write,
    point1: Point,
    point2: Point,
    point_center: Point,
    color: &Color,
    direction_flag: bool,
) {
    if point1.x == point2.x || point1.y != point2.y {
        draw_straight_line(dest, point1, point2, color);
        return;
    }
    let off_x = point_center.x - point1.x;
    let off_z = point_center.z - point1.z;
    let mut flag = 1;
    if direction_flag {
        flag = -1;
    }
    let r = point1.dist_to(&point_center);
    let mut theta = 2.0 * (point1.dist_to(&point2) / (2.0 * r)).asin(); // 所画圆弧的弧长
    if (flag == -1 && point_center.z < (point1.z + point2.z) / 2.0)
        || (flag == 1 && point_center.z > (point1.z + point2.z) / 2.0)
    {
        theta = 2.0 * PI - theta;
    }
    let ve_pcenter_to_p1 = Ve::new(point_center, point1);
    let ve_z = Ve::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );
    let mut rotate_theta = (ve_pcenter_to_p1.dot_product(&ve_z)
        / (ve_pcenter_to_p1.get_module() * ve_z.get_module()))
    .acos();
    if point_center.x < point1.x {
        rotate_theta = 2.0 * PI - rotate_theta;
    }
    rotate_theta *= flag as f64;
    let cpt = 10.max((theta * 180.0) as i64);
    let step = theta / (point2.x - point1.x) / (cpt as f64);
    write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z={}*sin({}*(t-{}))+{},0,{}*cos(t-{})+{};cr,cg,cb=sin(t*0.5)/8+{},sin(t)/8+{},sin(t*1.5)/8+{}\" {} {} 50\r\n",point1.x,point1.x,point1.y,point1.z,theta,r,flag,rotate_theta,off_x,r,rotate_theta,off_z,color.r,color.g,color.b,step,cpt).unwrap();
}
