use crate::data::{DataType, Plottable};

pub struct Layer {
    x: Option<DataType>,
    y: Option<DataType>,
    color: Option<DataType>,
    size: Option<DataType>,
    name: Option<String>,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            color: None,
            size: None,
            name: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn x(mut self, data: impl Plottable) -> Self {
        self.x = Some(data.to_conspire_data());
        self
    }

    pub fn get_x(&self) -> &Option<DataType> {
        &self.x
    }

    pub fn y(mut self, data: impl Plottable) -> Self {
        self.y = Some(data.to_conspire_data());
        self
    }

    pub fn get_y(&self) -> &Option<DataType> {
        &self.y
    }

    pub fn color(mut self, data: impl Plottable) -> Self {
        self.color = Some(data.to_conspire_data());
        self
    }

    pub fn get_color(&self) -> &Option<DataType> {
        &self.color
    }

    pub fn size(mut self, data: impl Plottable) -> Self {
        self.size = Some(data.to_conspire_data());
        self
    }

    pub fn get_size(&self) -> &Option<DataType> {
        &self.size
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
}
