use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <搜索词> <文件路径>", args[0]);
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("读取文件失败: {}", err);
        process::exit(1);
    });

    for (i, line) in content.lines().enumerate() {
        if line.contains(query) {
            println!("{}: {}", i + 1, line);
        }
    }
}
