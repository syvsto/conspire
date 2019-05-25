use conspire::{Backend, Layer, Plot, PlotBuilder};

fn main() {
    let layer1 = Layer::new()
        .x(vec![1.0, 1.3, 2.0, 2.7, 3.0, 4.0, 5.1, 6.2, 6.3])
        .y(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0, 4.2, 4.2, 4.3])
        .name("No");

    let layer2 = Layer::new()
        .x(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0, 4.2, 4.2, 4.3])
        .name("Bop");

    let plot = PlotBuilder::new(Backend::Plotly)
        .display(true)
        .add_layer(Plot::boxplot(&layer2))
        .build();

    plot.render();
}
