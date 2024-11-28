use super::super::view::chessview::*;
use crate::model::tools::pos_to_index;
use fltk::app::{event_text, App, Sender};
use fltk::enums::{Color, Event};
use fltk::image::SvgImage;
use fltk::{button::Button, prelude::*};
use fltk::{window::Window, *};

pub struct GTKView {
    chess_view: ChessViewModel,
}

impl GTKView {
    pub fn new() -> Self {
        Self {
            chess_view: ChessViewModel::new(),
        }
    }

    fn draw_button_at(&self, i: i8, j: i8, button: &mut Button) {
        // For some weird reasons, the buttons don't behave properly without a label.
        button.set_label("");
        button.set_label_size(0);

        if let Some(name) = self.chess_view.get_image_name_at(i, j) {
            let path = format!("src/images/{name}");
            let img = SvgImage::load(path).unwrap();
            button.set_image(Some(img));
        } else {
            let path = "src/images/transparent_square.svg".to_string();
            let img = SvgImage::load(path).unwrap();
            button.set_image(Some(img));
        }

        let index = i + j;
        match (self.chess_view.get_square_type(i, j), index % 2) {
            (SquareType::Attacked, _) => button.set_color(Color::from_hex(0xFF9933)),
            (SquareType::LastEngineMove, _) => button.set_color(Color::from_hex(0xf5f58c)),
            (SquareType::Idle, 1) => button.set_color(Color::from_hex(0xeeeed2)),
            (SquareType::Idle, 0) => button.set_color(Color::from_hex(0xbaca44)),
            _ => {
                println!("Weird for index {index}, i={i}, j={j}")
            }
        }

        button.set_frame(enums::FrameType::FlatBox);
    }

    /// Creates the FLTK window
    fn draw_window(&self, s: &Sender<Msg>) -> (Window, Vec<Vec<Button>>) {
        const BUTTON_WIDTH: i32 = 50;
        const TOP_MARGIN: i32 = 10;
        const SIDE_MARGIN: i32 = 30;
        const TEXT_SIZE: i32 = 0;

        // Create the window for the application
        let mut app_window = Window::default()
            .with_size(
                8 * BUTTON_WIDTH + 2 * SIDE_MARGIN + TEXT_SIZE,
                8 * BUTTON_WIDTH + 2 * TOP_MARGIN,
            )
            .with_label("Chess Engine by Arthur Bricq");
        app_window.set_color(Color::White);
        app_window.make_resizable(true);

        // If I ever want to improve the UI, here are some links to use...
        //
        // The chess window is the window which contains
        // https://fltk-rs.github.io/fltk-book/Layouts.html
        // https://fltk-rs.github.io/fltk-book/Group-widgets.html
        // https://fltk-rs.github.io/fltk-book/Trees.html
        // let mut chess_window = Window::new(
        //     SIDE_MARGIN,
        //     SIDE_MARGIN,
        //     8 *BUTTON_WIDTH,
        //     8 * BUTTON_WIDTH,
        //     "couch");
        // chess_window.make_resizable(false);

        // Add all the buttons
        let mut buttons: Vec<Vec<Button>> = Vec::new();
        for i in 0..8 {
            let mut row: Vec<Button> = Vec::new();
            for j in 0..8 {
                // Create a new button
                let mut button = Button::default()
                    .with_pos(
                        SIDE_MARGIN + BUTTON_WIDTH * i,
                        TOP_MARGIN + BUTTON_WIDTH * j,
                    )
                    .with_size(BUTTON_WIDTH, BUTTON_WIDTH);
                button.emit(*s, Msg::SquareTapped(pos_to_index(i as i8, 7 - j as i8)));
                self.draw_button_at(i as i8, 7 - j as i8, &mut button);
                row.push(button);
            }

            buttons.push(row);
        }

        app_window.end();
        app_window.show();
        (app_window, buttons)
    }

    pub fn run_app(&mut self) {
        let app = App::default();
        let (s, r) = fltk::app::channel();
        let (mut win, mut buttons) = self.draw_window(&s);

        // Handle when pressing some keys
        win.handle(move |_, event| -> bool {
            match event {
                Event::KeyDown => {
                    if let Some(ch) = event_text().chars().next() {
                        s.send(Msg::KeyPressed(ch));
                    }
                    return true;
                }
                _ => {}
            }
            false
        });

        while app.wait() {
            if let Some(msg) = r.recv() {
                // Call the chessview to run the logic
                self.chess_view.message_received(&msg);

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
