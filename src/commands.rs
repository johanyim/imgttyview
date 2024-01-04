

use std::time::Duration;

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



#[derive(Clone, Debug)]
struct ViewableImage {
    ///Path for this image
    path: String,
    ///The region of the image which is displayed to the screen
    region: Rect,
    ///Individual Cells for pixels in the image
    cells: Vec<Vec<Style>>, //I only wish this could be stack allocated or something, I know it's
                            //inefficient
} 

use itertools::Itertools;

impl ViewableImage {
    pub fn new(path: &str) -> error::Result<Self>{
        let full_img = 
            image::io::Reader::open(path)?
            .decode()?;

        let (img_width,img_height) = full_img.dimensions();

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
        let rgbimg = image.to_rgb8();
            // .expect("DynamicImage should be convertable to rgb");
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

pub fn run<B: Backend>(
    mut terminal: Terminal<B>,
    directory: std::fs::ReadDir,
    tick_rate: Duration,
) -> error::Result<()> {

    let path = "midres.jpg";
    let im = ViewableImage::new(path).unwrap();

    loop {
        let _ = terminal.draw(|frame|{
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), //main view
                              Constraint::Min(0),
                ]).split(frame.size());
                frame.render_widget(im.clone(), main_layout[0]);
                //frame.render_widget(ls, main_layout[1]); 
                // TODO

        });
        //handle events
        if event::poll(tick_rate)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        // KeyCode::Left => app.items.unselect(),
                        // KeyCode::Down => app.items.next(),
                        // KeyCode::Up => app.items.previous(), //TODO
                        _ => {},

                    }
                }
            }
        }
    }
}




