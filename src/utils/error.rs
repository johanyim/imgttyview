use core::result;
use std::io::Stdout;
use ratatui::prelude::CrosstermBackend;
use thiserror::Error;

// mod ui;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File could not be found at {0}.")]
    FileNotFound(String),


    #[error("Could not start terminal")]
    IOError(#[from] std::io::Error),
    // io::Result<Terminal<CrosstermBackend<Stdout>>> 
    
    #[error("Could not decode this image")]
    ImageDecodeError(#[from] image::ImageError),


    // #[error("Could not decode this image")]
    // ImageReadingError(#[from] image::ImageError),
    

}




