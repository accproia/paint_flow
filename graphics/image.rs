use crate::graphics::color::{Color, RgbaColor};
use std::mem;

macro_rules! declare_buffer_trait {
    ($name:ident,$($coord:ident{$dim:ident}),+) => {

        pub trait $name<ColorTypeT, ColorComponentTypeT> 
                                    where ColorComponentTypeT:Clone, ColorTypeT: Color<ColorComponentTypeT> + Clone {

            $(fn $dim(&self) -> usize;)+
            fn colors() -> usize { // number of color components
                ColorTypeT::COMPONENTS_NUMBER
            }
            fn color_bits() -> usize { // number of bits in a single color component
                mem::size_of::<ColorComponentTypeT>() * 8 // convert byte to bits
            }
            fn init(&self, $($dim:usize), +, color:ColorTypeT); // resize without keeping data
            fn resize(&self, $($dim:usize), +);
            fn get(&self, $($coord:usize), +) -> ColorTypeT;
        }
    }
}

declare_buffer_trait!(Buffer2d, x{width}, y{height});
declare_buffer_trait!(Buffer3d, x{width}, y{height}, z{depth});

pub struct RgbaImage<ColorComponentTypeT> {
    _buffer: Vec<RgbaColor<ColorComponentTypeT>>,
    _width: usize,
    _height: usize,
}

impl<ColorComponentTypeT> RgbaImage<ColorComponentTypeT>
                                where ColorComponentTypeT:Clone {

    #[allow(dead_code)]
    fn from_size(width: usize, height: usize, color: RgbaColor<ColorComponentTypeT>) -> RgbaImage<ColorComponentTypeT> {
        assert!(width > 0);
        assert!(height > 0);

        let size = width * height;
        let mut buffer = Vec::<RgbaColor<ColorComponentTypeT>>::with_capacity(size);

        for _ in 0..size {
            buffer.push(color.clone());
        }

        RgbaImage::<ColorComponentTypeT>{ _width: width, _height: height, _buffer: buffer }
    }
}

impl<ColorComponentTypeT> Buffer2d<RgbaColor<ColorComponentTypeT>, ColorComponentTypeT> 
                                for RgbaImage<ColorComponentTypeT>
                                where ColorComponentTypeT:Clone {
    
    fn width(&self) -> usize {
        self._width
    }

    fn height(&self) -> usize {
        self._height
    }
}

#[test]
fn rgba_image_test()
{
    let image = RgbaImage::<u32>::from_size(10, 10, RgbaColor::<u32>::white()); 
    assert_eq!(image.)
}