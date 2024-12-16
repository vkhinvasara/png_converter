use image::GenericImageView;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Load the image
    let img = image::open("RunningManjoo2.1.png").expect("Failed to open image");

    // Get dimensions
    let (width, height) = img.dimensions();

    // Iterate over pixels and collect ARGB values
    let mut pixel_data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let argb = ((rgba[3] as u32) << 24)    // Alpha
                     | ((rgba[0] as u32) << 16)    // Red
                     | ((rgba[1] as u32) << 8)     // Green
                     | (rgba[2] as u32); // Blue
            pixel_data.push(argb);
        }
    }

    // Print the pixel data
    let file = File::create("running_manjoo2.txt").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    for (i, val) in pixel_data.iter().enumerate() {
        write!(writer, "0x{:08x}, ", val).expect("Failed to write to file");
        if (i + 1) % width as usize == 0 {
            writeln!(writer).expect("Failed to write to file");
        }
    }
}
