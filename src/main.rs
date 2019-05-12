use conspire::{Backend, PlotBuilder, Plot, Layer};


fn main() {
    let layer1 = Layer::new()
        .x(vec![1.0, 1.3, 2.0, 1.7, 3.0, 4.0])
        .y(vec![8.0, 8.1, 7.0, 6.4, 5.0, 4.0]);

    let layer2 = Layer::new()
        .x(vec![1.0, 2.0, 3.0, 4.0])
        .y(vec![9.0, 1.0, 10.0, 11.0])
        .color(vec![1,2,3,4]);

    let plot = PlotBuilder::new(Backend::Plotly)
        .display(true)
        .add_layer(Plot::scatter(layer1))
        .add_layer(Plot::line(layer2))
        .build();

    plot.render();
}
