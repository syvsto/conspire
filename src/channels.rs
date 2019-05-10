use crate::data::{Quantitative, Nominal, Ordinal};

pub trait PositionX<T: Quantitative> {
    fn x(self, data: &[T]) -> Self; 
    fn get_x(&self) -> Option<&[T]>;
}

pub trait PositionY<T: Quantitative> {
    fn y(self, data: &[T]) -> Self; 
    fn get_y(&self) -> Option<&[T]>;
}

pub trait Color<T: Ordinal> {
    fn color(self, data: &[T]) -> Self; 
    fn get_color(&self) -> Option<&[T]>;
}

pub trait Size<T: Quantitative> {
    fn size(self, data: &[T]) -> Self; 
    fn get_size(&self) -> Option<&[T]>;
}
