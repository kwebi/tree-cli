use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        dfs(".", &mut "".to_string())?
    } else {
        dfs(&args[1], &mut "".to_string())?
    }
    Ok(())
}

fn dfs(path: &str, blank: &mut String) -> io::Result<()> {
    let mut i: usize = 0;
    let len = fs::read_dir(path)?.count();
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let dir_path = dir.path();
        let this_path_str = dir_path.to_str().expect("path convert to str error");
        let this_path = Path::new(this_path_str);
        //判断是否为最后一个
        if i + 1 == len {
            blank.push_str("└── ");
        } else {
            blank.push_str("├── ");
        }
        println!("{}{}", &blank, dir.file_name().to_str().unwrap());
        for _ in 0..4 {
            blank.pop();
        }
        if this_path.is_dir() {
            if i + 1 != len {
                blank.push('│');
            } else {
                blank.push(' ');
            }
            blank.push_str("   ");
            dfs(this_path_str, blank)?;
            for _ in 0..4 {
                blank.pop();
            }
        }
        i += 1;
    }
    Ok(())
}