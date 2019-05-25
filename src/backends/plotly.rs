use super::Renderable;
use crate::data::DataType;
use crate::Plot;
use crate::Layer;

use std::error;
use std::fmt;
use core::fmt::Debug;

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
            Plot::Scatter {x, y, color, size, name} => {
                let type_str = "mode: 'markers', type: 'scatter'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, size);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            },
            Plot::Line {x, y, color, size, name} => {
                let type_str = "mode: 'lines', type: 'scatter'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, size);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            },
            Plot::Bar {x, y, color, name} => {
                let type_str = "type: 'bar'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            },
            Plot::Pie {x, color, name} => {
                let type_str = "type: 'pie'";
                let base = type_to_string_1d(type_str, x);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            },
            Plot::HorizontalBar {x, y, color, name} => { 
                let type_str = "type: 'bar'";
                let base = type_to_string_2d(type_str, x, y);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            }
            Plot::SimpleHeatmap {..} => unimplemented!(),
            Plot::Box {x, color, name} => {
                let type_str = "type: 'box', boxpoints: 'Outliers'";
                let base = type_to_string_1d(type_str, x);
                let markers = markers_to_string(color, &None);
                let name = name_to_string(name);
                write!(f, "{} {} {}", base, markers, name)
            },
        }
    }
}

fn type_to_string_2d(plot_definition: &'static str, x: &DataType, y: &DataType) -> String {
    let x = AttributePair::new("x", x);
    let y = AttributePair::new("y", y);
    format!("{} {} {}", x, y, plot_definition)
}

fn type_to_string_1d(plot_definition: &'static str, x: &DataType) -> String {
    let x = AttributePair::new("x", x);
    format!("{} {}", x, plot_definition)
}

fn markers_to_string(color: &Option<DataType>, size: &Option<DataType>) -> String {
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

fn dimension_to_text(data: &DataType) -> String {
        match data {
            DataType::Quantitative(v) => format!("{}", stringify_data_vec(v)),
            DataType::Categorical(v) => format!("{}", stringify_data_vec(v)),
        }
}

fn stringify_data_vec<T: Debug>(v: &[T]) -> String {
    if v.len() > 1 {
        format!("{:?}", v)
    } else {
        format!("{:?}", v[0])
    }
}

struct AttributePair {
    key: String,
    value: String,
}

impl AttributePair {
    fn new(key: &'static str, value: &DataType) -> Self {
        Self {key: key.to_string(), value: dimension_to_text(value)}
    }
}

impl fmt::Display for AttributePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {},", self.key, self.value)
    }
}
