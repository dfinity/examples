use image::{imageops, ImageBuffer, Rgba};
use qrcode_generator::QrCodeEcc;
use std::io::Cursor;

use crate::Options;

/// Generates a QR code image in PNG format for the given input text.
/// The requested image size should be specified in pixels.
pub(super) fn generate(
    input: String,
    options: Options,
    logo: &[u8],
    image_size: usize,
) -> Result<Vec<u8>, anyhow::Error> {
    // Generate a QR code image that can tolerate 25% of erroneous codewords.
    let mut qr = image::DynamicImage::ImageLuma8(qrcode_generator::to_image_buffer(
        input,
        QrCodeEcc::Quartile,
        image_size,
    )?)
    .into_rgba8();

    if options.add_transparency == Some(true) {
        make_transparent(&mut qr);
    }

    if options.add_logo {
        add_logo(&mut qr, logo);
    }

    if options.add_gradient {
        add_gradient(&mut qr);
    }

    let mut result = vec![];
    qr.write_to(&mut Cursor::new(&mut result), image::ImageOutputFormat::Png)?;
    Ok(result)
}

/// Replaces white pixels in the image with transparent pixels.
fn make_transparent(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (_x, _y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [255, 255, 255, 255] {
            *pixel = image::Rgba([255, 255, 255, 0]);
        }
    }
}


/// Adds the given logo at the center of QR code image.
/// It ensures that the logo does not cover more than 10% of the image, which is
/// below the QR error threshold.
fn add_logo(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, logo: &[u8]) {
    let image_size = qr.width().min(qr.height()) as usize;
    let element_size = get_qr_element_size(qr);

    // Find the right size of the logo by starting with the smallest square.
    let mut logo_size = element_size;

    // The ratio `5/16` gives about 10% when squared.
    while logo_size + 2 * element_size <= 5 * image_size / 16 {
        // Note that two elements are added in order to keep the logo at the
        // center of the image.
        logo_size += 2 * element_size;
    }

    let mut logo = image::io::Reader::new(Cursor::new(logo))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    logo = logo.resize(
        logo_size as u32,
        logo_size as u32,
        imageops::FilterType::Lanczos3,
    );

    imageops::replace(
        qr,
        &logo,
        ((image_size - logo_size) / 2) as i64,
        ((image_size - logo_size) / 2) as i64,
    );
}

/// Adds a color gradient to the black squares of the QR code image.
/// The gradient goes from the center of the image to its sides.
fn add_gradient(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let image_size = qr.width().min(qr.height()) as usize;

    // Prepare a linear gradient function based on two colors.
    // For each point `x` in range `[0.0 .. 1.0]`, the function returns a color
    // between the two initial colors: `gradient.at(x)`.
    let gradient = colorgrad::CustomGradient::new()
        .colors(&[
            colorgrad::Color::from_rgba8(100, 0, 100, 255),
            colorgrad::Color::from_rgba8(30, 5, 60, 255),
        ])
        .build()
        .unwrap();

    // The gradient goes from the center of the image to its sides.
    let center = (image_size / 2) as u32;
    for (x, y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [0, 0, 0, 255] {
            // Use a simple Manhattan distance as an estimate of how far the
            // pixel is from the center of the image.
            let distance = x.abs_diff(center) + y.abs_diff(center);
            let rgba = gradient.at(distance as f64 / image_size as f64).to_rgba8();
            *pixel = image::Rgba(rgba);
        }
    }
}

/// Given a QR code image, this function returns the size of the smallest black
/// square by inspecting the special element in the top-left part of the image.
fn get_qr_element_size(qr: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> usize {
    const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];

    let size = qr.width().min(qr.height());

    // Find the first black pixel by traversing the image diagonally starting
    // from the top-left corner.
    let mut start = size;
    for i in 0..size {
        if qr.get_pixel(i, i).0 == BLACK_PIXEL {
            start = i;
            break;
        }
    }

    // Continue the diagonal traversal until the color switches.
    let mut element_size = 1;
    for i in 0..size - start {
        if qr.get_pixel(start + i, start + i).0 != BLACK_PIXEL {
            element_size = i;
            break;
        }
    }

    element_size as usize
}
