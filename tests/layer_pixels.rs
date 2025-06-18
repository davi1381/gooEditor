use goo::LayerContent;

#[test]
fn roundtrip_pixels() {
    let width = 3;
    let height = 2;
    let pixels = vec![0u8, 1, 2, 3, 4, 5];
    let mut layer = LayerContent::default();
    layer.set_pixels(width, height, &pixels);
    assert_eq!(layer.decode_pixels(width, height), pixels);
}

#[cfg(feature = "image")]
#[test]
fn image_helpers() {
    let width = 2;
    let height = 2;
    let pixels = vec![0u8, 255, 128, 64];
    let mut layer = LayerContent::default();
    layer.set_pixels(width, height, &pixels);
    let img = layer.to_image(width, height);
    assert_eq!(img.width(), width);
    assert_eq!(img.height(), height);
    let mut layer2 = LayerContent::default();
    layer2.set_from_image(&img);
    assert_eq!(layer2.decode_pixels(width, height), pixels);
}

#[test]
fn display_example_layer() {
    use goo::LayerContent;

    let (width, height) = (8, 8);
    let mut pixels = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            // Create a simple checkerboard pattern
            if (x + y) % 2 == 0 {
                pixels.push(255); // White
            } else {
                pixels.push(0);   // Black
            }
        }
    }

    let mut layer = LayerContent::default();
    layer.set_pixels(width as u32, height as u32, &pixels);

    let decoded_pixels = layer.decode_pixels(width as u32, height as u32);

    println!("\n--- Displaying Example Layer ({}x{}) ---", width, height);
    for y in 0..height {
        let mut row_str = String::new();
        for x in 0..width {
            let pixel_value = decoded_pixels[y * width + x];
            // Use block characters for a simple visual representation in the terminal
            if pixel_value < 128 {
                row_str.push('▓'); // Dark square
            } else {
                row_str.push('░'); // Light square
            }
        }
        println!("{}", row_str);
    }
    println!("------------------------------------");

    // The test will pass if it runs without panicking.
    // To see the printed output, run this test with: cargo test -- --nocapture
    assert_eq!(decoded_pixels, pixels);
}

#[test]
fn save_and_open_first_layer_from_file() -> anyhow::Result<()> {
    use goo::GooFile;
    use image::GrayImage;
    use std::fs;
    use std::process::Command;

    let file_path = "/Users/aarondavis/Documents/CodingProjects/gooEditor/Embedded_microfluidic_mixer_0.05_6_2025_06_18_08_55_00.goo";
    let goo_data = fs::read(file_path)?;

    let goo_file = GooFile::deserialize(&goo_data)?;

    if let Some(first_layer) = goo_file.layers.get(0) {
        let width = goo_file.header.x_resolution as u32;
        let height = goo_file.header.y_resolution as u32;

        let pixels = first_layer.decode_pixels(width, height);

        let image_buffer = GrayImage::from_raw(width, height, pixels)
            .expect("Failed to create image from pixel data");

        let output_path = "first_layer.png";
        image_buffer.save(output_path)?;

        println!("First layer saved to {}", output_path);

        // Open the image with the default application
        #[cfg(target_os = "macos")]
        Command::new("open").arg(output_path).status()?;

        #[cfg(target_os = "windows")]
        Command::new("cmd").args(&["/C", "start", "", output_path]).status()?;

        #[cfg(target_os = "linux")]
        Command::new("xdg-open").arg(output_path).status()?;

    } else {
        println!("No layers found in the file.");
    }

    Ok(())
}