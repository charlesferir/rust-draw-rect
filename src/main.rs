mod canvas;

use canvas::{Canvas, Rectangle, Vec2};
use clap::Parser;
use log::error;
use simple_logger::SimpleLogger;
use std::{fs::OpenOptions, thread::sleep, time::Duration};

const DEFAULT_WIDTH: u32 = 160;
const DEFAULT_HEIGHT: u32 = 144;
const DEFAULT_RESOLUTION: u32 = 1;

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

    /// Canvas width
    #[arg(long, default_value_t = DEFAULT_WIDTH)]
    width: u32,

    /// Canvas height
    #[arg(long, default_value_t = DEFAULT_HEIGHT)]
    height: u32,
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

    let mut canvas = Canvas::new(args.width, args.height, args.resolution, file);

    // Draw a droping rectangle
    for i in 0..args.height - 9 {
        canvas.clear();
        canvas
            .draw_rect(Rectangle {
                pos: Vec2 { x: 10, y: i },
                size: Vec2 { x: 10, y: 10 },
            })
            .expect("Could not draw rectangle");
        canvas.render();
        sleep(Duration::from_millis(100));
    }
    Ok(())
}
