use std::{
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

pub fn init_dir() {
    fs::remove_dir_all("./functions").unwrap();
    fs::create_dir_all("./functions/play_match_tick").unwrap();
    File::create("./functions/play.mcfunction").unwrap();
    File::create("./functions/clear.mcfunction").unwrap();
    File::create("./functions/setblocks.mcfunction").unwrap();
}

pub fn write_play_tick(content: String, tick: i64) {
    let path = format!("./functions/play_match_tick/{}.mcfunction", tick);
    let mut dest = match OpenOptions::new().append(true).open(&path) {
        Ok(a) => a,
        Err(_) => File::create(path).unwrap(),
    };
    dest.write_all((content + "\n").as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_play_tick() {
        init_dir();
    }
}
