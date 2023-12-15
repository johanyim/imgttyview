mod commands;
mod ui;
mod utils;
// mod utils::error;

use crate::{utils::error, ui::terminal::start_terminal};
fn main() -> error::Result<()>{
    

    commands::run();


    return Ok(())
}






