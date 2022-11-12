use core::num;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use rand::{self, Rng};

use crate::{
    components::get_api,
    pre_load::{
        color::{self, Color},
        graph::Point,
    },
    utils::{
        draw::{draw_effects, write_mcfunction},
        math::cal,
    },
};
use crate::{
    pre_load::{
        configuration::Config,
        graph::{PointGroup, TickNode},
    },
    utils::draw::draw_lines,
};

pub fn create_setblocks_mcfunction(point_group_list: &Vec<PointGroup>, config: &Config) {
    let mut dest = BufWriter::new(File::create(config.output_setblocks_path.as_str()).unwrap());
    for point_group in point_group_list {
        for (_, z) in point_group.z_list.iter().enumerate() {
            write!(
                dest,
                "setblock ~{} ~{} ~{} minecraft:redstone_lamp[lit=true]\r\n",
                point_group.x, config.height, z
            )
            .unwrap();
        }
    }
    create_tp_mcfuntion(&mut dest, config);
}

/**
 *  play.mcfunction文件生成主函数
 */
pub fn create_play_mcfunction(point_group_list: &Vec<PointGroup>, config: &Config) {
    let mut dest = BufWriter::new(File::create(config.output_play_path.as_str()).unwrap());
    let sub_point_group_list: Vec<Vec<PointGroup>> =
        get_api::get_sub_point_group_list(&point_group_list, config, 4, 10.0);
    let mut count = 0;
    for sub_point_group in sub_point_group_list {
        count += 1;
        if count == 4 || count == 1 {
            write_mcfunction::write_soma_lines_mcfunction(&mut dest, config, &sub_point_group);
        }
        let edge_list = get_api::get_edge_list(&sub_point_group, config);
        if count == 2 || count == 3 {
            for edge in edge_list {
                draw_effects::draw_circle_grow(
                    &mut dest,
                    edge.point2,
                    &Color::get_gradient_base_color(),
                );
                // if edge.point1.x == edge.point2.x {
                //     draw_lines::draw_straight_line(
                //         &mut dest,
                //         edge.point1,
                //         edge.point2,
                //         &Color::get_gradient_base_color(),
                //     );
                // } else {
                // if count == 3 {
                // draw_lines::draw_parabola(
                //     &mut dest,
                //     edge.point1,
                //     edge.point2,
                //     &Color::get_gradient_base_color(),
                // );
                // }
                // else {
                //     draw_lines::draw_straight_line(
                //         &mut dest,
                //         edge.point1,
                //         edge.point2,
                //         &Color::get_gradient_base_color(),
                //     );
                // }
                // }
            }
        }
    }
    // create_rhythm_point_mcfunction(&mut dest, config, point_group_list);
    create_tp_mcfuntion(&mut dest, config);
}

pub fn create_clear_mcfunction(tick_node_list: &Vec<TickNode>, config: &Config) {
    let mut dest = BufWriter::new(File::create(config.output_clear_path.as_str()).unwrap());
    for tick_node in tick_node_list {
        for (_, off_z) in tick_node.pitch_set.iter().enumerate() {
            write!(
                dest,
                "execute if score @p Timer matches {} run setblock ~{} ~{} ~{} minecraft:air\r\n",
                tick_node.tick_id, tick_node.tick_id, config.height, off_z
            )
            .unwrap();
        }
    }
    create_tp_mcfuntion(&mut dest, config);
}

pub fn create_tp_mcfuntion(dest: &mut dyn Write, config: &Config) {
    for i in -30..config.tot_tick {
        write!(
            dest,
            "execute if score @p Timer matches {} run tp @p ~{} ~{} ~{}\r\n",
            i,
            i - 40,
            config.height + 15.0,
            config.mid_pitch
        )
        .unwrap();
    }
}

pub fn create_rhythm_point_mcfunction(
    dest: &mut dyn Write,
    config: &Config,
    point_group_list: &Vec<PointGroup>,
) {
    let mut rng = rand::thread_rng();
    let color_list = color::COLOR_LIST;
    for point_group in point_group_list {
        for (_, z) in point_group.z_list.iter().enumerate() {
            let mut index_color = rng.gen_range(7..11);
            if index_color == 7 {
                index_color = 0;
            }
            let point = Point {
                x: point_group.x,
                y: config.height,
                z: *z,
            };
            draw_effects::draw_cube_flyup(dest, point, color_list.get(index_color).unwrap());
        }
    }
}
