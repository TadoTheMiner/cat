use std::{env::args, fs, path::Path, process::Command};

fn main() {
    let args = args().collect::<Vec<String>>();
    let path = Path::new(args.get(1).unwrap());
    let file = fs::read(path).unwrap();
    let formatted_data = file
        .iter()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let result_file = format!(include_str!("../template"), formatted_data);
    let path_stem = path.file_stem().unwrap().to_str().unwrap();
    fs::write(format!("{}.s", path_stem), result_file).unwrap();
    Command::new("nasm")
        .args([
            "-f",
            "elf64",
            "-o",
            format!("{}.o", path_stem).as_str(),
            format!("{}.s", path_stem).as_str(),
        ])
        .output()
        .expect("failed to execute nasm");
    Command::new("ld")
        .args(["-o", path_stem, format!("{}.o", path_stem).as_str()])
        .output()
        .expect("failed to execute ld");
}
