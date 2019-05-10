use crate::Plot;
use crate::data::{Quantitative, Nominal, Ordinal};

use std::error;

mod plotly;
mod util;

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
    fn render<T, U, V>(&self, data: &[Plot<T, U, V>], display: bool) -> Result<()> where T: Quantitative, U: Nominal, V: Ordinal;
}
