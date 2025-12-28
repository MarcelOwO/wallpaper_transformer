use std::io;
use std::path::{Path, PathBuf};

pub(crate) fn get_path_from_console() -> PathBuf {
    let mut input = String::new();
    let mut path = Path::new("");

    loop {
        input.clear();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read line");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Input can't be empty");
            continue;
        }

        path = Path::new(trimmed);

        if !path.is_dir() {
            println!("input is not a valid path");
            continue;
        }

        break;
    }

    PathBuf::from(path)
}

pub(crate) fn get_monitor_size() -> (u32, u32) {
    let mut width = String::new();

    let w: u32;

    let mut height = String::new();

    let h: u32;
    println!("Enter monitor width");

    loop {
        width.clear();

        if let Err(_) = io::stdin().read_line(&mut width) {
            println!("Failed to read line");
            continue;
        }

        let trimmed = width.trim();

        if trimmed.is_empty() {
            println!("Width can't be empty");
            continue;
        }

        w = match trimmed.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };

        break;
    }
    println!("Enter monitor height");

    loop {
        height.clear();

        if let Err(_) = io::stdin().read_line(&mut height) {
            println!("Failed to read line");
            continue;
        }

        let trimmed = height.trim();

        if trimmed.is_empty() {
            println!("Height can't be empty");
            continue;
        }

        h = match trimmed.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };

        break;
    }

    (w, h)
}
