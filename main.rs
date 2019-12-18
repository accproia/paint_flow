extern crate png;
extern crate std;

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;

fn initializeImageData() -> [u8; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize] {
    let img_data:[u8; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    img_data
}

fn main() {
    let out = std::fs::File::open("1.png").expect("Couldn't open the file");
    let mut encoder = png::Encoder::new(out, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Couldn't write a header");
    let img_data = initializeImageData();
    writer.write_image_data(&img_data).expect("Error occured during file writing");
}