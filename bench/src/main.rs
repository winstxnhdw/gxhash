use std::fmt;
use std::fs::write;
use std::io;
use std::ops::Div;
use std::ops::Mul;

use polars::prelude::*;
use tagu::build::raw;
use tagu::elem::Elem;

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Polars(PolarsError),
    Io(io::Error),
    Fmt(fmt::Error),
}

impl From<PolarsError> for Error {
    fn from(e: PolarsError) -> Self {
        Error::Polars(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

fn generate_benchmark_bar_plot(heading: &str, lazyframe: LazyFrame) -> Result<String, Error> {
    let dataframe = lazyframe.select([col("name"), col("throughput")]).collect()?;
    let data = dataframe["throughput"]
        .f64()?
        .iter()
        .flatten()
        .zip(dataframe["name"].str()?.iter().flatten());

    let theme = poloto::render::Theme::light()
        .append(raw(".poloto0.poloto_fill{fill: #6340AC;}"))
        .append(raw(".poloto_background{fill: white;}"));

    let svg = poloto::build::bar::gen_simple("", data, [0.0])
        .label((heading, "Throughput (MiB/s)", ""))
        .append_to(poloto::header().with_dim([2800.0, 1500.0]).append(theme))
        .render_string()?;

    Ok(svg)
}

fn generate_benchmark_line_plot(heading: &str, lazyframe: LazyFrame) -> Result<String, Error> {
    let dataframe = lazyframe
        .select([col("name"), col("payload_size"), col("throughput")])
        .sort(["payload_size"], SortMultipleOptions::default())
        .collect()?;

    let names = dataframe["name"].str()?.unique()?;
    let plots = names.iter().flatten().map(|name| {
        let data = dataframe
            .clone()
            .lazy()
            .filter(col("name").eq(lit(name)))
            .collect()
            .expect("Should be able to collect the filtered dataframe");

        let throughputs = data["throughput"]
            .f64()
            .expect("throughput column should be f64")
            .iter()
            .flatten();

        let points = data["payload_size"]
            .i64()
            .expect("payload_size column should be i64")
            .iter()
            .flatten()
            .map(|size| f64::log10(size as f64))
            .zip(throughputs);

        poloto::build::plot(name).line(points)
    });

    let tick_positions: Vec<_> = dataframe["payload_size"]
        .i64()?
        .unique()?
        .iter()
        .flatten()
        .map(|size| f64::log10(size as f64))
        .collect();

    let x_min = tick_positions.iter().copied().reduce(f64::min).unwrap_or(0.0);
    let x_max = tick_positions.iter().copied().reduce(f64::max).unwrap_or(4000.0);

    let colours = [
        "#253b77", "#E6194B", "#3CB44B", "#FFE119", "#4363D8", "#F58231", "#79059c", "#42D4F4", "#F032E6", "#BFEF45",
        "#FABED4", "#469990", "#c28dff", "#9A6324", "#800000", "#AAFFC3", "#808000", "#FFD8B1", "#e30081", "#054f03",
    ];

    let colour_css = colours.iter().enumerate().map(|(i, colour)| {
        format!(".poloto{i}.poloto_stroke{{stroke:{colour};}}.poloto{i}.poloto_fill{{fill:{colour};}}")
    });

    let theme = poloto::render::Theme::light()
        .append(raw(".poloto_background{fill: white;}"))
        .append(raw(".poloto_text.poloto_legend{font-size:8px;fill:black;}"))
        .append(raw(colour_css.collect::<String>()));

    let viewbox = [1200.0, 800.0];
    let data = poloto::plots!(poloto::build::markers::<_, _, (f64, f64)>([x_min, x_max], [0.0]), plots);
    let header = poloto::header()
        .with_dim([2800.0, 1500.0])
        .with_viewbox(viewbox)
        .append(theme);

    let svg = poloto::frame()
        .with_viewbox(viewbox)
        .build()
        .data(data)
        .map_xticks(|_| {
            poloto::ticks::from_iter(tick_positions).with_tick_fmt(|value: &f64| {
                let kilobyte = (1 << 10) as f64;
                let megabyte = (1 << 20) as f64;
                let gigabyte = (1 << 30) as f64;

                match 10.0_f64.powf(*value) {
                    bytes if bytes >= gigabyte => format!("{:.0} GiB", bytes / gigabyte),
                    bytes if bytes >= megabyte => format!("{:.0} MiB", bytes / megabyte),
                    bytes if bytes >= kilobyte => format!("{:.0} KiB", bytes / kilobyte),
                    bytes => format!("{:.0} B", bytes),
                }
            })
        })
        .build_and_label((heading, "Payload Size", "Throughput (MiB/s)"))
        .append_to(header)
        .render_string()?;

    Ok(svg)
}

fn main() -> Result<(), Error> {
    let path = "benchmarks.parquet";

    let duration_to_throughput = col("payload_size")
        .mul(lit(1e9))
        .div(col("hot_duration").mul(lit(1 << 20)));

    let dataframe = LazyFrame::scan_parquet(PlRefPath::new(path), ScanArgsParquet::default())?
        .with_column(duration_to_throughput.alias("throughput"))
        .sort(["throughput"], SortMultipleOptions::default());

    let throughtput_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(1))
        .filter(col("payload_size").eq(64 << 10));

    let throughtput_batched_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(16))
        .filter(col("length").eq(128))
        .filter(col("payload_size").gt_eq(256));

    let throughput_32bit_dataframe = throughtput_dataframe.clone().filter(col("length").eq(32));
    let throughput_64bit_dataframe = throughtput_dataframe.clone().filter(col("length").eq(64));
    let throughput_128bit_dataframe = throughtput_dataframe.clone().filter(col("length").eq(128));

    let throughput_32bit_svg =
        generate_benchmark_bar_plot("32-bit hash with 64KiB payload", throughput_32bit_dataframe.clone())?;
    let throughput_64bit_svg =
        generate_benchmark_bar_plot("64-bit hash with 64KiB payload", throughput_64bit_dataframe.clone())?;
    let throughput_128bit_svg =
        generate_benchmark_bar_plot("128-bit hash with 64KiB payload", throughput_128bit_dataframe.clone())?;
    let throughput_batched_svg = generate_benchmark_line_plot(
        "128-bit hash with batch size of 16",
        throughtput_batched_dataframe.clone(),
    )?;

    write("throughput-32bit.svg", &throughput_32bit_svg)?;
    write("throughput-64bit.svg", &throughput_64bit_svg)?;
    write("throughput-128bit.svg", &throughput_128bit_svg)?;
    write("throughput-batched.svg", &throughput_batched_svg)?;

    Ok(())
}
