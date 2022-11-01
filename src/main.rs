use office::{DataType, Excel, Range};
use std::{
    collections::BTreeSet,
    fs::File,
    io::{stdin, stdout, BufWriter, Write},
    ops::IndexMut,
    path::PathBuf,
    vec,
};

#[derive(Debug)]
struct TickNode {
    tick_id: i64,
    pitch_set: BTreeSet<i64>,
}

#[derive(Debug)]
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
}

fn init_config() -> Config {
    print!("请输入相对y轴高度: ");
    let path = r"H:\mc\mc粒子特效\midi_xlsx\最伟大的作品.xlsx";
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();
    let y: i64 = s.next().unwrap().parse().unwrap();
    let config = Config {
        height: y,
        tot_tick: 5000,
        mid_pitch: 60,
        midi_xlsx_path: r"H:\mc\mc粒子特效\midi_xlsx\最伟大的作品.xlsx".to_string(),
        output_setblocks_path: "./functions/setblocks.mcfunction".to_string(),
        output_play_path: "./functions/play.mcfunction".to_string(),
    };
    config
}
// opens a new workbook
fn main() {
    let config = init_config();

    let mut workbook = Excel::open(config.midi_xlsx_path.as_str()).unwrap();
    let sheet_list = workbook.sheet_names().unwrap();
    println!("{}", sheet_list[0]);
    // Read whole worksheet data and provide some statistics
    let range = workbook.worksheet_range(sheet_list[0].as_str()).unwrap();
    // let mut dest = BufWriter::new(File::create("./1.txt").unwrap());
    // write_range(&mut dest, &range);
    let tick_node_list: Vec<TickNode> = get_tick_node_list(&range);
    // println!("{:#?}", tick_node_list);
    create_setblocks_mcfunction(&tick_node_list, &config);
    let point_group_list = get_point_group_list(&tick_node_list, &config);
    // println!("{:#?}", point_group_list);
    let point_list = get_point_list(&tick_node_list, &config);
    // println!("{:?}", point_list);
    let edge_list = get_edge_list(&point_list, &config);
    create_play_mcfunction(edge_list, &config);
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

fn get_edge_list(point_list: &Vec<Point>, config: &Config) -> Vec<Edge> {
    let mut edge_list = Vec::new();
    let mut point1 = Point {
        x: -20,
        y: config.height,
        z: config.mid_pitch,
    };
    for point in point_list {
        edge_list.push(Edge {
            point1: point1,
            point2: *point,
        });
        point1 = *point;
    }
    edge_list
}

fn create_setblocks_mcfunction(tick_node_list: &Vec<TickNode>, config: &Config) {
    let mut dest = BufWriter::new(File::create(config.output_setblocks_path.as_str()).unwrap());
    for tick_node in tick_node_list {
        for (_, off_z) in tick_node.pitch_set.iter().enumerate() {
            write!(
                dest,
                "setblock ~{} ~{} ~{} minecraft:redstone_lamp[lit=true]\r\n",
                tick_node.tick_id, config.height, off_z
            )
            .unwrap();
            // println!(
            //     "setblock ~{} ~{} ~{} minecraft:redstone_lamp",
            //     tick_node.tick_id, y, off_z
            // );
        }
    }
    create_tp_mcfuntion(&mut dest, config);
}

fn create_play_mcfunction(edge_list: Vec<Edge>, config: &Config) {
    let mut dest = BufWriter::new(File::create(config.output_play_path.as_str()).unwrap());
    for edge in edge_list {
        draw_line(&mut dest, edge.point1, edge.point2);
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

fn draw_line(dest: &mut dyn Write, point1: Point, point2: Point) {
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
        write!(dest,"execute if score @p Timer matches {} run particleex tickparameter minecraft:end_rod ~{} ~{} ~{} 0.8 0.8 0 1 0 0 0 0 {} \"x={}*t;y={:.5}*t;z={:.5}*t;\" {:.5} {} 25\r\n",point1.x,point1.x,point1.y,point1.z,off_x,k_x,k_y,k_z,num_sep,num_cpt).unwrap();
    } else {
        write!(dest,"execute if score @p Timer matches {} run particleex parameter minecraft:end_rod ~{} ~{} ~{} 0.8 0.8 0 1 0 0 0 0 {:.5} \"x={}*t;y={:.5}*t;z={:.5}*t;\" {:.5} {} 25\r\n",point1.x,point1.x,point1.y,point1.z,dist,k_x,k_y,k_z,num_sep,num_cpt).unwrap();
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
