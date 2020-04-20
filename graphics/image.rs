use crate::graphics::color::{BaseColor};
use crate::{multiply, get_number_of_variadic_arguments, coord_to_shift, last_one};
use std::cmp;

macro_rules! generate_dim_getters_ {
    ($num:expr, $dim:ident) => {
        #[allow(dead_code)]
        fn $dim(&self) -> usize{self._sizes[$num]}
    };
    ($num:expr, $dim:ident, $($dims:ident),+) => {
        #[allow(dead_code)]
        fn $dim(&self) -> usize{self._sizes[$num]}
        generate_dim_getters_!(1+$num, $($dims),+);
    };
}

macro_rules! generate_dim_getters {
    ($dim:ident) => {pub fn $dim(&self){self._sizes[0]}};
    ($dim:ident, $($dims:ident),+) => {generate_dim_getters_!(0, $dim, $($dims),+);};
}

macro_rules! implement_image {
    ($name:ident,$($coord:ident{$dim:ident}),+) => {

        pub struct $name<ColorTypeT> {
            _buffer: Vec<ColorTypeT>,
            _sizes: [usize; get_number_of_variadic_arguments!($($dim),+)],
        }
        
        impl<ColorTypeT> $name<ColorTypeT> 
                        where ColorTypeT:Clone + BaseColor {

            #[allow(dead_code)]
            pub fn from_size($($dim:usize),+, color: ColorTypeT) -> $name<ColorTypeT> {
                let mut image = $name::<ColorTypeT>{_buffer: Vec::<ColorTypeT>::new(), _sizes: [$($dim),+]};
                image.init($($dim),+, color);
                image
            }

            #[allow(dead_code)]
            pub fn size(&self, num:usize) -> usize{
                self._sizes[num]
            }

            generate_dim_getters!($($dim),+);

            #[allow(dead_code)]
            pub fn colors() -> usize { // number of color components
                ColorTypeT::COMPONENTS_NUMBER
            }
            #[allow(dead_code)]
            pub fn color_bits() -> usize { // number of bits in a single color component
                ColorTypeT::COMPONENT_BITS_NUMBER // convert byte to bits
            }
            #[allow(dead_code)]
            pub fn init(&mut self, $($dim:usize),+, color: ColorTypeT){ // resize without keeping data
                $(assert!($dim > 0));+;
        
                let size = multiply!($($dim),+);
                let minsize = cmp::min(size, self._buffer.len());

                for i in 0..minsize {
                    self._buffer[i] = color.clone();
                }

                self._buffer.resize(size, color.clone());
            }
            // fn resize(&self, $($dim:usize), +);
            #[allow(dead_code)]
            pub fn get(&self, $($coord:usize), +) -> ColorTypeT{
                self._buffer[coord_to_shift!(self._sizes, $($coord),+)].clone()
            }

            #[allow(dead_code)]
            pub fn get_data(&self) -> &Vec<ColorTypeT>
            {
                return &self._buffer;
            }
            
            #[allow(dead_code)]
            pub fn get_mut_data(&mut self) -> &mut Vec<ColorTypeT>
            {
                return &mut self._buffer;
            }
            
            #[allow(dead_code)]
            pub fn set(&mut self, $($coord:usize), +, color:&ColorTypeT){
                self._buffer[coord_to_shift!(self._sizes, $($coord),+)] = color.clone();
            }
        }
    }
}

implement_image!(Image, x{width}, y{height});
implement_image!(Volume, x{width}, y{height}, z{depth});


#[cfg(test)]
mod images_test
{
    use crate::graphics::color::{RgbaColor};
    use crate::graphics::image::Image;
    
    #[test]
    fn rgba_image_test()
    {
        let mut image = Image::<RgbaColor<u32>>::from_size(10, 10, RgbaColor::<u32>::white()); 
        assert_eq!(image.get(0, 0), RgbaColor::<u32>::white());
        assert_eq!(image.get(5, 4), RgbaColor::<u32>::white());
        assert_eq!(image.get(9, 6), RgbaColor::<u32>::white());
        image.set(9, 6, &RgbaColor::<u32>::black());
        assert_eq!(image.get(9, 6), RgbaColor::<u32>::black());
    }

}