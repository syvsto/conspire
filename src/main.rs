use conspire::{Backend, PlotBuilder, Plot, Layer};
use conspire::channels::{PositionX, PositionY, Color, Size};


fn main() {
    let layer1: Layer<f32, u8, u32> = Layer::new()
        .x(&[1.0, 2.0, 3.0, 4.0])
        .y(&[8.0, 7.0, 5.0, 4.0])
        .size(&[1.0, 1.0, 4.0, 1.0]);

    let layer2: Layer<f32, u8, u32> = Layer::new()
        .x(&[1.0, 2.0, 3.0, 4.0])
        .y(&[9.0, 1.0, 10.0, 11.0])
        .color(&[1,2,3,4]);

    let plot = PlotBuilder::new(Backend::Plotly)
        .display(true)
        .add_layer(Plot::scatter(layer1))
        .add_layer(Plot::scatter(layer2))
        .build();

    plot.render();
}
