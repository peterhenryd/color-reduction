# color-reduction
[![Crates.io](https://img.shields.io/crates/v/color-reduction)](https://crates.io/crates/color-reduction)

A Rust library to reduce the colors of an image to a specified set of colors.

## How to Use

![Old Image](https://i.imgur.com/pBcsyBq.jpg)

... combined with ...

```rust
use color_reduction::reduce_colors;
use color_reduction::image::Rgb;
use color_reduction::image;

fn main() {
    let image = image::open("./old-image.jpg")
        .expect("error loading image");
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

    new_image.save("./new.jpg").expect("error saving image");
}
```

... turns into ...

![New Image](https://i.imgur.com/ibvl93F.jpg)
