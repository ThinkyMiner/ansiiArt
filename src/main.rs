mod color;
mod image_handling;
mod resizing;

use anyhow::Result;
use crossterm::style::{Color, Colors, Print, ResetColor, SetColors};
use crossterm::{terminal, QueueableCommand};
use image_handling::load_image_from_file;
use resizing::resize_lanczos;
use std::io::{stdout, Write};

const BLOCK_CHAR: &'static str = "▀";

fn main() -> Result<()> {
    let image = load_image_from_file("images/test1.jpg")?;
    println!(
        "Loaded image {width}x{height}",
        width = image.width,
        height = image.height
    );
    let (term_width, term_height) = terminal::size()?;

    let width_ratio = term_width as f64 / image.width as f64;
    let height_ratio = term_height as f64 * 2.0 / image.height as f64;
    let image_ratio = width_ratio.min(height_ratio);

    let target_width = (image.width as f64 * image_ratio).floor() as usize;
    let target_height = (image.height as f64 * image_ratio).floor() as usize;

    let scaled_image = resize_lanczos(&image, target_width, target_height, 2.0)?;

    let mut stdout = stdout();

    for y in (0..scaled_image.height).step_by(2) {
        for x in 0..scaled_image.width {
            let foreground = scaled_image.data[y * scaled_image.width + x];
            let background = scaled_image.data[(y + 1) * scaled_image.width + x];
            stdout.queue(SetColors(Colors::new(
                Color::Rgb {
                    r: foreground.r,
                    g: foreground.g,
                    b: foreground.b,
                },
                Color::Rgb {
                    r: background.r,
                    g: background.g,
                    b: background.b,
                },
            )))?;
            stdout.queue(Print(BLOCK_CHAR))?;
        }
        stdout.queue(ResetColor)?;
        stdout.queue(Print("\n"))?;
    }

    stdout.flush()?;

    Ok(())
}