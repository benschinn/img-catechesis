extern crate image;

use std;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

fn main() {
     let file = std::fs::File::open("./src/input.json")
             .expect("file should open read only");
     let json: serde_json::Value = serde_json::from_reader(file)
             .expect("file should be proper JSON");
     let colors = json.get("Color")
             .expect("file should have color key");
     //println!("{:?}", colors)

     // Construct a new RGB ImageBuffer with the specified width and height.
     let img: RgbImage = ImageBuffer::new(512, 512);

     // Construct a new by repeated calls to the supplied closure.
     let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
         if x % 10 == 0 {
             image::Rgb([179, 49, 46])
         } else {
             image::Rgb([23, 179, 78])
         }
     });

     // Obtain the image's width and height.
     let (width, height) = img.dimensions();

     // Access the pixel at coordinate (2, 2).
     let pixel = img[(100, 100)];

     // Or use the ```get_pixel``` method from the ```GenericImage``` trait.
     let pixel = *img.get_pixel(100, 100);

     // Put a pixel at coordinate (2, 2).
     img.put_pixel(100, 100, pixel);
     img.save("output.png").unwrap();
}
