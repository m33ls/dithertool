#[macro_use]
extern crate bmp;
//use bmp::{Image, Pixel};
use image::{RgbImage, Rgb, ImageReader, ImageBuffer, DynamicImage};

fn basic_dither(img:DynamicImage) -> RgbImage {
    let mut error:i32 = 0;
    //let mut output = Image::new(img.get_width(), img.get_height());
    let mut imgbuf = img.into_rgb8();
    let mut output = RgbImage::new(imgbuf.width(), imgbuf.height());

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let mut value = (r as i32 + g as i32 + b as i32) / 3;
   
        value = value + error;

        if value > 127 {
            error = 0 - (255 - value);
            value = 255;
        } else {
            error = value;
            value = 0;
        }

        output.put_pixel(x, y, Rgb([value as u8, value as u8, value as u8]));
    }

    return output;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("No path entered");
    let mut img = ImageReader::open(path)?.with_guessed_format()?.decode()?;

    img = img.resize(512,512, image::imageops::FilterType::CatmullRom ); 

    let mut new_img = RgbImage::new(img.width(), img.height());
    
    new_img = basic_dither(img);

    new_img.save("result.bmp").unwrap();
    Ok(())
}

