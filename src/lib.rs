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
use data::{ VectorData, MatrixData };

pub enum Plot<'a> {
    Scatter {
        x: &'a VectorData,
        y: &'a VectorData,
        color: &'a Option<VectorData>,
        size: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    Line {
        x: &'a VectorData,
        y: &'a VectorData,
        color: &'a Option<VectorData>,
        size: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    Bar {
        x: &'a VectorData,
        y: &'a VectorData,
        color: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    Pie {
        x: &'a VectorData,
        color: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    HorizontalBar{
        x: &'a VectorData,
        y: &'a VectorData,
        color: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    Box {
        x: &'a VectorData,
        color: &'a Option<VectorData>,
        name: &'a Option<String>,
    },
    SimpleHeatmap {
        z: &'a MatrixData,
        color: &'a Option<MatrixData>,
        name: &'a Option<String>,
    },
}

impl<'a> Plot<'a> {
    pub fn scatter(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::Scatter {
            x: plot.get_x().as_ref().expect("No X axis found"),
            y: plot.get_y().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            size: plot.get_size(),
            name: plot.get_name(),
        }
    }

    pub fn line(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::Line {
            x: plot.get_x().as_ref().expect("No X axis found"),
            y: plot.get_y().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            size: plot.get_size(),
            name: plot.get_name(),
        }
    }

    pub fn bar(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::Bar {
            x: plot.get_x().as_ref().expect("No X axis found"),
            y: plot.get_y().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            name: plot.get_name(),
        }
    }

    pub fn pie(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::Pie {
            x: plot.get_x().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            name: plot.get_name(),
        }
    }

    pub fn horizontal_bar(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::HorizontalBar {
            x: plot.get_x().as_ref().expect("No X axis found"),
            y: plot.get_y().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            name: plot.get_name(),
        }
    }

    pub fn boxplot(plot: &'a Layer<VectorData>) -> Plot<'a> {
        Plot::Box {
            x: plot.get_x().as_ref().expect("No X axis found"),
            color: plot.get_color(),
            name: plot.get_name(),
        }
    }

    pub fn heatmap(plot: &'a Layer<MatrixData>) -> Plot<'a> {
        Plot::SimpleHeatmap {
            z: plot.get_x().as_ref().expect("No Y axis found"),
            color: plot.get_color(),
            name: plot.get_name(),
        }
    }
}

/// A plot under construction
pub struct PlotBuilder<'a> {
    backend: Backend,
    display: bool,
    data: Vec<Plot<'a>>,
}

impl<'a> PlotBuilder<'a> {
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

    pub fn add_layer(mut self, data: Plot<'a>) -> Self {
        self.data.push(data);
        self
    }

    pub fn display(mut self, should_display: bool) -> Self {
        self.display = should_display;
        self
    }

    pub fn build(self) -> PlotSystem<'a> {
        if self.data.len() < 1 { panic!("Cannot make a plot without data") };

        PlotSystem {
            backend: self.backend,
            display: self.display,
            data: self.data,
        }
    }
}

pub struct PlotSystem<'a> {
    backend: Backend,
    display: bool,
    data: Vec<Plot<'a>>,
}

impl<'a> PlotSystem<'a> {
    pub fn render(&self) {
        let backend = self.backend.clone();
        let _ = backend.to_struct().render(&self.data, self.display);
    }
}
