use mc_particle_creator::{
    components::{create_api, get_api},
    pre_load::{configuration::Config, graph::TickNode},
};
use office::Excel;
use std::io::{self, Write};

fn init_config() -> Config {
    print!("请输入相对y轴高度: ");
    let path = r"H:\mc\mc粒子特效\midi_xlsx\仙剑奇缘.xlsx";
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();
    let y: i64 = s.next().unwrap().parse().unwrap();
    let config = Config {
        height: y as f64,
        tot_tick: 1600,
        mid_pitch: 56.0,
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
    let tick_node_list: Vec<TickNode> = get_api::get_tick_node_list(&range);
    // println!("{:#?}", tick_node_list);
    let point_group_list = get_api::get_point_group_list(&tick_node_list, &config);
    create_api::create_setblocks_mcfunction(&point_group_list, &config);
    // println!("{:#?}", point_group_list[0]);
    let point_list = get_api::get_point_list(&tick_node_list, &config);
    // println!("{:?}", point_list);
    create_api::create_play_mcfunction(&point_group_list, &config);
    create_api::create_clear_mcfunction(&tick_node_list, &config);
}
