pub extern crate image;

use image::{GenericImage, Pixel, RgbImage, Rgb, GenericImageView, ImageBuffer};

/// Reduces the colors of an image to a specified set of [colors].
/// The similarity algorithm matches the human perception of colors, so the image will look as close
/// as it can to how it did before it was ran through this function.
pub fn reduce_colors<T: GenericImage> (image: T, colors: &[Rgb<u8>]) -> RgbImage
    where <T as GenericImageView>::Pixel: Pixel<Subpixel = u8> {
    // The new image with the reduced colors.
    let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());

    // Loop through every pixel horizontally.
    for x in 0..image.width() {
        // Loop through every pixel vertically.
        for y in 0..image.height() {
            // Grab an individual pixel.
            let pixel = image.get_pixel(x, y);
            // Grab the color of the pixel.
            let color = pixel.to_rgb();

            // The color in the given `colors` that is most visually similar to the pixel's color.
            // The default color is black, and this may change if the following algorithm determines
            // a color that is more similar.
            let mut most_similar = &Rgb::from([0, 0, 0]);
            // The numerical similarity of the given color in `colors` to the pixel color.
            // This is recalculated with every loop, and is a color with higher similarity is found,
            // this value is changed to the new value (`most_similar` is also changed).
            let mut similarity = 255i32.pow(2) * 3 + 1;
            for c in colors {
                // Calculate the difference between the red of the given color and pixel color.
                let r = c.0[0] as i32 - color.0[0] as i32;
                // Calculate the difference between the green of the given color and pixel color.
                let g = c.0[1] as i32 - color.0[1] as i32;
                // Calculate the difference between the blue of the given color and pixel color.
                let b = c.0[2] as i32 - color.0[2] as i32;
                // Do something magical to combine all four differences into one similarity value.
                // I don't really know how this works, but if someone does, please feel free to fork
                // and change this comment.
                let s = r.pow(2) + g.pow(2) + b.pow(2);

                // If the given color is more similar than the current most similar color, then
                // change the color to the new most similar color.
                if similarity > s {
                    similarity = s;
                    most_similar = c;
                }
            }

            // Set the pixel in the new image to the most similar color that was calculated.
            // Every pixel in the image will be set, and if a similar color was not found, then
            // black will be used.
            new_image.put_pixel(x, y, *most_similar);
        }
    }

    new_image
}