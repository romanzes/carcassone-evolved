mod carcassone;
mod evolution;
mod cards;
mod model;

use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use gio::ApplicationFlags;
use gdk_pixbuf::Pixbuf;
use cairo::ImageSurface;
use gdk::prelude::GdkContextExt;
use std::f64::consts::PI;
use std::borrow::Borrow;
use crate::cards::CARDS;
use crate::model::{Card, Board};
use crate::evolution::{start_evolution, create_empty_board};

const PROGRAM_NAME: &str = "Carcassone Evolved";
const SCALE: f64 = 0.5;
const WINDOW_SIZE: i32 = 645;

fn main() {
    let app = gtk::Application::new(
        Some("com.romanzes.carcassone"),
        ApplicationFlags::HANDLES_OPEN | ApplicationFlags::NON_UNIQUE,
    ).unwrap();
    app.connect_startup(build_ui);
    app.connect_activate(|_| ());
    app.run(&std::env::args().collect::<Vec<_>>());
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    let card_images = CARDS.iter().map(|card| {
        let file_name = format!("./resources/{}-{}-{}-{}.png", card.sides[0], card.sides[1], card.sides[2], card.sides[3]);
        let image = Pixbuf::new_from_file(file_name).unwrap();
        (card.clone(), image)
    }).collect::<HashMap<Card, Pixbuf>>();
    let state: Rc<State> = Rc::new(State {
        app: app.clone(),
        window: window.clone(),
        canvas_surface: RefCell::new(CanvasSurface::new(card_images)),
    });

    window.set_title(PROGRAM_NAME);
    window.set_default_size(WINDOW_SIZE, WINDOW_SIZE);
    let drawing_area = build_drawing_area(&state);
    window.add(&drawing_area);

    window.show_all();

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    std::thread::spawn(move || {
        start_evolution(&tx);
    });

    rx.attach(None, move |(score, board)| {
        state.window.borrow().set_title(format!("{}: {}", PROGRAM_NAME, score).as_str());
        state.canvas_surface.borrow_mut().update(score, board);
        drawing_area.queue_draw();
        Continue(true)
    });
}

fn build_drawing_area(state: &Rc<State>) -> gtk::DrawingArea {
    let area = gtk::DrawingArea::new();
    area.set_size_request(WINDOW_SIZE, WINDOW_SIZE);
    area.connect_draw({
        let state = state.clone();
        move |_, context| {
            state.canvas_surface.borrow().draw(&context);
            Inhibit(false)
        }
    });

    area.show_all();
    area
}

#[derive(Debug)]
pub struct State {
    app: gtk::Application,
    window: gtk::ApplicationWindow,
    pub canvas_surface: RefCell<CanvasSurface>,
}

#[derive(Debug)]
pub struct CanvasSurface {
    score: usize,
    board: Board,
    card_images: HashMap<Card, Pixbuf>,
    surface: ImageSurface,
}

impl CanvasSurface {
    pub fn new(card_images: HashMap<Card, Pixbuf>) -> CanvasSurface {
        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, WINDOW_SIZE, WINDOW_SIZE).unwrap();
        CanvasSurface {
            score: 0,
            board: create_empty_board(),
            card_images,
            surface,
        }
    }

    pub fn update(&mut self, score: usize, board: Board) {
        self.score = score;
        self.board = board;
    }

    pub fn draw(&self, context: &cairo::Context) {
        context.scale(SCALE, SCALE);
        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint();
        context.fill();
        for x in 0..self.board.width {
            for y in 0..self.board.height {
                if let Some(cell) = self.board.cells[x][y] {
                    let image = &self.card_images[&cell.card];
                    context.save();
                    context.translate((x as f64 + 0.5) * 86.0, (y as f64 + 0.5) * 86.0);
                    context.rotate(cell.card_side as f64 * PI / 2.0);
                    context.set_source_pixbuf(image, -43.0, -43.0);
                    context.paint();
                    context.fill();
                    context.restore();
                }
            }
        }
    }
}
