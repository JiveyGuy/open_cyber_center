#[macro_use]
extern crate fstrings;

use std::process::Command;
use std::fs;

// Input: gid, out path including fiel name
fn update_local_resources(g_id: &str, out_str: &str) {
    let output = Command::new("gdown")
        .arg(g_id)
        .arg("-O")
        .arg(out_str)
        .output()
        .expect("Failed to run command.");
    println!("{}", output.status);  
}


// call with where the file is and the path to it's resting dir
// example: mv_and_unzip("tmp.zip", "tmp")
fn mv_and_unzip(loc_start: &str, loc_end: &str) {
    
    fs::create_dir("loc_end")?;
    
    let _output = Command::new("mv")
        .arg(loc_start)
        .arg(loc_end)
        .output()
        .expect("Failed to run command.");

    let _output = Command::new("tar")
        .arg("-xf")
        .arg(f!("{loc_end}/tmp.zip"))
        .output()
        .expect("Failed to run command.");
}

fn main() {
    update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
    println!("Download, done!");
    mv_and_unzip("tmp.zip", "tmp");
}
