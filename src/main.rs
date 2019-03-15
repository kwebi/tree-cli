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
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let dir_path = dir.path();
        let this_path_str = dir_path.to_str().expect("path convert to str error");
        let this_path = Path::new(this_path_str);

        println!("{}ㅣ", &blank);
        println!("{}{}", &blank, dir.file_name().to_str().unwrap());
        if this_path.is_dir() {
            blank.push('|');
            for _ in 0..5 {
                blank.push(' ');
            }
            dfs(this_path_str, blank)?;
            for _ in 0..5 {
                blank.pop();
            }
            blank.pop();
        }
    }
    Ok(())
}