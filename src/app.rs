use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

use crate::atom::Atom;
use crate::relation::Relation;

pub struct Model {
    _window: window::Id,
    settings: Settings,
    atoms: Vec<Atom>,
    egui: Egui,
}

pub struct Settings {
    pub r_min: f32,
    pub r_max: f32,
    pub friction: f32,
    pub num: usize,
    pub num_t: usize,
    pub rel: Relation,
    pub pn: usize,
    pub pnt: usize,
    pub zoom: f32,
    pub psize: f32,
}

fn init(app: &App, _window: window::Id) -> Model {
    let win = app.window(_window).unwrap();
    let egui = Egui::from_window(&win);

    let (r_min, r_max) = (0.2, 50.0);
    let friction = 0.2;

    const NUM: usize = 1200;
    const NUM_T: usize = 2;
    let mut rel = Relation::new(NUM_T);
    rel.set(0.03, 0_usize, 0_usize);
    rel.set(0.015, 1_usize, 0_usize);
    rel.set(-0.02, 0_usize, 1_usize);
    rel.set(-0.02, 1_usize, 1_usize);

    let settings = Settings {
        r_min,
        r_max,
        friction,
        num: NUM,
        num_t: NUM_T,
        rel,
        pn: NUM,
        pnt: NUM_T,
        zoom: 1.0,
        psize: 5.0,
    };
    let mut atoms: Vec<Atom> = vec![Atom::default(); NUM];
    let (bx, by) = app.window_rect().w_h();
    for (i, atom) in atoms.iter_mut().enumerate() {
        atom.pos.x = random_range(-bx / 2.0, bx / 2.0);
        atom.pos.y = random_range(-by / 2.0, by / 2.0);
        atom.t = i % NUM_T;
    }
    Model {
        _window,
        settings,
        atoms,
        egui,
    }
}

fn restart(app: &App, _window: window::Id, n: usize, n_t: usize) -> Model {
    let m = init(app, _window);
    let num: usize = n;
    let num_t: usize = n_t;

    let rel = Relation::new(num_t);

    let settings = Settings {
        r_min: m.settings.r_min,
        r_max: m.settings.r_max,
        friction: m.settings.friction,
        num,
        num_t,
        rel,
        pn: num,
        pnt: num_t,
        zoom: m.settings.zoom,
        psize: m.settings.psize,
    };
    let mut atoms: Vec<Atom> = vec![Atom::default(); n];
    let (bx, by) = app.window_rect().w_h();
    for (i, atom) in atoms.iter_mut().enumerate() {
        atom.pos.x = random_range(-bx / 2.0, bx / 2.0);
        atom.pos.y = random_range(-by / 2.0, by / 2.0);
        atom.t = i % num_t;
    }
    let e = Egui::from_window(&(app.window(_window).unwrap()));
    Model {
        _window,
        settings,
        atoms,
        egui: e,
    }
}

pub fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("Particle life")
        .size(640, 320)
        .raw_event(raw_events)
        .event(events)
        .view(view)
        .build()
        .unwrap();

    init(app, _window)
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    //-------------------EGUI-------------------
    let egui = &mut model.egui;
    //let set = &mut model.settings;
    egui.set_elapsed_time(update.since_start);

    let c = egui.begin_frame();

    egui::Window::new("Settings for particle life: ").show(&c, |ui| {
        ui.label("Zoom:");
        if ui.button("+").clicked() {
            model.settings.zoom -= 0.1;
        }
        if ui.button("-").clicked() {
            model.settings.zoom += 0.1;
        }
        ui.label("Friction: ");
        ui.add(egui::Slider::new(&mut model.settings.friction, 0.01..=0.75));

        ui.label("Particle size:");
        ui.add(egui::Slider::new(&mut model.settings.psize, 1.0..=10.0));

        ui.label("Minumum distance:");
        ui.add(egui::Slider::new(&mut model.settings.r_min, 0.01..=0.9));

        ui.label("Maximum distance:");
        ui.add(egui::Slider::new(&mut model.settings.r_max, 5.0..=200.0));

        ui.label("Species:");
        ui.add(egui::Slider::new(
            &mut model.settings.pnt,
            1_usize..=5_usize,
        ));

        ui.label("Number of particles:");
        ui.add(egui::Slider::new(
            &mut model.settings.pn,
            500_usize..=5000_usize,
        ));

        //ui.add(
        egui::Grid::new("Atomic relations:").show(ui, |ui| {
            ui.label("");
            for i in 0..model.settings.rel.table.len() {
                ui.label(format!("{}", i));
            }
            ui.end_row();
            for i in 0..model.settings.rel.table.len() {
                ui.label(format!("{}", i));
                for j in 0..model.settings.rel.table[i].len() {
                    ui.add(egui::Slider::new(
                        &mut model.settings.rel.table[i][j],
                        -0.5..=0.5,
                    ));
                }
                ui.end_row();
            }
        })
    });

    if model.settings.pn != model.settings.num || model.settings.pnt != model.settings.num_t {
        let dmod = restart(_app, model._window, model.settings.pn, model.settings.pnt);
        model._window = dmod._window;
        model.atoms = dmod.atoms;

        if dmod.settings.rel.table.len() == model.settings.rel.table.len() {
            let reld = model.settings.rel.table.clone();
            model.settings = dmod.settings;
            model.settings.rel.table = reld;
        } else {
            model.settings = dmod.settings;
        }
    }

    for i in 0..model.atoms.len() {
        let mut f = Vec2::ZERO;
        for j in 0..model.atoms.len() {
            if i == j {
                continue;
            }
            f += model.atoms[i].get_force(&model.atoms[j], &model.settings);
        }

        model.atoms[i].apply_forces(f, &model.settings);
        model.atoms[i].update();
    }
}

fn events(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        // TODO
        KeyPressed(Key::Escape) => { /* quit */ },
        KeyPressed(Key::Space) => { /* pause */ },
        _ => {}
    }
}

fn raw_events(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKSLATEGRAY);
    //draw.ellipse().color(STEELBLUE);
    for i in &model.atoms {
        i.draw(&draw, model.settings.zoom, model.settings.psize);
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
