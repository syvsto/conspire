use conspire::{Backend, Layer, Plot, PlotBuilder};

fn main() {
    let layer1 = Layer::new()
        .x(vec![1.0, 1.3, 2.0, 2.7, 3.0, 4.0, 5.1, 6.2, 6.3])
        .y(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0, 4.2, 4.2, 4.3])
        .color(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        .size(vec![30]);

    let layer2 = Layer::new()
        .x(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0])
        .y(vec![9.0, 1.0, 10.0, 11.0, 11.0, 11.0, 11.0])
        .color(vec!["blue".to_string()]);

    let plot = PlotBuilder::new(Backend::Plotly)
        .display(true)
        .add_layer(Plot::scatter(layer1))
        .add_layer(Plot::line(layer2))
        .build();

    plot.render();
}
