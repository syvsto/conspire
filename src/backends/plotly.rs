use super::common::{stringify_data_vec, AttributePair};
use super::Renderable;
use crate::data::{MatrixData, VectorData};
use crate::Plot;

use std::error;
use std::fmt;

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
    plot: &'a Plot<'a>,
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

impl<'a> fmt::Display for PlotlyPlot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.plot {
            Plot::Scatter {
                x,
                y,
                color,
                size,
                name,
            } => {
                let type_str = "mode: 'markers', type: 'scatter'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, size);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::Line {
                x,
                y,
                color,
                size,
                name,
            } => {
                let type_str = "mode: 'lines', type: 'scatter'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, size);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::Bar { x, y, color, name } => {
                let type_str = "type: 'bar'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::Pie { x, color, name } => {
                let type_str = "type: 'pie'";
                let base = type_to_string_1d(type_str, x, Some("values"));
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::HorizontalBar { x, y, color, name } => {
                let type_str = "type: 'bar', orientation: 'h'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::SimpleHeatmap { z, color, name } => {
                let type_str = "type: 'heatmap'";
                let base = type_to_string_1d(type_str, z, Some("z"));
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::Box { x, color, name } => {
                let type_str = "type: 'box', boxpoints: 'Outliers'";
                let base = type_to_string_1d(type_str, x, None);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
        }
    }
}

fn type_to_string_2d<T: fmt::Display>(plot_definition: &'static str, x: &T, y: &T) -> String {
    let x = AttributePair::new("x", x);
    let y = AttributePair::new("y", y);
    format!("{} {} {}", x, y, plot_definition)
}

fn type_to_string_1d<T: fmt::Display>(
    plot_definition: &'static str,
    data: &T,
    attr_name: Option<&'static str>,
) -> String {
    if let Some(name) = attr_name {
        let data = AttributePair::new(name, data);
        format!("{} {}", data, plot_definition)
    } else {
        let x = AttributePair::new("x", data);
        format!("{} {}", x, plot_definition)
    }
}

fn markers_to_string<T: fmt::Display>(color: &Option<T>, size: &Option<T>) -> String {
    let mut marker = String::new();

    if let Some(c) = color {
        let color = AttributePair::new("color", c);
        marker = format!("{}", color);
    }

    if let Some(s) = size {
        let size = AttributePair::new("size", s);
        marker = format!("{} {}", marker, size);
    }

    format!("{},", marker)
}

fn name_to_string(name: &Option<String>) -> String {
    if let Some(n) = name {
        format!("name: \'{}\'", n)
    } else {
        String::from("")
    }
}

impl fmt::Display for VectorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VectorData::Quantitative(v) => write!(f, "{}", stringify_data_vec(v)),
            VectorData::Categorical(v) => write!(f, "{}", stringify_data_vec(v)),
        }
    }
}

impl fmt::Display for MatrixData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatrixData::Quantitative(v) => write!(f, "{}", stringify_data_vec(v)),
        }
    }
}

impl fmt::Display for AttributePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {},", self.key, self.value)
    }
}
