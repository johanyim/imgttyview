type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::Paragraph,
    layout::{Layout, Constraint, Rect, 
        Direction::{Horizontal, Vertical}
    }, 
    Frame,
};
use std::{rc::Rc, io::Stdout};
use std::io::stdout;
use crate::utils::error;





pub fn start_terminal() -> error::Result<Terminal> {
    // stdout().execute(EnterAlternateScreen)?;
    // enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    return Ok(terminal)
}


pub fn restore_terminal() -> error::Result<()>{
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(())
}



