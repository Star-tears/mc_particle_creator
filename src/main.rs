use office::{DataType, Excel, Range};
use rand::{thread_rng, Rng};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{stdin, stdout, BufWriter, Write},
    vec,
};

#[derive(Debug)]
struct TickNode {
    tick_id: i64,
    pitch_set: BTreeSet<i64>,
}

#[derive(Clone, Debug)]
struct PointGroup {
    x: i64,
    z_list: Vec<i64>,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Copy, Clone)]
struct Edge {
    point1: Point,
    point2: Point,
}

struct Config {
    height: i64,
    tot_tick: i64,
    mid_pitch: i64,
    midi_xlsx_path: String,
    output_setblocks_path: String,
    output_play_path: String,
    output_clear_path: String,
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

fn init_config() -> Config {
    print!("请输入相对y轴高度: ");
    let path = r"H:\mc\mc粒子特效\midi_xlsx\仙剑奇缘.xlsx";
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();
    let y: i64 = s.next().unwrap().parse().unwrap();
    let config = Config {
        height: y,
        tot_tick: 1600,
        mid_pitch: 56,
        midi_xlsx_path: path.to_string(),
        output_setblocks_path: "./functions/setblocks.mcfunction".to_string(),
        output_play_path: "./functions/play.mcfunction".to_string(),
        output_clear_path: "./functions/clear.mcfunction".to_string(),
    };
    config
}
// opens a new workbook
fn main() {
    let config = init_config();

    let mut workbook = Excel::open(config.midi_xlsx_path.as_str()).unwrap();
    // let sheet_list = workbook.sheet_names().unwrap();
    // println!("{:#?}", sheet_list);
    // Read whole worksheet data and provide some statistics
    let range = workbook.worksheet_range("sheet1").unwrap();
    // let mut dest = BufWriter::new(File::create("./1.txt").unwrap());
    // write_range(&mut dest, &range);
    let tick_node_list: Vec<TickNode> = get_tick_node_list(&range);
    // println!("{:#?}", tick_node_list);
    let point_group_list = get_point_group_list(&tick_node_list, &config);
    create_setblocks_mcfunction(&point_group_list, &config);
    // println!("{:#?}", point_group_list[0]);
    let point_list = get_point_list(&tick_node_list, &config);
    // println!("{:?}", point_list);
    create_play_mcfunction(&point_group_list, &config);
    create_clear_mcfunction(&tick_node_list, &config);
}

fn get_tick_node_list(range: &Range) -> Vec<TickNode> {
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

fn get_point_group_list(tick_node_list: &Vec<TickNode>, config: &Config) -> Vec<PointGroup> {
    let mut point_group_list = Vec::new();
    for tick_node in tick_node_list {
        let mut point_group = PointGroup {
            x: tick_node.tick_id,
            z_list: Vec::new(),
        };
        for pitch in tick_node.pitch_set.clone() {
            point_group.z_list.push(pitch);
        }
        point_group_list.push(point_group);
    }
    point_group_list
}

fn get_sub_point_group_list(
    point_group_list: &Vec<PointGroup>,
    config: &Config,
    mut group_count: usize,
    mut sep_step: i64,
) -> Vec<Vec<PointGroup>> {
    if group_count & 1 == 1 {
        group_count += 2;
        group_count -= group_count & 1;
    }
    if sep_step <= 0 {
        sep_step = 5;
    }
    let mut sub_point_group_list: Vec<Vec<PointGroup>> = Vec::new();
    for i in 0..group_count {
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
            let mut now_sep: i64 = config.mid_pitch - sep_step * (group_count as i64 / 2);
            now_sep += sep_step;
            for i in 0..group_count - 1 {
                if *z <= now_sep {
                    sub_point_group_list[i].last_mut().unwrap().z_list.push(*z);
                    break;
                } else if (i == group_count - 2) {
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

fn get_point_list(tick_node_list: &Vec<TickNode>, config: &Config) -> Vec<Point> {
    let mut point_list = Vec::new();
    for tick_node in tick_node_list {
        for pitch in tick_node.pitch_set.clone() {
            let point = Point {
                x: tick_node.tick_id,
                y: config.height,
                z: pitch,
            };
            point_list.push(point);
        }
    }
    point_list
}

/**
 *  边表生成主函数
 */
fn get_edge_list(point_group_list: &Vec<PointGroup>, config: &Config) -> Vec<Edge> {
    let mut rng = thread_rng();
    let mut edge_list = Vec::new();
    let mut point_pre = Point {
        x: -20,
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
        x: config.tot_tick,
        y: config.height,
        z: config.mid_pitch,
    };
    edge_list.push(Edge {
        point1: point_pre,
        point2: point_nxt,
    });
    edge_list
}

fn create_setblocks_mcfunction(point_group_list: &Vec<PointGroup>, config: &Config) {
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
fn create_play_mcfunction(point_group_list: &Vec<PointGroup>, config: &Config) {
    let mut rng = thread_rng();
    let mut dest = BufWriter::new(File::create(config.output_play_path.as_str()).unwrap());
    let color_list: Vec<Color> = vec![
        //白色
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        //七彩色
        Color {
            r: 0.8,
            g: 0.0,
            b: 0.0,
            a: 0.9,
        },
        Color {
            r: 0.8,
            g: 0.544,
            b: 0.0,
            a: 0.9,
        },
        Color {
            r: 0.8,
            g: 0.8,
            b: 0.0,
            a: 0.9,
        },
        Color {
            r: 0.0,
            g: 0.8,
            b: 0.0,
            a: 0.9,
        },
        Color {
            r: 0.0,
            g: 0.8,
            b: 0.8,
            a: 0.9,
        },
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.8,
            a: 0.9,
        },
        Color {
            r: 0.442,
            g: 0.0,
            b: 0.8,
            a: 0.9,
        },
        // 亮rgb
        Color {
            r: 0.95,
            g: 0.45,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.95,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.45,
            b: 0.95,
            a: 0.9,
        },
        //渐变基色
        Color {
            r: 0.65,
            g: 0.55,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.65,
            g: 0.45,
            b: 0.55,
            a: 0.9,
        },
        Color {
            r: 0.55,
            g: 0.65,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.55,
            g: 0.45,
            b: 0.65,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.65,
            b: 0.55,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.55,
            b: 0.65,
            a: 0.9,
        },
    ];
    let sub_point_group_list: Vec<Vec<PointGroup>> =
        get_sub_point_group_list(&point_group_list, config, 4, 10);
    let mut count = 0;
    for sub_point_group in sub_point_group_list {
        count += 1;
        let edge_list = get_edge_list(&sub_point_group, config);
        for edge in edge_list {
            if edge.point1.x == edge.point2.x {
                draw_line(
                    &mut dest,
                    edge.point1,
                    edge.point2,
                    color_list.get(0).unwrap(),
                );
            } else {
                let index_color = rng.gen_range(11..17);
                if count & 1 == 1 {
                    draw_parabola(
                        &mut dest,
                        edge.point1,
                        edge.point2,
                        color_list.get(index_color).unwrap(),
                    );
                } else {
                    draw_line(
                        &mut dest,
                        edge.point1,
                        edge.point2,
                        color_list.get(index_color).unwrap(),
                    );
                }
            }
        }
    }
    create_rhythm_point_mcfunction(&mut dest, config, point_group_list);
    create_tp_mcfuntion(&mut dest, config);
}

fn create_clear_mcfunction(tick_node_list: &Vec<TickNode>, config: &Config) {
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

fn create_tp_mcfuntion(dest: &mut dyn Write, config: &Config) {
    for i in -30..config.tot_tick {
        write!(
            dest,
            "execute if score @p Timer matches {} run tp @p ~{} ~{} ~{}\r\n",
            i,
            i - 20,
            config.height + 10,
            config.mid_pitch
        )
        .unwrap();
    }
}

fn create_rhythm_point_mcfunction(
    dest: &mut dyn Write,
    config: &Config,
    point_group_list: &Vec<PointGroup>,
) {
    let mut rng = thread_rng();
    let color_list: Vec<Color> = vec![
        //白色
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        // 亮rgb
        Color {
            r: 0.95,
            g: 0.45,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.95,
            b: 0.45,
            a: 0.9,
        },
        Color {
            r: 0.45,
            g: 0.45,
            b: 0.95,
            a: 0.9,
        },
    ];
    for point_group in point_group_list {
        for (_, z) in point_group.z_list.iter().enumerate() {
            let index_color = rng.gen_range(0..4);
            let point = Point {
                x: point_group.x,
                y: config.height,
                z: *z,
            };
            draw_cube(dest, point, color_list.get(index_color).unwrap());
        }
    }
}

fn draw_cube(dest: &mut dyn Write, point: Point, color: &Color) {
    write!(dest,"execute if score @p Timer matches {} run particleex conditional minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0.4 0 0.5 0.5 0.5 \"dis>0.7\" 0.1 60\r\n",point.x,point.x,point.y,point.z,color.r,color.g,color.b,color.a).unwrap();
}

fn draw_line(dest: &mut dyn Write, point1: Point, point2: Point, color: &Color) {
    let off_x = point2.x - point1.x;
    let off_y = point2.y - point1.y;
    let off_z = point2.z - point1.z;
    let mut k_x: i64 = 1;
    if off_x == 0 {
        k_x = 0;
    }
    let dist2: i64 = off_x * off_x + off_y * off_y + off_z * off_z;
    let dist: f64 = (dist2 as f64).sqrt();
    let mut k_y: f64 = (off_y as f64) / dist;
    let mut k_z: f64 = (off_z as f64) / dist;
    if off_x != 0 {
        k_y = (off_y as f64) * 1.0 / (off_x as f64);
        k_z = (off_z as f64) * 1.0 / (off_x as f64);
    }
    let mut num_cpt: i64 = 10;
    if off_x != 0 {
        num_cpt = (dist / (off_x as f64)) as i64 * 10;
    }
    let num_sep: f64 = 1.0 / (num_cpt as f64);
    if off_x != 0 {
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb={}*t,{:.5}*t,{:.5}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" {} {} 25\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_x,k_y,k_z,color.r,color.g,color.b,num_sep,num_cpt).unwrap();
    } else {
        write!(dest,"execute if score @p Timer matches {} run particleex parameter minecraft:end_rod ~{} ~{} ~{} {:.3} {:.3} {:.3} {:.3} 0 0 0 0 {:.5} \"x={}*t;y={:.5}*t;z={:.5}*t;\" {} 25\r\n",point1.x,point1.x,point1.y,point1.z,color.r,color.g,color.b,color.a,dist,k_x,k_y,k_z,num_sep).unwrap();
    }
}

fn draw_parabola(dest: &mut dyn Write, point1: Point, point2: Point, color: &Color) {
    let off_x = point2.x - point1.x;
    let off_y = point2.y - point1.y;
    let off_z = point2.z - point1.z;
    let k_y2 = -0.1;
    let k_z: f64 = (off_z as f64) / (off_x as f64);
    if off_x != 0 {
        write!(dest,"execute if score @p Timer matches {} run particleex rgbatickparameter minecraft:end_rod ~{} ~{} ~{} 0 0 0 0 {} \"x,y,z,cr,cg,cb=t,{:.3}*t*(t-{}),{:.3}*t,sin(t/7)/8+{},sin(t/5)/8+{},sin(t/3)/8+{}\" 0.05 20 25\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_y2,off_x,k_z,color.r,color.g,color.b).unwrap();
    }
}

// fn write_range<W: Write>(dest: &mut W, range: &Range) -> std::io::Result<()> {
//     let n = range.get_size().1 - 1;
//     for i in 0..range.get_size().0 {
//         let data = (range.get_value(i, 0), range.get_value(i, 2));
//         println!("{:?}", data);
//     }
//     for r in range.rows() {
//         for (i, c) in r.iter().enumerate() {
//             if i == 0 || i == 2 {
//                 match *c {
//                     DataType::Empty => Ok(()),
//                     DataType::String(ref s) => write!(dest, "{}", s),
//                     DataType::Float(ref f) => write!(dest, "{}", f),
//                     DataType::Int(ref i) => write!(dest, "{}", i),
//                     DataType::Error(ref e) => write!(dest, "{:?}", e),
//                     DataType::Bool(ref b) => write!(dest, "{}", b),
//                 }?;
//                 if i != n {
//                     write!(dest, " ")?;
//                 }
//             }
//         }
//         write!(dest, "\r\n")?;
//     }
//     Ok(())
// }
