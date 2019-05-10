//! Conspire provides a general, Rusty interface for creating plots. It supports multiple backends, so you can render
//! to SVG, to a terminal, to an IPython viewer or to a browser as you see fit. 
//! 
//! ## Goals
//! 
//! * Provide a large set of available plots. Many plotting libraries only provide the basics, such as bar charts,
//!    scatterplots and line charts. Conspire aims to also provide various plots for statistics, 3D charts, statistical
//!    plots, and various scientific charts for applicable backends.
//!    
//! * Flexibility. Allow for specifying multiple different channels for each plot, and let the user display multiple
//!   plots simultaneously, in different layouts.
//!   
//! * Ease of use. The library should be easy to use, provide functions for working with a variety of popular data
//!   formats (such as [ndarray](https://docs.rs/ndarray/0.12.1/ndarray/) vectors/matrices), and gracefully handle
//!   different data types, be they quantitative, ordinal or nominal.
//!   
//! ## Non-goals
//! 
//! * A custom, Rust-based backend. There are a multitude of very good plotting libraries for many different use cases
//!   out there, and a new, Rust-based one doesn't bring many benefits. I would rather spend time making a solid interface.
//!   
//! * The absolute best performance. Though Rust provides a lot of performance for free through it's zero-cost abstractions,
//!   conspire will not attempt to optimize all aspects of performance.
//!   
//! # How to use
//! The following plot plots two lines using the Plotly backend.
//! 
//! ```
//! use conspire::{Backend, PlotBuilder, Plot, Layer};
//! use conspire::channels::{PositionX, PositionY, Color, Size};
//! 
//! let layer1: Layer<f32, u8, u32> = Layer::new()
//!    .x(&[1.0, 2.0, 3.0, 4.0])
//!    .y(&[8.0, 7.0, 5.0, 4.0]);
//!  
//!  let layer2: Layer<f32, u8, u32> = Layer::new()
//!    .x(&[1.0, 2.0, 3.0, 4.0])
//!    .y(&[9.0, 1.0, 10.0, 11.0]);
//!  
//!  let plot = PlotBuilder::new(Backend::Plotly)
//!    .display(true)
//!    .add_layer(Plot::scatter(layer1))
//!    .add_layer(Plot::scatter(layer2))
//!    .build();
//!  
//!  plot.render();
//! ```
//! 
//! To create a plot, you make one or more `Layer`s. A Layer is a generalized plot, specifying what data should go into
//! which channels. In the above example, the data is assigned the x and y dimensions of two layers.
//! 
//! Once you've specified your layers, you need to assign them to actual plots. Create a `PlotBuilder` by specifying a
//! backend, set whatever settings you want, and add each layer by converting it into a specific plot type. The `Plot`
//! enum provides methods for this. In the above example, the plot is set to automatically display it's output, then
//! the two layers are added as scatterplots. The plot is finalized using the `build` method, converting it into a
//! `PlotSystem`, which can be rendered.

mod data;
pub mod channels;
mod backends;
mod error;
mod layer;

pub use backends::Backend;
pub use error::DimensionError;
pub use layer::Layer;
use backends::*;
use data::{Quantitative, Nominal, Ordinal};

pub enum Plot<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    Scatter(Layer<T, U, V>),
    Line(Layer<T, U, V>),
    Bar(Layer<T, U, V>),
    Pie(Layer<T, U, V>),
    HorizontalBar(Layer<T, U, V>),
}

impl<T, U, V> Plot<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    pub fn scatter(plot: Layer<T, U, V>) -> Self {
        Plot::Scatter(plot)
    }

    pub fn line(plot: Layer<T, U, V>) -> Self {
        Plot::Line(plot)
    }

    pub fn bar(plot: Layer<T, U, V>) -> Self {
        Plot::Bar(plot)
    }

    pub fn pie(plot: Layer<T, U, V>) -> Self {
        Plot::Pie(plot)
    }

    pub fn horizontal_bar(plot: Layer<T, U, V>) -> Self {
        Plot::HorizontalBar(plot)
    }

    fn get_layer(&self) -> &Layer<T, U, V> {
        match self {
            Plot::Scatter(l) => &l,
            Plot::Line(l) => &l,
            Plot::Bar(l) => &l,
            Plot::Pie(l) => &l,
            Plot::HorizontalBar(l) => &l,
        }
    }
}

/// Constructs a new plot
///
/// To make a plot, make a 
pub struct PlotBuilder<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal{
    backend: Backend,
    display: bool,
    data: Vec<Plot<T, U, V>>,
}

impl<T, U, V> PlotBuilder<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            display: false,
            data: Vec::new(),
        }
    }

    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    pub fn add_layer(mut self, data: Plot<T, U, V>) -> Self {
        self.data.push(data);
        self
    }

    pub fn display(mut self, should_display: bool) -> Self {
        self.display = should_display;
        self
    }

    pub fn build(self) -> PlotSystem<T, U, V> {
        if self.data.len() < 1 { panic!("Cannot make a plot without data") };

        PlotSystem {
            backend: self.backend,
            display: self.display,
            data: self.data,
        }
    }
}

pub struct PlotSystem<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    backend: Backend,
    display: bool,
    data: Vec<Plot<T, U, V>>,
}

impl<T, U, V> PlotSystem<T, U, V> where T: Quantitative, U: Nominal, V: Ordinal {
    pub fn render(&self) {
        let backend = self.backend.clone();
        let _ = backend.to_struct().render(&self.data, self.display);
    }
}
