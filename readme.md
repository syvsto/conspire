# Conspire

Conspire (named in the grand tradition of Sussman and Steele), provides a general, Rusty interface for creating
plots. It supports multiple backends, so you can render to SVG, to a terminal, to an IPython viewer or to a browser
as you see fit.

*Beware*: In an extremely early stage, still settling on a design. Only has a single backend, only a few available
plots, and may possibly be mean to your data.

### Goals

* Provide a large set of available plots. Many plotting libraries only provide the basics, such as bar charts,
   scatterplots and line charts. Conspire aims to also provide various plots for statistics, 3D charts, statistical
   plots, and various scientific charts for applicable backends.
   
* Flexibility. The API should be flexible enough to handle many different data types, data should be easy to assign
to user-specified channels, and be in control over the layout when multiple plots are presented at the same time.
In addition, it should be possible to swap between different internal data representations to trade precision for
memory if the need arises, and there should be a well-documented path to creating new backends.
  
* Ease of use. The library should be easy to use, provide functions for working with a variety of popular data
  formats (such as [ndarray](https://docs.rs/ndarray/0.12.1/ndarray/) vectors/matrices), and gracefully handle
  different data types, be they quantitative, ordinal or nominal.
  
### Non-goals

* A custom, Rust-based backend. There are a multitude of very good plotting libraries for many different use cases
  out there, and a new, Rust-based one doesn't bring many immediate benefits. I would rather spend time making a
  solid interface.
  
* The absolute best performance. Though Rust provides a lot of performance for free through it's zero-cost abstractions,
  conspire will not attempt to optimize all aspects of performance.
  
## How to use
The following plot plots two lines using the Plotly backend.

```rust
use conspire::{Backend, PlotBuilder, Plot, Layer};
use conspire::channels::{PositionX, PositionY, Color, Size};

let layer1 = Layer::new()
   .x(vec![1.0, 1.3, 2.0, 1.7, 3.0, 4.0])
   .y(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0]);
 
 let layer2 = Layer::new()
   .x(vec![1.0, 2.0, 3.0, 4.0])
   .y(vec![9.0, 1.0, 10.0, 11.0]);
 
 let plot = PlotBuilder::new(Backend::Plotly)
   .display(true)
   .add_layer(Plot::scatter(layer1))
   .add_layer(Plot::line(layer2))
   .build();
 
 plot.render();
```

To create a plot, you make one or more `Layer`s. A Layer is a generalization over different channels to which you
can assign data. In the above example, the data is assigned the x and y dimensions of two layers.

Once you've specified your layers, you need to assign them to actual plots. Create a `PlotBuilder` by specifying a
backend, set whatever settings you want, and add each layer by converting it into a specific plot type. The `Plot`
enum provides methods for this. In the above example, the plot is set to automatically display it's output, then
the two layers are added as scatterplots. The plot is finalized using the `build` method, converting it into a
`PlotSystem`, which can be rendered.

## Lists of things

The following lists should provide some insight into the current state of the project as well as what I would like to do
with it in the near future.

### Backends
Currently, there is only one supported backend: Plotly.js. It is quite easy to build for, provides a lot of plots,
interactivity, and renders in a browser (so you don't need additional software, unless you don't have a browser, in
which case, how are you reading this?). Other backends are planned, however:

- [x] Plotly.js
- [ ] matplotlib
- [ ] asciiplot
- [ ] gnuplot
- [ ] SVG rendering (possibly)


### Supported plots

Currently, no plots have more than the bare minimum of support. Very few channels can be assigned data, and there
may be other issues. As the design stabilizes more, I expect this to change. Nevertheless, here is a list of
different categories of plots I'd like to support as quickly as possible:

**Basic plots:**
- [x] Bar chart
- [x] Scatter plot
- [x] Pie chart
- [x] Horizontal bar chart
- [x] Line chart

**Statistical plots:**
- [ ] Box plot
- [ ] Histogram (with automatic binning)
- [ ] SPLOM
- [ ] Treemap

### Features

In no particular order, here are some features I'd like to incorporate into a final design:
- [x] Automatic handling of differences in categorical and quantitative data
- [ ] Toggle between different internal datatypes depending on whether you need more precision or more memory
- [ ] Layer creation directly from common data types: Vectors of tuples, nested vectors, ndarray matrices, etc.


## Writing backends

The backend is responsible for converting data from the generic representation used in conspire into a suitable
representation for the backend, as well as handling the actual rendering of the plot. In time, each backend will be
it's own crate. Backend creation will be more fully documented in time, for the time being, take a look at the
Plotly backend in `src/backends/plotly.rs` to get an idea of how backends work.
