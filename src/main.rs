mod carcassone;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use std::cell::{Cell as StdCell, RefCell};
use std::env::args;
use std::rc::Rc;
use std::thread;
use crate::carcassone::start_evolution;

const PROGRAM_NAME: &str = "Carcassone Evolved";

fn main() {
    glib::set_program_name(Some(PROGRAM_NAME));

    let application = gtk::Application::new(
        Some("com.romanzes.carcassone"),
        gio::ApplicationFlags::empty(),
    )
        .expect("initialization failed");

    application.connect_startup(|app| {
        let application = Application::new(app);

        let application_container = RefCell::new(Some(application));
        app.connect_shutdown(move |_| {
            let application = application_container
                .borrow_mut()
                .take()
                .expect("Shutdown called multiple times");
            drop(application);
        });
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}

pub struct Application {
    pub widgets: Rc<Widgets>,
}

impl Application {
    pub fn new(app: &gtk::Application) -> Self {
        let app = Application {
            widgets: Rc::new(Widgets::new(app)),
        };

        app.connect_progress();

        app
    }

    fn connect_progress(&self) {
        let active = Rc::new(StdCell::new(false));
        let on_click = move |widgets: Rc<Widgets>| {
            if active.get() {
                return;
            }

            active.set(true);

            let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            thread::spawn(move || {
                start_evolution(tx);
            });

            let active = active.clone();
            let widgets = widgets.clone();
            rx.attach(None, move |value| match value {
                Some(value) => {
                    widgets
                        .progress
                        .set_text(format!("{}", value).as_str());

                    glib::Continue(true)
                }
                None => {
                    active.set(false);
                    glib::Continue(false)
                }
            });
        };
        self.widgets.button.connect_clicked(
            clone!(@weak self.widgets as widgets => move |_| on_click(widgets)),
        );
    }
}

pub struct Widgets {
    pub container: gtk::Grid,
    pub progress: gtk::Label,
    pub button: gtk::Button,
}

impl Widgets {
    pub fn new(application: &gtk::Application) -> Self {
        let progress = gtk::Label::new(Some("Not started"));
        progress.set_hexpand(true);

        let button = gtk::Button::new();
        button.set_label("start");
        button.set_halign(gtk::Align::Center);

        let container = gtk::Grid::new();
        container.attach(&progress, 0, 0, 1, 1);
        container.attach(&button, 0, 1, 1, 1);
        container.set_row_spacing(12);
        container.set_border_width(6);
        container.set_vexpand(true);
        container.set_hexpand(true);

        let window = gtk::ApplicationWindow::new(application);
        window.set_property_window_position(gtk::WindowPosition::Center);
        window.set_title(PROGRAM_NAME);
        window.add(&container);
        window.show_all();
        window.connect_delete_event(move |window, _| {
            window.destroy();
            Inhibit(false)
        });

        Widgets {
            container,
            progress,
            button
        }
    }
}