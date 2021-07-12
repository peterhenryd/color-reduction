pub extern crate image;

use image::{GenericImage, Pixel, RgbImage, Rgb, GenericImageView, ImageBuffer};

/// Reduces the colors of an image to a specified set of [colors].
/// The similarity algorithm matches human perception so visibly similar color should correlate.
pub fn reduce_colors<T: GenericImage> (image: T, colors: &[Rgb<u8>]) -> RgbImage where <T as GenericImageView>::Pixel: Pixel<Subpixel = u8> {
    let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            let color = pixel.to_rgb();

            let mut most_similar = &Rgb::from([0, 0, 0]);
            let mut similarity = 255i32.pow(2) * 3 + 1;
            for c in colors {
                let r = c.0[0] as i32 - color.0[0] as i32;
                let g = c.0[1] as i32 - color.0[1] as i32;
                let b = c.0[2] as i32 - color.0[2] as i32;
                let s = r.pow(2) + g.pow(2) + b.pow(2);
                if similarity > s {
                    similarity = s;
                    most_similar = c;
                }
            }

            new_image.put_pixel(x, y, *most_similar);
        }
    }

    new_image
}

#[cfg(test)]
mod tests {
    use crate::reduce_colors;
    use image::Rgb;

    #[test]
    fn it_works() {
        let image = image::open("old-image.jpg").unwrap();
        let new_image = reduce_colors(image, &[
            Rgb::from([240,207,149]),
            Rgb::from([24,30,75]),
            Rgb::from([10,6,11]),
            Rgb::from([142,117,98]),
            Rgb::from([67,62,70]),
            Rgb::from([28,42,84]),
            Rgb::from([19,13,20]),
            Rgb::from([248,217,158]),
            Rgb::from([9,4,7]),
            Rgb::from([62,53,52]),
            Rgb::from([62,71,99]),
            Rgb::from([134,125,118]),
            Rgb::from([198,154,106]),
            Rgb::from([15,14,59]),
            Rgb::from([113,90,78]),
            Rgb::from([255, 255, 255]),
            Rgb::from([255, 249, 189]),
        ]);

        new_image.save("new.jpg");
    }
}