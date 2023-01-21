use crate::{
    pre_load::{color::Color, graph::Point},
    utils::fsop::write_play_tick,
};
use std::{
    f64::consts::{PI, TAU},
    fmt::format,
    io::Write,
};

pub fn draw_cube_flyup(point: Point, color: &Color) {
    write_play_tick(format!("execute if score @p Timer matches {} run particleex conditional minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0.4 0 0.5 0.5 0.5 \"abs(x)==0.5&abs(z)==0.5 | abs(y)==0.5&abs(z)==0.5 | abs(x)==0.5&abs(y)==0.5\" 0.1 60",point.x,point.x,point.y,point.z,color.r,color.g,color.b,color.a), point.x as i64);
}

pub fn draw_cube_rotate_flyup(point: Point) {
    write_play_tick(format!("execute if score @p Timer matches {} run particleex conditional minecraft:end_rod ~{} ~{} ~{} 1 1 1 1 0 0 0 0.5 0.5 0.5 \"abs(x)==0.5&abs(z)==0.5 | abs(y)==0.5&abs(z)==0.5 | abs(x)==0.5&abs(y)==0.5\" 0.1 60 \"a=0.05;(vx,,vy,,vz)=(-sin(a),0,-cos(a),,0,1,0,,cos(a),0,-sin(a))*(x*2*sin(a),,0.1,,z*2*sin(a)) 1.0\"",point.x as i64,point.x,point.y,point.z), point.x as i64);
}

pub fn draw_circle_grow(dest: &mut dyn Write, point: Point, color: &Color) {
    writeln!(dest,"execute if score @p Timer matches {} run particleex rgbaparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z=0.5*sin(t),0,0.5*cos(t);cr,cg,cb=sin(t*3)/4+0.75,sin(t*2)/4+0.75,sin(t)/4+0.75\" 0.05 20 \"(vx,vy,vz)=(x,1,z)/100*10*t*(6.28-t)\" 0.01",point.x,point.x,point.y+1.0,point.z,2.0*PI).unwrap();
}

pub fn draw_circle_grow2(point: Point, color: &Color) {
    let mut k = 0.0;
    for _ in 0..60 {
        write_play_tick(format!("execute if score @p Timer matches {} run particleex tickpolarparameter minecraft:end_rod ~{} ~{} ~{} 1 1 1 1 0 0 0 0 3 \"s1,dis={},t\" 0.1 1 8",point.x,point.x,point.y,point.z,PI*k), point.x as i64);
        k += TAU / 60.0;
    }
}

pub fn draw_column(point: Point) {
    write_play_tick(format!("execute if score @p Timer matches {} run particle minecraft:end_rod ~{} ~{} ~{} 0 100 0 0.05 2000 force",point.x as i64,point.x,point.y,point.z), point.x as i64);
}
