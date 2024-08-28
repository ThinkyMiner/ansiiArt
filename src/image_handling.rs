use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};

use crate::color::Color;

pub struct Image {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

pub fn load_image_from_memory(bytes: &[u8]) -> Result<Image> {
    use stb_image::image;
    use stb_image::image::LoadResult::*;
    match image::load_from_memory(bytes) {
        Error(msg) => return Err(anyhow!("Could not load image from memory: {msg}")),
        ImageF32(_) => return Err(anyhow!("Could not load hdr image from memory")),
        ImageU8(image) => {
            if image.depth != 3 {
                return Err(anyhow!(
                    "Could not load image with depth != 3. It has {depth}",
                    depth = image.depth
                ));
            }

            let mut buffer: Vec<Color> = Vec::with_capacity(image.width * image.height);
            for i in (0..image.width * image.height * image.depth).step_by(image.depth) {
                buffer.push(Color::from_rgb(
                    image.data[i],
                    image.data[i + 1],
                    image.data[i + 2],
                ))
            }

            return Ok(Image {
                width: image.width,
                height: image.height,
                data: buffer,
            });
        }
    }
}

pub fn load_image_from_file(input: &str) -> Result<Image> {
    let mut reader =
        BufReader::new(File::open(input).with_context(|| format!("open {input} for reading"))?);
    let mut buffer: Vec<u8> = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .with_context(|| format!("read image data from {input}"))?;
    load_image_from_memory(buffer.as_slice())
}