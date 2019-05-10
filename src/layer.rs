use crate::data::{Ordinal, Nominal, Quantitative};
use crate::channels::{PositionX, PositionY, Color, Size};

pub struct Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    x: Vec<T>,
    y: Vec<T>,
    color: Vec<V>,
    size: Vec<T>,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U, V> Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    pub fn new() -> Self {
        Self {
            x: Vec::new(),
            y: Vec::new(),
            color: Vec::new(),
            size: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, U, V> PositionX<T> for Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    fn x(mut self, data: &[T]) -> Self {
        self.x = data.to_vec();
        self
    }

    fn get_x(&self) -> Option<&[T]> {
        if self.x.is_empty() {
            None
        } else {
            Some(&self.x)
        }
    }
}

impl<T, U, V> PositionY<T> for Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    fn y(mut self, data: &[T]) -> Self {
        self.y = data.to_vec();
        self
    }

    fn get_y(&self) -> Option<&[T]> {
        if self.y.is_empty() {
            None
        } else {
            Some(&self.y)
        }
    }
}

impl<T, U, V> Color<V> for Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    fn color(mut self, data: &[V]) -> Self {
        self.color = data.to_vec();
        self
    }

    fn get_color(&self) -> Option<&[V]> {
                if self.color.is_empty() {
            None
        } else {
            Some(&self.color)
        }
    }
}

impl<T, U, V> Size<T> for Layer<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    fn size(mut self, data: &[T]) -> Self {
        self.size = data.to_vec();
        self
    }

    fn get_size(&self) -> Option<&[T]> {
        if self.size.is_empty() {
            None
        } else {
            Some(&self.size)
        }
    }
}
