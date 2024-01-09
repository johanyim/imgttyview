mod commands;
mod ui;
mod utils;
mod files;
// mod utils::error;


use std::{
    time::Duration, 
    fs::DirEntry, 
    path::{Path, PathBuf},
};

use crate::{
    utils::error, 
    ui::terminal::{ self, start_terminal, restore_terminal, }
};

use clap::Parser;

#[derive(Parser)]
// #[command(name = "imgttyview")]
// #[command(author = "Johan Y. <johanjyyim@gmail.com>")]
// #[command(version = "1.0")]
// #[command(about = "View/Select images in the terminal", long_about = None)]
#[command(author, version, about, long_about)]
struct Arguments {
    path: Option<String>, 
    // relative path vs absolute
    // image vs directory

    // #[arg(short, long)]
    // config: Option<String>,
}

fn main() -> error::Result<()>{
    let args = Arguments::parse();

    let path = match args.path {
        Some(args_path) => args_path,
        None => "./".to_string(),
    };
    let path: &Path = Path::new(&path);


    let (directory, file) = files::init_open(path)?;



        // if let Some(parent) = path.parent() {
        //     directory = parent;
        //     // file = DirEntry::path(&);
        // } else {
        //     println!("Unable to get parent directory for the file: '{}'", path.display());
        // }
        //
    

    


    if let Ok(entries) = std::fs::read_dir(directory) {
        let f = entries.into_iter()
            .filter_map(Result::ok)
            .filter(|direntry| is_image(&direntry.path()))
            .next();
        println!("first image file = {:?}", f);
    };

    // path.is_dir() display_first_file_in_directory(path);
    // } else {
    //     println!("The provided path '{}' does not exist or is not a valid file or directory.", path.display());
    // }



    // let mut terminal = start_terminal()?;
    // let tick_rate = Duration::from_millis(100);
    let starting_dir = std::fs::read_dir(directory)?;

    // let _ = commands::run(terminal, starting_dir, tick_rate)?;
    // let _ = restore_terminal();
    for dir in starting_dir {
        println!("{:?}", dir? );
    }


    return Ok(())
}



fn is_image(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        match ext.to_string_lossy().to_lowercase().as_str() {
            "avif" | "bmp"  | "farbfeld" | "gif" |
            "ico"  | "jpeg" | "jpg"  | "png"  | "pnm" |
            "qoi"  | "tga"  | "tiff" | "webp" => return true,
            _ => return false,
        };
    };
    return false
}
