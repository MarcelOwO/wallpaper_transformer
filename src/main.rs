use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

mod image_processor;
mod utility;

fn main() {
    println!("Starting wallpaper transformer");

    println!("Reading monitor resolution");

    let (w, h) = utility::get_monitor_size();

    println!("Please enter directory to the wallpapers");
    let input_dir = utility::get_path_from_console();

    println!("Please enter target directory");
    let target_dir = utility::get_path_from_console();

    let files_iter = match fs::read_dir(input_dir) {
        Ok(value) => value,
        Err(_) => panic!("Could not get iter on directory"),
    };

    let files: Vec<PathBuf> = files_iter
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();

    files.par_iter().for_each(|f| {
        image_processor::process_image(f, &target_dir, (w, h));
    });
}
