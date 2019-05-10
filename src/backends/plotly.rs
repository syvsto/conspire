use super::Renderable;
use crate::channels::{Color, PositionX, PositionY, Size};
use crate::data::{Nominal, Ordinal, Quantitative};
use crate::error::DimensionError;
use crate::Plot;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Plotly {}

impl Plotly {
    fn build_javascript<T, U, V>(&self, data: &[Plot<T, U, V>]) -> Result<String>
    where
        T: Quantitative,
        U: Nominal,
        V: Ordinal,
    {
        let traces: Vec<String> = data
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let t = PlotlyPlot { plot: d };
                Plotly::trace(i, t)
            })
            .collect();

        let names: Vec<String> = data
            .iter()
            .enumerate()
            .map(|(i, _d)| Plotly::name(i))
            .collect();

        Ok(format!(
            "{}\nlet data = [{}]; Plotly.newPlot('myDiv', data);",
            traces.concat(),
            names.concat()
        ))
    }

    fn html(plot_script: String) -> String {
        format!(
            r#"<head>
    <!-- Plotly.js -->
    <script src="https://cdn.plot.ly/plotly-latest.min.js"></script>
    </head>
    <body>
    <div id="myDiv"></div>
    <script>
    {}
    </script>
    </body>
    "#,
            plot_script
        )
    }

    fn trace<T, U, V>(idx: usize, t: PlotlyPlot<T, U, V>) -> String
    where
        T: Quantitative,
        U: Nominal,
    V: Ordinal,
    {
        format!("let trace{} = {{ {} }};\n", idx, t)
    }

    fn name(idx: usize) -> String {
        format!("trace{},", idx)
    }
}

struct PlotlyPlot<'a, T, U, V>
where
    T: Quantitative,
    U: Nominal,
    V: Ordinal,
{
    plot: &'a Plot<T, U, V>,
}

impl Renderable for Plotly {
    fn render<T, U, V>(&self, data: &[Plot<T, U, V>], display: bool) -> Result<()>
    where
        T: Quantitative,
        U: Nominal,
        V: Ordinal,
    {
        use super::util::write_to_file;
        use std::path::Path;

        let html = Plotly::html(self.build_javascript(data)?);

        write_to_file(&Path::new("render.html"), &html);

        if display {
            use std::process::Command;
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(&["/C", "start render.html"])
                    .output()
                    .expect("failed to execute process");
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("open render.html")
                    .output()
                    .expect("failed to execute process");
            }
        }
        Ok(())
    }
}

impl<'a, T, U, V> std::fmt::Display for PlotlyPlot<'a, T, U, V>
where
    T: Quantitative,
    U: Nominal,
    V: Ordinal,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.plot {
            Plot::Scatter(p) => write!(
                f,
                "x: {:?}, y: {:?}, mode: 'markers', type: 'scatter',",
                p.get_x().expect("No X dimenison"),
                p.get_y().expect("No Y dimension")
            ),
            Plot::Line(p) => write!(
                f,
                "x: {:?}, y: {:?}, mode: 'lines', type: 'scatter',",
                p.get_x().expect("No X dimenison"),
                p.get_y().expect("No Y dimension")
            ),
            Plot::Bar(p) => write!(
                f,
                "x: {:?}, y: {:?}, type: 'bar',",
                p.get_x().expect("No X dimenison"),
                p.get_y().expect("No Y dimension")
            ),
            Plot::Pie(p) => write!(f, "x: {:?}, type: pie,", p.get_x()),
            Plot::HorizontalBar(p) => write!(
                f,
                "x: {:?}, y: {:?}, orientation: 'h', type: 'bar',",
                p.get_x().expect("No X dimenison"),
                p.get_y().expect("No Y dimension")
            ),
        }
    }
}
