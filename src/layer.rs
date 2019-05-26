use crate::data::{Plottable};

pub struct Layer<T> {
    x: Option<T>,
    y: Option<T>,
    color: Option<T>,
    size: Option<T>,
    name: Option<String>,
}

impl<T> Layer<T> {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            color: None,
            size: None,
            name: None,
        }
    }

    pub fn name<'a>(mut self, name: &'a str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
    
    pub fn x(mut self, data: impl Plottable<D = T>) -> Self {
        self.x = Some(data.to_conspire_data());
        self
    }

    pub fn get_x(&self) -> &Option<T> {
        &self.x
    }

    pub fn y(mut self, data: impl Plottable<D = T>) -> Self {
        self.y = Some(data.to_conspire_data());
        self
    }

    pub fn get_y(&self) -> &Option<T> {
        &self.y
    }

    pub fn color(mut self, data: impl Plottable<D = T>) -> Self {
        self.color = Some(data.to_conspire_data());
        self
    }

    pub fn get_color(&self) -> &Option<T> {
        &self.color
    }

    pub fn size(mut self, data: impl Plottable<D = T>) -> Self {
        self.size = Some(data.to_conspire_data());
        self
    }

    pub fn get_size(&self) -> &Option<T> {
        &self.size
    }
}
