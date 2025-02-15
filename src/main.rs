use std::env;

fn main() {
    let program_name = env::current_exe()
        .expect("Failed to get current executable path")
        .file_name()
        .expect("Executable path has no filename")
        .to_string_lossy()
        .into_owned();

    println!("{program_name} {}", env!("BUILD_VERSION"));
}
