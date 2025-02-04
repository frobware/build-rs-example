use std::env;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn main() {
    let program_name = env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "unknown".to_string());

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--version") => println!("{BUILD_VERSION}"),
        Some("--build-date") => println!("{BUILD_DATE}"),
        Some("--build-toolchain") => println!("{BUILD_TOOLCHAIN_VERSION}"),
        _ => {
            println!("{program_name} {BUILD_VERSION} ({BUILD_DATE}) [{BUILD_TOOLCHAIN_VERSION}]");
        }
    }
}
