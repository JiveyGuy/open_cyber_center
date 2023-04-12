#[macro_use]
extern crate fstrings;

use std::process::Command;

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

fn main() {
    update_local_resources("1DrhBGl67bjmZA9MiZA2Y11L6Ts_1FN1J", "tmp.zip");
    println!("Download, done!");
}
