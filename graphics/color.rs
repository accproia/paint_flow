extern crate num_traits;
use std::mem;
use num_traits::bounds::Bounded;

pub trait Color<T> where T:Clone {
    fn size(&self) -> usize;
    fn get(&self, num: usize) -> T; 
}

pub trait BaseColor {
    const COMPONENTS_NUMBER: usize;
    const COMPONENT_BITS_NUMBER: usize;
}

#[derive(Clone, PartialEq, Debug)]
pub struct RgbaColor<T> {
    color: [T; 4],
}

impl<T> RgbaColor<T> where T:Bounded{
    #[allow(dead_code)]
    pub fn white() -> RgbaColor<T> {
        RgbaColor{color: [T::max_value(), T::max_value(), T::max_value(), T::max_value()]}
    }
    #[allow(dead_code)]
    pub fn black() -> RgbaColor<T> {
        RgbaColor{color: [T::min_value(), T::min_value(), T::min_value(), T::min_value()]}
    }
}

impl<T> BaseColor for RgbaColor<T> where T:Clone {
    const COMPONENTS_NUMBER: usize = 4usize;
    const COMPONENT_BITS_NUMBER: usize = mem::size_of::<T>() * 8;
}

impl<T> Color<T> for RgbaColor<T> where T:Clone {
    fn size(&self) -> usize {
        self.color.len()
    }
    fn get(&self, num: usize) -> T {
        assert!(num < Self::COMPONENTS_NUMBER);
        self.color[num].clone()
    }
}

#[test]
fn rgba_color_test()
{
    let white = RgbaColor::<i8>::white();
    for i in 0..RgbaColor::<i8>::COMPONENTS_NUMBER {
        assert_eq!(white.get(i), std::i8::MAX);
    }
    
    let black = RgbaColor::<i8>::black();
    for i in 0..RgbaColor::<i8>::COMPONENTS_NUMBER {
        assert_eq!(black.get(i), std::i8::MIN);
    }
}