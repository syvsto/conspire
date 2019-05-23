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
    plot: &'a Plot,
}

impl Renderable for Plotly {
    fn render(&self, data: &[Plot], display: bool) -> Result<()> {
        use super::util::write_to_file;
        use std::path::Path;

        let html = Plotly::html(self.build_javascript(data)?);

        println!("{}", &html);

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
            Plot::Scatter(p) => stringify_2d_plot(f, "mode: 'markers', type: 'scatter'", p),
            Plot::Line(p) => stringify_2d_plot(f, "mode: 'lines', type: 'scatter'", p),
            Plot::Bar(p) => stringify_2d_plot(f, "type: 'bar'", p),
            Plot::Pie(p) => stringify_1d_plot(f, "type: 'pie'", p),
            Plot::HorizontalBar(p) => stringify_2d_plot(f, "orientation: 'h', type: 'bar'", p),
            Plot::SimpleHeatmap(p) => stringify_1d_matrix_plot(f, "type: 'heatmap'", p),
            Plot::Box(p) => stringify_1d_plot(f, "type: 'box', boxpoints: 'Outliers'", p),
        }
    }
}

fn stringify_2d_plot(f: &mut fmt::Formatter, plot_definition: &'static str, p: &Layer) -> fmt::Result {
    assert!(p.get_x().is_some() && p.get_y().is_some());

    let x = AttributePair::new("x", p.get_x());
    let y = AttributePair::new("y", p.get_y());
    let base = format!("{} {} {}", x, y, plot_definition);

    stringify_plot(base, &p)
}

fn stringify_1d_matrix_plot(f: &mut fmt::Formatter, plot_definition: &'static str, p: &Layer) -> fmt::Result {
    assert!(p.get_x().is_some());
    let x = AttributePair::new("z", p.get_x());
    let base = format!("{} {}", x, plot_definition);

    stringify_plot(base, &p)
}

fn stringify_1d_plot(f: &mut fmt::Formatter, plot_definition: &'static str, p: &Layer) -> fmt::Result {
    assert!(p.get_x().is_some());

    let x = AttributePair::new("x", p.get_x());
    let base = format!("{} {}", x, plot_definition);

    stringify_plot(base, &p)
}

fn stringify_plot(base: String, layer: &Layer) -> fmt::Result {
    let color = AttributePair::new("color", layer.get_color());
    let size = AttributePair::new("size", layer.get_size());
    let name = AttributePair::new("name", layer.get_name());
    let mut marker = String::new();

    if color.value.is_some() {
        marker = format!("{}", color);
    }

    if size.value.is_some() {
        marker = format!("{} {}", marker, size);
    }

    if name.value.is_none() {
        write!(f, "{}, marker: {{ {} }}", base, marker)
    } else {
        write!(f, "{}, marker: {{ {} }}, name: {}", base, marker, name)
    }
}

fn dimension_to_text(data: &Option<DataType>) -> Option<String> {
    if let Some(d) = data {
        match d {
            DataType::Quantitative(v) => Some(format!("{}", stringify_data_vec(v))),
            DataType::Categorical(v) => Some(format!("{}", stringify_data_vec(v))),
        }
    } else {
        None
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
    value: Option<String>,
}

impl AttributePair {
    fn new(key: &'static str, value: &Option<DataType>) -> Self {
        Self {key: key.to_string(), value: dimension_to_text(value)}
    }
}

impl fmt::Display for AttributePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = &self.value {
            write!(f, "{}: {},", self.key, v)
        } else {
            write!(f, "")
        }
    }
}
