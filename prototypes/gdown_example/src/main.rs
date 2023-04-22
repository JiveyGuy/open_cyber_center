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
    
    let resting_path: &str = &f!("{loc_end}/tmp.zip");
    println!("{}",f!("resting_path = ./{resting_path}"));
    
    if std::path::Path::new(resting_path).exists() {
        fs::remove_file(resting_path)
            .expect("Failed to remove goal file.");
    }   
   
    fs::rename(loc_start, resting_path)
        .expect("Failed to move file.");

    let _output = Command::new("tar")
        .arg("-xf")
        .arg(resting_path)
        .arg("-C")
        .arg(loc_end)
        .output()
        .expect("Failed to run command.");
}

fn download_all(){
    update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
    println!("Download, done!");
    mv_and_unzip("tmp.zip", "./tmp");
}

fn main() {
    
}
