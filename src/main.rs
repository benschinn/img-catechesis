extern crate image;

use std;
use image::{ImageBuffer, Rgb};

fn main() {
     let file = std::fs::File::open("./src/input.json")
             .expect("file should open read only");
     let json: serde_json::Value = serde_json::from_reader(file)
             .expect("file should be proper JSON");
     let colors = json.get("Color")
             .expect("file should have color key");
     //println!("{:?}", colors)

     // a default (black) image containing Rgb values
     let mut image = ImageBuffer::<Rgb<u8>, Vec<u8> >::new(500, 500);
     // set a central pixel to white
     image.get_pixel_mut(10, 5);
     // write it out to a file
     image.save("output.png").unwrap();
}
