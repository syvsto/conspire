use crate::Plot;

use std::error;

mod plotly;
mod util;
mod common;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone)]
pub enum Backend {
    Plotly,
}

impl Backend {
    pub(crate) fn to_struct(self) -> impl Renderable {
        match self {
            Backend::Plotly => plotly::Plotly {},
        }
    }
}

pub trait Renderable {
    fn render(&self, data: &[Plot], display: bool) -> Result<()>;
}
