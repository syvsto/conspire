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
//! let layer1 = Layer::new()
//!     .x(vec![1.0, 1.3, 2.0, 1.7, 3.0, 4.0])
//!     .y(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0]);
//!
//! let layer2 = Layer::new()
//!     .x(vec![1.0, 2.0, 3.0, 4.0])
//!     .y(vec![9.0, 1.0, 10.0, 11.0])
//!     .color(vec![1,2,3,4]);
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
mod backends;
mod error;
mod layer;

pub use backends::Backend;
pub use error::DimensionError;
pub use layer::Layer;
use backends::*;

pub enum Plot {
    Scatter(Layer),
    Line(Layer),
    Bar(Layer),
    Pie(Layer),
    HorizontalBar(Layer),
    Box(Layer),
    SimpleHeatmap(Layer),
}

impl Plot {
    pub fn scatter(plot: Layer) -> Self {
        Plot::Scatter(plot)
    }

    pub fn line(plot: Layer) -> Self {
        Plot::Line(plot)
    }

    pub fn bar(plot: Layer) -> Self {
        Plot::Bar(plot)
    }

    pub fn pie(plot: Layer) -> Self {
        Plot::Pie(plot)
    }

    pub fn horizontal_bar(plot: Layer) -> Self {
        Plot::HorizontalBar(plot)
    }

    pub fn box(plot: Layer) -> Self {
        Plot::Box(plot)
    }

    pub fn heatmap(plot: Layer) -> Self {
        Plot::SimpleHeatmap(plot)
    }
    
    fn get_layer(&self) -> &Layer {
        match self {
            Plot::Scatter(l) => &l,
            Plot::Line(l) => &l,
            Plot::Bar(l) => &l,
            Plot::Pie(l) => &l,
            Plot::HorizontalBar(l) => &l,
            Plot::Box(l) => &l,
            Plot::SimpleHeatmap(l) => &l,
        }
    }
}

/// A plot under construction
pub struct PlotBuilder{
    backend: Backend,
    display: bool,
    data: Vec<Plot>,
}

impl PlotBuilder {
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

    pub fn add_layer(mut self, data: Plot) -> Self {
        self.data.push(data);
        self
    }

    pub fn display(mut self, should_display: bool) -> Self {
        self.display = should_display;
        self
    }

    pub fn build(self) -> PlotSystem {
        if self.data.len() < 1 { panic!("Cannot make a plot without data") };

        PlotSystem {
            backend: self.backend,
            display: self.display,
            data: self.data,
        }
    }
}

pub struct PlotSystem {
    backend: Backend,
    display: bool,
    data: Vec<Plot>,
}

impl PlotSystem {
    pub fn render(&self) {
        let backend = self.backend.clone();
        let _ = backend.to_struct().render(&self.data, self.display);
    }
}
