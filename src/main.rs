mod commands;
mod ui;
mod utils;
// mod utils::error;

use video::{utils::error, ui::terminal::start_terminal};
fn main() -> error::Result<()>{
    

    commands::run();


    return Ok(())
}






