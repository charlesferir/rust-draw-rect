mod canvas;
mod physics;

use canvas::Canvas;
use clap::Parser;
use log::error;
use physics::Simulation;
use simple_logger::SimpleLogger;
use std::{fs::OpenOptions, thread::sleep, time::Duration};

const DEFAULT_WIDTH: u32 = 100;
const DEFAULT_HEIGHT: u32 = 30;
const DEFAULT_RESOLUTION: u32 = 1;
const DEFAULT_CHARS_PER_METER: u32 = 10;

/// Simple program that draws rectangles in a file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// File in which rectangles should be drawn
    #[arg(short, long)]
    out_file: String,

    /// Canvas resolution
    #[arg(short, long, default_value_t = DEFAULT_RESOLUTION)]
    resolution: u32,

    /// Chars per meters
    #[arg(short, long, default_value_t = DEFAULT_CHARS_PER_METER)]
    chars_per_meter: u32,

    /// Canvas width
    #[arg(long, default_value_t = DEFAULT_WIDTH)]
    width: u32,

    /// Canvas height
    #[arg(long, default_value_t = DEFAULT_HEIGHT)]
    height: u32,
}

trait ToVec2Sim {
    fn to_vec2_sim(&self, chars_per_meter: u32) -> physics::Vec2;
}

trait ToVec2Canvas {
    fn to_vec2_canvas(&self, chars_per_meter: u32) -> canvas::Vec2;
}

trait ToRectCanvas {
    fn to_rect_canvas(&self, chars_per_meter: u32) -> canvas::Rectangle;
}

impl ToVec2Sim for canvas::Vec2 {
    fn to_vec2_sim(&self, chars_per_meter: u32) -> physics::Vec2 {
        physics::Vec2 {
            x: self.x as f32 / chars_per_meter as f32,
            y: self.y as f32 / chars_per_meter as f32,
        }
    }
}

impl ToVec2Canvas for physics::Vec2 {
    fn to_vec2_canvas(&self, chars_per_meter: u32) -> canvas::Vec2 {
        canvas::Vec2 {
            x: (self.x * chars_per_meter as f32).round() as u32,
            y: (self.y * chars_per_meter as f32).round() as u32,
        }
    }
}

impl ToRectCanvas for physics::Rectangle {
    fn to_rect_canvas(&self, chars_per_meter: u32) -> canvas::Rectangle {
        canvas::Rectangle {
            pos: self.pos.to_vec2_canvas(chars_per_meter),
            size: self.size.to_vec2_canvas(chars_per_meter),
        }
    }
}

fn sim_to_canvas(sim: &Simulation, canvas: &mut Canvas, chars_per_meter: u32) {
    for rectangle in sim.get_rectangles().iter() {
        canvas
            .draw_rect(rectangle.to_rect_canvas(chars_per_meter))
            .expect("Could not draw rectangle");
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let log_level = match args.debug {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    SimpleLogger::new().with_level(log_level).init().unwrap();

    let file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.out_file.clone())
    {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open file {:?}: {}", args.out_file, e);
            return Err(e);
        }
    };

    let canvas_size = canvas::Vec2 {
        x: args.width,
        y: args.height,
    };
    let sim_size = canvas_size.to_vec2_sim(args.chars_per_meter);
    let mut simulation = Simulation::new(sim_size);
    let mut canvas = Canvas::new(canvas_size, args.resolution, file);

    simulation
        .add_rectangle(physics::Rectangle {
            pos: physics::Vec2 {
                x: sim_size.x / 2.0,
                y: sim_size.y / 2.0,
            },
            size: physics::Vec2 { x: 1.0, y: 1.0 },
            vel: physics::Vec2 { x: 1.0, y: 1.0 },
        })
        .expect("Could not add rectangle");

    // Draw a bouncing rectangle
    loop {
        simulation.update(0.1);
        canvas.clear();
        sim_to_canvas(&simulation, &mut canvas, args.chars_per_meter);
        canvas.render();
        sleep(Duration::from_millis(100));
    }
}
