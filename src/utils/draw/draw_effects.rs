use crate::pre_load::{color::Color, graph::Point};
use std::{f64::consts::PI, io::Write};

pub fn draw_cube_flyup(dest: &mut dyn Write, point: Point, color: &Color) {
    write!(dest,"execute if score @p Timer matches {} run particleex conditional minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0.4 0 0.5 0.5 0.5 \"dis>0.7\" 0.1 60\r\n",point.x,point.x,point.y,point.z,color.r,color.g,color.b,color.a).unwrap();
}

pub fn draw_circle_grow(dest: &mut dyn Write, point: Point, color: &Color) {
    writeln!(dest,"execute if score @p Timer matches {} run particleex rgbaparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z=0.5*sin(t),0,0.5*cos(t);cr,cg,cb=sin(t*3)/4+0.75,sin(t*2)/4+0.75,sin(t)/4+0.75\" 0.05 20 \"(vx,vy,vz)=(x,1,z)/100*10*t*(6.28-t)\" 0.01",point.x,point.x,point.y+1.0,point.z,2.0*PI).unwrap();
}
