extern crate png;
extern crate std;
extern crate bit_vec;

mod utils;
mod graphics;

use utils::iterators_util::{*};
use graphics::image::{*};
use graphics::color::{*};
//use bit_vec::BitVec;

const IMAGE_WIDTH: usize = 100;
const IMAGE_HEIGHT: usize = 100;

// fn initialize_image_data() -> Vec<u8> {
//     let mut img_data: Vec<u8> = Vec::new();
//     img_data.resize_with(IMAGE_WIDTH * IMAGE_HEIGHT * png::ColorType::RGBA.samples(), Default::default);
//     for (r, g, b, a) in iterate_by_tuple!(4 of &mut img_data) {
//         *r = 255u8;
//         *g = 255u8;
//         *b = 255u8;
//         *a = 255u8;
//     }
//     img_data
// }

// fn draw_line<F>(image: &mut Vec<u8>, img_width:u32, img_height:u32, pt1:(f64, f64), pt2:(f64, f64), width:f64, smoothFn:F)
//     where F: Fn(f64)->f64
// {
//     let passed = bit_vec::BitVec::from_elem((img_width * img_height) as usize, false);
    

// }

fn main() {
    let img = Image::<RgbaColor<i8>>::from_size(IMAGE_WIDTH, IMAGE_HEIGHT, RgbaColor::<i8>::white());
    
    
    // let out = std::fs::File::create("1.png").expect("Couldn't open the file");
    // let mut encoder = png::Encoder::new(out, IMAGE_WIDTH, IMAGE_HEIGHT);
    // encoder.set_color(png::ColorType::RGBA);
    // encoder.set_depth(png::BitDepth::Eight);
    // let mut writer = encoder.write_header().expect("Couldn't write a header");
    // let img_data = initialize_image_data();
    // //draw_line(&mut img_data, IMAGE_WIDTH, IMAGE_HEIGHT, (0.0,0.0), (IMAGE_WIDTH as f64, IMAGE_HEIGHT as f64), 4.7, |x|x);
    // writer.write_image_data(&img_data).expect("Error occured during file writing");
}