use log::{error, trace};
use std::{fs::File, os::unix::prelude::FileExt, time::Instant};

const CLEAR_CHAR: u8 = 0x20;
const RECT_CHAR: u8 = 0x23;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub pos: Vec2,
    pub size: Vec2,
}

/// Provides a canvas to draw rectangles and render them to a file
#[derive(Debug)]
pub struct Canvas {
    size: Vec2,
    /// resolution
    res: u32,
    /// content to render
    content: Vec<Vec<u8>>,
    file: File,
    /// content scaled to resolution
    screen: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum CanvasError {
    OutOfBounds,
}

impl Canvas {
    pub fn new(size: Vec2, res: u32, file: File) -> Self {
        let content = vec![vec![CLEAR_CHAR; size.x as usize]; size.y as usize];
        let mut screen = vec![vec![0; (size.x * res + 1) as usize]; (size.y * res) as usize];
        for row in screen.iter_mut() {
            row[size.x as usize] = 0x0A;
        }

        Self {
            size,
            res,
            content,
            file,
            screen,
        }
    }

    /// Scale content to canvas resolution and add line returns.
    fn scale_screen(&mut self) {
        for (i, row) in self.screen.iter_mut().enumerate() {
            for (j, pixel) in row.iter_mut().enumerate() {
                *pixel = if j == (self.size.x * self.res) as usize {
                    // add line break at the end of each row
                    0x0A
                } else {
                    self.content[(i as f32 / self.res as f32) as usize]
                        [(j as f32 / self.res as f32) as usize]
                }
            }
        }
    }

    /// Draw a rectangle on the canvas.
    pub fn draw_rect(&mut self, rect: Rectangle) -> Result<(), CanvasError> {
        if rect.pos.x + rect.size.x > self.size.x || rect.pos.y + rect.size.y > self.size.y {
            error!(
                "Rectangle out of bounds: {:?}, Canva size: {:?}",
                rect, self.size
            );
            return Err(CanvasError::OutOfBounds);
        }

        for i in rect.pos.y..(rect.pos.y + rect.size.y) {
            for j in rect.pos.x..(rect.pos.x + rect.size.x) {
                self.content[i as usize][j as usize] = RECT_CHAR;
            }
        }
        Ok(())
    }

    /// Clear canvas buffer.
    pub fn clear(&mut self) {
        for row in self.content.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = CLEAR_CHAR;
            }
        }
    }

    /// Render canvas buffer to a file.
    pub fn render(&mut self) {
        let now = Instant::now();
        self.scale_screen();
        let buf = self.screen.concat();
        self.file
            .write_all_at(&buf, 0)
            .expect("Could not write to file");
        trace!("Rendered in {}ms", now.elapsed().as_millis());
    }
}
