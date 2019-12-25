extern crate png;
extern crate std;

mod utils;

use utils::iterators_util::TupleIteratable;

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

fn initialize_image_data() -> Vec<u8> {
    let mut img_data: Vec<u8> = Vec::new();
    img_data.resize_with((IMAGE_WIDTH * IMAGE_HEIGHT) as usize * png::ColorType::RGBA.samples(), Default::default);
    let mut iter = TupleIteratable::new(&mut img_data);
    for (r, g, b, a) in &mut iter {
        *r = 255u8;
        *g = 255u8;
        *b = 255u8;
        *a = 255u8;
    }
    img_data
}

fn main() {
    let out = std::fs::File::create("1.png").expect("Couldn't open the file");
    let mut encoder = png::Encoder::new(out, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Couldn't write a header");
    let img_data = initialize_image_data();
    writer.write_image_data(&img_data).expect("Error occured during file writing");
}