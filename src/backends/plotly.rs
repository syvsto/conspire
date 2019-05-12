use super::Renderable;
use crate::data::DataType;
use crate::Plot;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Plotly {}

impl Plotly {
    fn build_javascript(&self, data: &[Plot]) -> Result<String> {
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

    fn trace(idx: usize, t: PlotlyPlot) -> String {
        format!("let trace{} = {{ {} }};\n", idx, t)
    }

    fn name(idx: usize) -> String {
        format!("trace{},", idx)
    }
}

struct PlotlyPlot<'a> {
    plot: &'a Plot,
}

impl Renderable for Plotly {
    fn render(&self, data: &[Plot], display: bool) -> Result<()> {
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

impl<'a> std::fmt::Display for PlotlyPlot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.plot {
            Plot::Scatter(p) => write!(
                f,
                "x: {}, y: {}, mode: 'markers', type: 'scatter',",
                dimension_to_text(p.get_x()).expect("No X dimension"),
                dimension_to_text(p.get_y()).expect("No Y dimension")
            ),
            Plot::Line(p) => write!(
                f,
                "x: {}, y: {}, mode: 'lines', type: 'scatter',",
                dimension_to_text(p.get_x()).expect("No X dimension"),
                dimension_to_text(p.get_y()).expect("No Y dimension")
            ),
            Plot::Bar(p) => write!(
                f,
                "x: {}, y: {}, type: 'bar',",
                dimension_to_text(p.get_x()).expect("No X dimension"),
                dimension_to_text(p.get_y()).expect("No Y dimension")
            ),
            Plot::Pie(p) => write!(f, "x: {:?}, type: pie,", dimension_to_text(p.get_x()).expect("No X Dimension")),
            Plot::HorizontalBar(p) => write!(
                f,
                "x: {}, y: {}, orientation: 'h', type: 'bar',",
                dimension_to_text(p.get_x()).expect("No X dimension"),
                dimension_to_text(p.get_y()).expect("No Y dimension")
            ),
        }
    }
}

fn dimension_to_text(data: &Option<DataType>) -> Option<String> {
    if let Some(d) = data {
        match d {
            DataType::Quantitative(v) => Some(format!("{:?}", v)),
            DataType::Categorical(v) => Some(format!("{:?}", v)),
        }
    } else {
        None
    }
}
