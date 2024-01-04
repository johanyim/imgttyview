mod commands;
mod ui;
mod utils;
// mod utils::error;


use std::time::Duration;

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
    // #[command(subcommand)]
    // subcommand: Option<Command>,

    ///Path to open on startup
    #[arg(short, long)]
    path: Option<String>, 
    // relative path vs absolute
    // image vs directory

    // ///Path to configuration file
    // #[arg(short, long)]
    // config: Option<String>,
}

fn main() -> error::Result<()>{
    

    let args = Arguments::parse();

    let directory: String = match args.path {
        Some(path) => path,
        None => String::from(".")
    };
    let mut terminal = start_terminal()?;
    let tick_rate = Duration::from_millis(100);

    //if directory is a file, open the parent directory and display that file. 
    //if directory is a dir,  open that directory and display the first image
    let starting_dir = std::fs::read_dir(directory)?;

    // let _ = commands::run(terminal, starting_dir, tick_rate)?;
    let _ = restore_terminal();
    for dir in starting_dir {
        println!("{:?}", dir?);
    }


    return Ok(())
}


