use std::collections::BTreeSet;

use office::{DataType, Range};
use rand::Rng;

use crate::pre_load::{
    configuration::Config,
    graph::{Edge, Point, PointGroup, TickNode},
};

pub fn get_tick_node_list(range: &Range) -> Vec<TickNode> {
    let mut tick_node_list: Vec<TickNode> = Vec::new();
    for i in 1..range.get_size().0 {
        let mut is_vaild: bool = true;
        let tick_id: i64 = match *range.get_value(i, 0) {
            DataType::Int(a) => a as i64,
            DataType::Float(b) => b as i64,
            DataType::String(_) => todo!(),
            DataType::Bool(_) => todo!(),
            DataType::Error(_) => todo!(),
            DataType::Empty => {
                is_vaild = false;
                0
            }
        };
        let pitch: i64 = match *range.get_value(i, 2) {
            DataType::Int(a) => a as i64,
            DataType::Float(b) => b as i64,
            DataType::String(_) => todo!(),
            DataType::Bool(_) => todo!(),
            DataType::Error(_) => todo!(),
            DataType::Empty => {
                is_vaild = false;
                0
            }
        };
        if !is_vaild {
            println!("第{}行不合法", i);
            continue;
        }
        if tick_node_list.len() == 0 || tick_node_list.last().unwrap().tick_id != tick_id {
            let mut pitch_set = BTreeSet::new();
            pitch_set.insert(pitch);
            let tick_node: TickNode = TickNode {
                tick_id: tick_id,
                pitch_set: pitch_set,
            };
            tick_node_list.push(tick_node);
        } else {
            tick_node_list.last_mut().unwrap().pitch_set.insert(pitch);
        }
    }
    tick_node_list
}

pub fn get_point_group_list(tick_node_list: &Vec<TickNode>, config: &Config) -> Vec<PointGroup> {
    let mut point_group_list = Vec::new();
    for tick_node in tick_node_list {
        let mut point_group = PointGroup {
            x: tick_node.tick_id as f64,
            z_list: Vec::new(),
        };
        for pitch in tick_node.pitch_set.clone() {
            point_group.z_list.push(pitch as f64);
        }
        point_group_list.push(point_group);
    }
    point_group_list
}

pub fn get_sub_point_group_list(
    point_group_list: &Vec<PointGroup>,
    config: &Config,
    mut group_count: usize,
    mut sep_step: f64,
) -> Vec<Vec<PointGroup>> {
    if group_count & 1 == 1 {
        group_count += 2;
        group_count -= group_count & 1;
    }
    if sep_step <= 0.0 {
        sep_step = 5.0;
    }
    let mut sub_point_group_list: Vec<Vec<PointGroup>> = Vec::new();
    for _ in 0..group_count {
        sub_point_group_list.push(Vec::new());
    }

    for point_group in point_group_list {
        let init_point_group = PointGroup {
            x: point_group.x,
            z_list: Vec::new(),
        };
        for i in 0..group_count {
            sub_point_group_list[i].push(init_point_group.clone());
        }
        for (_, z) in point_group.z_list.iter().enumerate() {
            let mut now_sep = config.mid_pitch - sep_step * (group_count as f64 / 2.0);
            now_sep += sep_step;
            for i in 0..group_count - 1 {
                if *z <= now_sep {
                    sub_point_group_list[i].last_mut().unwrap().z_list.push(*z);
                    break;
                } else if i == group_count - 2 {
                    sub_point_group_list[i + 1]
                        .last_mut()
                        .unwrap()
                        .z_list
                        .push(*z);
                    break;
                }
                now_sep += sep_step;
            }
        }
        for i in 0..group_count {
            if sub_point_group_list[i].last().unwrap().z_list.len() == 0 {
                sub_point_group_list[i].pop();
            }
        }
    }
    sub_point_group_list
}

pub fn get_point_list(tick_node_list: &Vec<TickNode>, config: &Config) -> Vec<Point> {
    let mut point_list = Vec::new();
    for tick_node in tick_node_list {
        for pitch in tick_node.pitch_set.clone() {
            let point = Point {
                x: tick_node.tick_id as f64,
                y: config.height,
                z: pitch as f64,
            };
            point_list.push(point);
        }
    }
    point_list
}

/**
 *  边表生成主函数
 */
pub fn get_edge_list(point_group_list: &Vec<PointGroup>, config: &Config) -> Vec<Edge> {
    let mut rng = rand::thread_rng();
    let mut edge_list = Vec::new();
    let mut point_pre = Point {
        x: -20.0,
        y: config.height,
        z: config.mid_pitch,
    };
    let mut point_nxt: Point;
    for point_group in point_group_list {
        if point_group.z_list.len() > 1 {
            for i in 0..point_group.z_list.len() - 1 {
                let point1 = Point {
                    x: point_group.x,
                    y: config.height,
                    z: point_group.z_list[i],
                };
                let point2 = Point {
                    x: point_group.x,
                    y: config.height,
                    z: point_group.z_list[i + 1],
                };
                edge_list.push(Edge {
                    point1: point1,
                    point2: point2,
                });
            }
        }
        let index_nxt = rng.gen_range(0..point_group.z_list.len());
        point_nxt = Point {
            x: point_group.x,
            y: config.height,
            z: point_group.z_list[index_nxt],
        };
        edge_list.push(Edge {
            point1: point_pre,
            point2: point_nxt,
        });
        point_pre = point_nxt;
    }
    point_nxt = Point {
        x: config.tot_tick as f64,
        y: config.height,
        z: config.mid_pitch,
    };
    edge_list.push(Edge {
        point1: point_pre,
        point2: point_nxt,
    });
    edge_list
}
