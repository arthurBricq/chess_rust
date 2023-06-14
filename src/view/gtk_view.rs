use fltk::{*, prelude::*, window::Window};
use fltk::{button::Button, frame::Frame, prelude::*};

use super::super::view::chessview::*;
use super::chessview;
use crate::model::game::{ChessGame, Type, pos_to_index};
use fltk::enums::Color;
use fltk::app::Sender;
use fltk::image::*;

pub struct GTKView {
    chessview: ChessViewModel,
}

const BUTTON_WIDTH: i32 = 80;
const TOP: i32 = 10;
const LEFT: i32 = 10;

impl GTKView {
    pub fn new() -> Self {
        Self {
            chessview: ChessViewModel::new()
        }
    }

    fn draw_button_at(&self, i: i8, j: i8, button: &mut Button) {
        // For wome weird reaons, the buttons don't behve properly without a label.
        button.set_label("");
        button.set_label_size(0);

        if let Some(name) = self.chessview.get_image_name_at(i as i8, j as i8) {
            let path = format!("src/images/{name}");
            let img = SvgImage::load(path).unwrap();
            button.set_image(Some(img));
        } else {
            let path = format!("src/images/transparent_square.svg");
            let img = SvgImage::load(path).unwrap();
            button.set_image(Some(img));
        }

        // let index = pos_to_index(i as i8, j as i8); 
        let index = i + j;
        match (self.chessview.get_square_type(i as i8, j as i8), index % 2) {
            (SquareType::Attacked, _) => button.set_color(Color::from_hex(0xFF9933)),
            (SquareType::LastEngineMove, _) => button.set_color(Color::from_hex(0xf5f58c)),
            (SquareType::Idle, 0) => button.set_color(Color::from_hex(0xeeeed2)),
            (SquareType::Idle, 1) => button.set_color(Color::from_hex(0xbaca44)),
            _ => { println!("Weird for index {index}, i={i}, j={j}") }
        }

        button.set_frame(enums::FrameType::FlatBox);
    }

    fn draw(&self, app: &app::App, s: &Sender<Msg>) -> Vec<Vec<Button>> {
        let mut buttons: Vec<Vec<Button>> = Vec::new();

        let mut win = window::Window::default()
            .with_size(8 * BUTTON_WIDTH + 2 * LEFT, 8 * BUTTON_WIDTH + 2 * TOP)
            .with_label("Chess Engine by Arthur Bricq")
            ;

        win.set_color(Color::White);

        println!("Color of window: {:?}", win.color());

        for i in 0..8 {
            let mut row: Vec<Button> = Vec::new();
            for j in 0..8 {

                // Create a new button 
                let mut button = Button::default()
                    .with_pos(
                        LEFT + BUTTON_WIDTH * i,
                        TOP + BUTTON_WIDTH * j,
                    )
                    .with_size(
                        BUTTON_WIDTH,
                        BUTTON_WIDTH,
                    );
                button.emit(*s, Msg::SquareTapped(pos_to_index(i as i8, 7 - j as i8)));
                self.draw_button_at(i as i8, 7 - j as i8, &mut button);
                row.push(button);
            }
            buttons.push(row);
        }
        win.end();
        win.show();

        return buttons;
    }

    pub fn run_app(&mut self) {
        let app = app::App::default();
        let (s, r) = app::channel();
        let mut buttons = self.draw(&app, &s);

        while app.wait() {
            if let Some(msg) = r.recv() {
                // Call the chessview to run the logic
                self.chessview.message_received(&msg);

                match msg {
                    Msg::SquareTapped(pos) => {
                        println!("Button tapped: {pos}");
                    }
                    _ => {}
                }

                for i in 0..8 {
                    for j in 0..8 {
                        self.draw_button_at(i as i8, 7 - j as i8, &mut buttons[i][j]);
                    }
                }
            }
        }

        // app.run().unwrap();
    }
}