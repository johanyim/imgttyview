

use crate::ui::terminal;
// use crate::ui::window;
use image::{GenericImageView, Pixel, DynamicImage, imageops::FilterType};

use image::buffer::EnumeratePixels;
use ratatui::text::StyledGrapheme;
use ratatui::widgets::Widget;
use ratatui::{
    layout::Layout,
    style::Color,
    prelude::*
};
use crate::utils::error;


struct Cell {
    //position: (usize, usize)
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

impl Cell {
    pub fn new(fg:(u8,u8,u8), bg:(u8,u8,u8)) -> Self {
        Cell {fg, bg}
    }
}
//
// impl Into<(Color, Color)> for Cell {
//     fn into(self){
//         return Color::Rgb(self.fg.0, self.fg.1, self.fg.2), Color::Rgb(self.bg.0, self.bg.1, self.bg.2)
//     }
// }


const MAX_ROWS: usize = 50;
const MAX_COLS: usize = 50;

struct ViewableImage {
    path: String,
    image: DynamicImage, 
    size: (usize,usize),
    // cells: [[Option<Span<'a>>; MAX_COLS]; MAX_ROWS],
    cells: Vec<Style>,
} 


impl ViewableImage {
    pub fn new(path: &str) -> error::Result<Self>{
        let img = image::io::Reader::open("lowres.jpg")?
            .decode()?;
        // //WTF? initializing an array with a function??
        // let mut cells: [[Option<Span<'a>>;MAX_COLS];MAX_ROWS] = 
        //     std::array::from_fn(
        //         |_| std::array::from_fn(
        //             |_| None));
        // for i in 0..MAX_COLS {
        //     for j in 0..MAX_ROWS {
        //         cells[i][j] = Some(Span::styled("#", style::Style::new()));
        //     }
        // }

        let mut cells: Vec<Style> = vec![];

        for (x, y, pix) in img.pixels() {
            cells.push(
                Style::new().fg(Color::Rgb(pix.0[0], pix.0[1], pix.0[2]))
                .bg(Color::Rgb(pix.0[0], pix.0[1], pix.0[2]))
                )
        }

        let img = ViewableImage {
            path: "hello".to_string(),
            image: img,
            size: (40, 40),
            cells,
        };

        return Ok(img)
    }
}

impl Widget for ViewableImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for row in 0..self.size.1 {
            for col in 0..self.size.0 {
                buf.get_mut(col as u16, row as u16)
                    .set_symbol("▀")
                    .set_style(self.cells[row]);
            }
        }
    }
}

pub fn run() -> error::Result<()> {
    let mut terminal = terminal::start_terminal()?;



    loop {
        let im = ViewableImage::new("test.jpeg").unwrap();
        let _ = terminal.draw(|frame|{
            //make a layout
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(75), //main view
                              Constraint::Min(0),
                ]).split(frame.size());

                frame.render_widget(im, main_layout[0])

        });
    }
    terminal::restore_terminal()?;
    Ok(())
}




