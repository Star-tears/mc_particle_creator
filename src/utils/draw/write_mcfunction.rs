use std::io::Write;

use rand::Rng;

use crate::{
    pre_load::{
        color::{Color, COLOR_LIST},
        configuration::Config,
        graph::{Point, PointGroup},
    },
    utils::{fsop::write_play_tick, math::cal},
};

use super::draw_lines;

pub fn write_soma_lines_mcfunction(
    dest: &mut dyn Write,
    config: &Config,
    point_group_list: &Vec<PointGroup>,
) {
    let mut rng = rand::thread_rng();
    let mut has_pre_circle_center = false;
    let mut last_direction = false;
    let mut last_circle_center_point: Point = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    // 开始前发散点
    let mut pre_point = Point {
        x: config.first_tick as f64,
        y: config.height,
        z: config.mid_pitch,
    };

    for point_group in point_group_list {
        let num_of_z = point_group.z_list.len();
        let index_nxt = rng.gen_range(0..num_of_z);
        let x = point_group.x;
        for (i, z) in point_group.z_list.iter().enumerate() {
            if i == index_nxt {
                continue;
            }
            let mut tmp_last_direction = last_direction;
            let now_point = Point {
                x,
                y: config.height,
                z: *z,
            };
            draw_lines::draw_parabola(
                dest,
                pre_point,
                now_point,
                &Color::get_gradient_base_color(),
            );
            //是否有上一个圆心
            // if !has_pre_circle_center {
            //     let circle_center_point = cal::gen_rand_circle_center_point(pre_point, now_point);
            //     draw_lines::draw_arc(
            //         dest,
            //         pre_point,
            //         now_point,
            //         circle_center_point,
            //         &Color::get_gradient_base_color(),
            //         rng.gen_bool(0.5),
            //     );
            // } else {
            //     let circle_center_point = match cal::get_dest_cir_center(
            //         pre_point,
            //         now_point,
            //         last_circle_center_point,
            //     ) {
            //         Some(p) => p,
            //         None => {
            //             draw_lines::draw_parabola(
            //                 dest,
            //                 pre_point,
            //                 now_point,
            //                 &Color::get_gradient_base_color(),
            //             );
            //             continue;
            //         }
            //     };
            //     let cos_a: f64 = pre_point.cos_alpha(now_point, last_circle_center_point);
            //     if cos_a < 0.0 {
            //         tmp_last_direction = !tmp_last_direction;
            //     }
            //     draw_lines::draw_arc(
            //         dest,
            //         pre_point,
            //         now_point,
            //         circle_center_point,
            //         &Color::get_gradient_base_color(),
            //         tmp_last_direction,
            //     );
            // }
        }
        let z = point_group.z_list.get(index_nxt).unwrap().clone();
        let now_point = Point {
            x,
            y: config.height,
            z,
        };
        //是否有上一个圆心
        if !has_pre_circle_center {
            let circle_center_point = cal::gen_rand_circle_center_point(pre_point, now_point);
            last_direction = rng.gen_bool(0.5);
            draw_lines::draw_arc(
                dest,
                pre_point,
                now_point,
                circle_center_point,
                &Color::get_gradient_base_color(),
                last_direction,
            );
            has_pre_circle_center = true;
            last_circle_center_point = circle_center_point;
        } else {
            let mut circle_center_point =
                match cal::get_dest_cir_center(pre_point, now_point, last_circle_center_point) {
                    Some(p) => p,
                    None => {
                        draw_lines::draw_parabola(
                            dest,
                            pre_point,
                            now_point,
                            &Color::get_gradient_base_color(),
                        );
                        has_pre_circle_center = false;
                        continue;
                    }
                };
            if circle_center_point.dist_to(&pre_point) > 20.0 {
                circle_center_point = cal::gen_rand_circle_center_point(pre_point, now_point);
            }
            let cos_a: f64 = pre_point.cos_alpha(now_point, last_circle_center_point);
            if cos_a < 0.0 {
                last_direction = !last_direction;
            }
            draw_lines::draw_arc(
                dest,
                pre_point,
                now_point,
                circle_center_point,
                &Color::get_gradient_base_color(),
                last_direction,
            );
            last_circle_center_point = circle_center_point;
        }
        pre_point = now_point;
    }
    // 增加结束后收敛点
    let now_point = Point {
        x: config.tot_tick as f64,
        y: config.height,
        z: config.mid_pitch,
    };
    //是否有上一个圆心
    if !has_pre_circle_center {
        let circle_center_point = cal::gen_rand_circle_center_point(pre_point, now_point);
        last_direction = rng.gen_bool(0.5);
        draw_lines::draw_arc(
            dest,
            pre_point,
            now_point,
            circle_center_point,
            &Color::get_gradient_base_color(),
            last_direction,
        );
    } else {
        let circle_center_point =
            match cal::get_dest_cir_center(pre_point, now_point, last_circle_center_point) {
                Some(p) => p,
                None => {
                    draw_lines::draw_parabola(
                        dest,
                        pre_point,
                        now_point,
                        &Color::get_gradient_base_color(),
                    );
                    return;
                }
            };
        let cos_a: f64 = pre_point.cos_alpha(now_point, last_circle_center_point);
        if cos_a < 0.0 {
            last_direction = !last_direction;
        }
        draw_lines::draw_arc(
            dest,
            pre_point,
            now_point,
            circle_center_point,
            &Color::get_gradient_base_color(),
            last_direction,
        );
    }
}

pub fn write_tp_mcfuntion_in_play(config: &Config) {
    for i in -30..config.tot_tick {
        write_play_tick(
            format!(
                "execute if score @p Timer matches {} run tp @p ~{} ~{} ~{}\r\n",
                i,
                i - 40,
                config.height + 30.0,
                config.mid_pitch
            ),
            i,
        );
    }
}
