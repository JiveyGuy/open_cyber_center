use std::process::Command;
struct Program{
    location: String, // where the .exe is 
}

async fn launch_program(prog: String) {
    let _output = Command::new(prog)
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    let gmod_program = "G:\\SteamLibrary\\steamapps\\common\\Among Us\\Among Us.exe".to_string();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(launch_program(gmod_program));

    println!("done.");
}
