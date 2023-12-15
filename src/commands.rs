

use crate::ui::terminal;
// use image::{Rgb, imageops};
// use crate::ui::window;
use image::{GenericImageView, DynamicImage};
use crossterm::event::{self, KeyCode};

use ratatui::widgets::Widget;
use ratatui::{
    layout::Layout,
    style::Color,
    prelude::*
};
use crate::utils::error;


// struct Cell(Color, Color);
//     //position: (usize, usize)
// // type Cell = (Rgb<u8>, Rgb<u8>) ;
//
// impl Cell {
//     pub fn new(fg:Rgb<u8>, bg:Rgb<u8>) -> Self {
//         Cell(Color::Rgb(fg.0[0],fg.0[1],fg.0[2]), 
//              Color::Rgb(bg.0[0],bg.0[1],bg.0[2]))
//     }
// }
// //
// impl Into<Style> for Cell {
//     fn into(self) -> ratatui::style::Style {
//         return Style::new().fg(self.0).bg(self.1)
//     }
// }

//crop (where the image should show)
//resize (how much space the image should take (terminal size))
//get cells (Styled graphemes)
//
// struct ImageViewer {
//     path: String,
//     image: DynamicImage,
//  
//     crop_region: (u32,u32),
//     view_size: (u32,u32),
//
// }


// pub struct Rect {
//     pub x: u16,
//     pub y: u16,
//     pub width: u16,
//     pub height: u16,
// }

// const MAX_COLS: usize = 1000; 
// const MAX_ROWS: usize = 1000;


// #[derive(Copy, Clone)]
#[derive(Clone, Debug)]
struct ViewableImage {
    path: String,
    // image: DynamicImage, 
    // crop_region: Rect, 
    // view_size: Rect,
    // region: (u16,u16,u16,u16),
    region: Rect,

    // cells: [[Option<Style>; MAX_COLS]; MAX_ROWS],
    cells: Vec<Vec<Style>>, //I only wish this could be stack allocated or something, I know it's
                            //inefficient
} 

use itertools::Itertools;

impl ViewableImage {
    pub fn new(path: &str) -> error::Result<Self>{


        let full_img = image::io::Reader::open(path)?
            .decode()?;

        let (img_width,img_height ) = full_img.dimensions();

        let img = ViewableImage {
            path: path.to_string(),
            region: 
                Rect{
                    x:0,
                    y:0,
                    width:img_width as u16,
                    height:img_height as u16
                },
            cells: ViewableImage::get_cells(&full_img),
        };

        return Ok(img)
    }

}


impl ViewableImage {

    fn get_cells(image: &DynamicImage) -> Vec<Vec<Style>>{
        let rgbimg = image.as_rgb8()
            .expect("DynamicImage should be convertable to rgb");
        let cells: Vec<Vec<Style>> = rgbimg.rows()
            //for top and bottom sections of unicode half block
            .tuples::<(_,_)>()
            .map(|(top, bot)| {
                top.zip(bot).map(move |(t,b)| {
                    Style::new().fg(Color::Rgb(t.0[0], t.0[1], t.0[2]))
                                .bg(Color::Rgb(b.0[0], b.0[1], b.0[2]))
                }).collect::<Vec<Style>>()
            }).collect::<Vec<Vec<Style>>>();
        cells
    }
    // pub fn update_size(&mut self) {
    //
    //
    //
    // }
}


impl Widget for ViewableImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // each of the terminal's cells
        for y in 0..area.height {
            // println!("row complete");
            for x in 0..area.width {

                //calculate the pixel on the full image to refer to 
                //  y ratio is halved because we are using half block pixels
                let y_ratio = (self.region.height as f32/2_f32) / (area.height) as f32 ;
                let x_ratio = (self.region.width  as f32)       / (area.width)  as f32 ;
                let nearest_y = (self.region.y + (y as f32 * y_ratio).floor() as u16) as usize;
                let nearest_x = (self.region.x + (x as f32 * x_ratio).floor() as u16) as usize;
                
                buf.get_mut(x, y)
                    .set_symbol("▀")
                    .set_style(self.cells[nearest_y][nearest_x]);
            }
        }
    }
}

pub fn run() -> error::Result<()> {
    let mut terminal = terminal::start_terminal()?;
    let path = "lowres.jpg";
    let im = ViewableImage::new(path).unwrap();

    // println!("{:?}", im);
    loop {
        let _ = terminal.draw(|frame|{
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(99), //main view
                              Constraint::Min(0),
                ]).split(frame.size());

                frame.render_widget(im.clone(), main_layout[0]);

        });

        //handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(key) => {
                    if let KeyCode::Esc = key.code {
                        // return Ok(());
                        break;
                    }
                },
                _ => {},

            }
        }
    }
    terminal::restore_terminal()?;
    Ok(())
}




