mod algorithm;
mod carcassone;
mod evolution;
mod model;

use crate::evolution::{create_empty_board, start_evolution};
use crate::model::{
    bottom_side, left_side, right_side, top_side, Board, Card, CardSide, TerrainType,
};
use cairo::ImageSurface;
use gdk::prelude::GdkContextExt;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
use std::rc::Rc;

const PROGRAM_NAME: &str = "Carcassone Evolved";
const SCALE: f64 = 0.5;
const WINDOW_SIZE: i32 = 645;

fn main() {
    let app = gtk::Application::new(
        Some("com.romanzes.carcassone"),
        ApplicationFlags::HANDLES_OPEN | ApplicationFlags::NON_UNIQUE,
    )
    .unwrap();
    app.connect_startup(build_ui);
    app.connect_activate(|_| ());
    app.run(&std::env::args().collect::<Vec<_>>());
}

fn load_cards() -> Vec<Card> {
    let cards_file = std::fs::File::open(Path::new("./resources/cards.json")).unwrap();
    serde_json::from_reader(cards_file).unwrap()
}

fn build_ui(app: &gtk::Application) {
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    std::thread::spawn(move || {
        // TODO find a way to load cards only once and share them among threads
        let cards = load_cards();
        start_evolution(&cards, &tx);
    });

    let cards = load_cards();
    let visualizer = GtkVisualizer::new(&cards, app);

    rx.attach(None, move |board| {
        visualizer.display_result(board.score, board.board);
        Continue(true)
    });
}

#[derive(Debug)]
pub struct State {
    app: gtk::Application,
    window: gtk::ApplicationWindow,
    canvas_surface: RefCell<CanvasSurface>,
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
        let surface =
            cairo::ImageSurface::create(cairo::Format::ARgb32, WINDOW_SIZE, WINDOW_SIZE).unwrap();
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
                if let Some(cell) = &self.board.cells[x][y] {
                    let image = &self.card_images[&cell.card];
                    context.save();
                    context.translate((x as f64 + 0.5) * 86.0, (y as f64 + 0.5) * 86.0);
                    let rotation = match cell.card_side {
                        CardSide::LEFT => 0.0,
                        CardSide::TOP => PI / 2.0,
                        CardSide::RIGHT => PI,
                        CardSide::BOTTOM => PI * 3.0 / 2.0,
                    };
                    context.rotate(rotation);
                    context.set_source_pixbuf(image, -43.0, -43.0);
                    context.paint();
                    context.fill();
                    context.restore();
                }
            }
        }
    }
}

pub trait Visualizer {
    fn display_result(&self, score: usize, board: Board);
}

struct GtkVisualizer {
    state: Rc<State>,
    drawing_area: gtk::DrawingArea,
}

impl GtkVisualizer {
    fn new(cards: &Vec<Card>, app: &gtk::Application) -> GtkVisualizer {
        let window = gtk::ApplicationWindow::new(app);

        let card_images = cards
            .iter()
            .map(|card| {
                let file_name = format!("./resources/{}", card.pic);
                let image = Pixbuf::new_from_file(file_name).unwrap();
                (card.clone(), image)
            })
            .collect::<HashMap<Card, Pixbuf>>();
        let state: Rc<State> = Rc::new(State {
            app: app.clone(),
            window: window.clone(),
            canvas_surface: RefCell::new(CanvasSurface::new(card_images)),
        });

        state.window.set_title(PROGRAM_NAME);
        state.window.set_default_size(WINDOW_SIZE, WINDOW_SIZE);
        let drawing_area = GtkVisualizer::build_drawing_area(&state);
        state.window.add(&drawing_area);

        state.window.show_all();

        GtkVisualizer {
            state,
            drawing_area,
        }
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

    fn display_board(board: &Board) {
        for y in 0..board.height {
            for _ in 0..board.width {
                print!("┼──────────");
            }
            println!();

            for x in 0..board.width {
                let side = match top_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "          ",
                    TerrainType::ROAD => "    ██    ",
                    TerrainType::TOWN => "██████████",
                };
                print!("│{}", side);
            }
            println!();

            for x in 0..board.width {
                let left_side = match left_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "  ",
                    TerrainType::TOWN => "██",
                };
                let right_side = match right_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "  ",
                    TerrainType::TOWN => "██",
                };
                print!("│{}      {}", left_side, right_side);
            }
            println!();

            for x in 0..board.width {
                let left_side = match left_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "██",
                    TerrainType::TOWN => "██",
                };
                let right_side = match right_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "██",
                    TerrainType::TOWN => "██",
                };
                print!("│{}      {}", left_side, right_side);
            }
            println!();

            for x in 0..board.width {
                let left_side = match left_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "  ",
                    TerrainType::TOWN => "██",
                };
                let right_side = match right_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "  ",
                    TerrainType::ROAD => "  ",
                    TerrainType::TOWN => "██",
                };
                print!("│{}      {}", left_side, right_side);
            }
            println!();

            for x in 0..board.width {
                let side = match bottom_side(&board.cells[x][y]) {
                    TerrainType::FIELD => "          ",
                    TerrainType::ROAD => "    ██    ",
                    TerrainType::TOWN => "██████████",
                };
                print!("│{}", side);
            }
            println!()
        }
    }
}

impl Visualizer for GtkVisualizer {
    fn display_result(&self, score: usize, board: Board) {
        println!("best result: {}", score);
        if score == 0 {
            GtkVisualizer::display_board(&board);
            let serialized = serde_json::to_string(&board).unwrap();
            println!("{:?}", serialized);
        }
        self.state
            .window
            .borrow()
            .set_title(format!("{}: {}", PROGRAM_NAME, score).as_str());
        self.state.canvas_surface.borrow_mut().update(score, board);
        self.drawing_area.queue_draw();
    }
}
