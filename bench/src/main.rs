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

fn generate_throughput_benchmark_bar_plot(heading: &str, lazyframe: LazyFrame) -> Result<String, Error> {
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

fn generate_latency_benchmark_bar_plot(heading: &str, lazyframe: LazyFrame) -> Result<String, Error> {
    let dataframe = lazyframe.select([col("name"), col("hot_duration")]).collect()?;
    let data = dataframe["hot_duration"]
        .f64()?
        .iter()
        .flatten()
        .zip(dataframe["name"].str()?.iter().flatten());

    let theme = poloto::render::Theme::light()
        .append(raw(".poloto0.poloto_fill{fill: #6340AC;}"))
        .append(raw(".poloto_background{fill: white;}"));

    let svg = poloto::build::bar::gen_simple("", data, [0.0])
        .label((heading, "Latency (μs)", ""))
        .append_to(poloto::header().with_dim([2800.0, 1500.0]).append(theme))
        .render_string()?;

    Ok(svg)
}

fn generate_throughput_benchmark_line_plot(heading: &str, lazyframe: LazyFrame) -> Result<String, Error> {
    let dataframe = lazyframe
        .with_column(col("payload_size").cast(DataType::Float64).log(lit(10.0)))
        .select([col("name"), col("payload_size"), col("throughput")])
        .sort(["payload_size"], SortMultipleOptions::default())
        .collect()?
        .partition_by(["name"], true)?;

    let payload_sizes = dataframe[0]["payload_size"].f64()?.unique()?;
    let x_min = payload_sizes.first().unwrap_or(0.0);
    let x_max = payload_sizes.last().unwrap_or(4000.0);
    let plots = dataframe.into_iter().map(|group| -> Result<_, Error> {
        let points = group["payload_size"]
            .f64()?
            .iter()
            .flatten()
            .zip(group["throughput"].f64()?.iter().flatten());

        Ok(poloto::build::plot(group["name"].str()?.first().unwrap_or("unknown").to_string()).line(points))
    });

    let data = poloto::plots!(poloto::build::markers::<_, _, (f64, f64)>([x_min, x_max], [0.0]), plots);
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
    let header = poloto::header()
        .with_dim([2800.0, 1500.0])
        .with_viewbox(viewbox)
        .append(theme);

    let svg = poloto::frame()
        .with_viewbox(viewbox)
        .build()
        .data(data)
        .map_xticks(|_| {
            poloto::ticks::from_iter(payload_sizes.iter().flatten()).with_tick_fmt(|value: &f64| {
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
    let postfix = std::env::args().nth(1).map(|s| format!("-{s}")).unwrap_or_default();

    let dataframe = LazyFrame::scan_parquet(PlRefPath::new(path), ScanArgsParquet::default())?.with_column(
        col("payload_size")
            .mul(col("batch_size"))
            .mul(lit(1e9))
            .div(col("hot_duration").mul(lit(1 << 20)))
            .alias("throughput"),
    );

    let throughput_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(1))
        .filter(col("payload_size").eq(64 << 10))
        .sort(["throughput"], SortMultipleOptions::default());

    let throughput_batched_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(16))
        .filter(col("payload_size").gt_eq(256))
        .filter(col("length").eq(128));

    let latency_dataframe = dataframe
        .filter(col("batch_size").eq(1))
        .filter(col("payload_size").eq(4))
        .with_column(col("hot_duration").div(lit(1e3)))
        .sort(
            ["hot_duration"],
            SortMultipleOptions::default().with_order_descending(true),
        );

    let throughput_32bit_dataframe = throughput_dataframe.clone().filter(col("length").eq(32));
    let throughput_64bit_dataframe = throughput_dataframe.clone().filter(col("length").eq(64));
    let throughput_128bit_dataframe = throughput_dataframe.filter(col("length").eq(128));

    let latency_32bit_dataframe = latency_dataframe.clone().filter(col("length").eq(32));
    let latency_64bit_dataframe = latency_dataframe.clone().filter(col("length").eq(64));
    let latency_128bit_dataframe = latency_dataframe.filter(col("length").eq(128));

    let latency_32bit_svg =
        generate_latency_benchmark_bar_plot("32-bit hash with 4-byte payload", latency_32bit_dataframe)?;
    let latency_64bit_svg =
        generate_latency_benchmark_bar_plot("64-bit hash with 4-byte payload", latency_64bit_dataframe)?;
    let latency_128bit_svg =
        generate_latency_benchmark_bar_plot("128-bit hash with 4-byte payload", latency_128bit_dataframe)?;

    let throughput_32bit_svg =
        generate_throughput_benchmark_bar_plot("32-bit hash with 64KiB payload", throughput_32bit_dataframe)?;
    let throughput_64bit_svg =
        generate_throughput_benchmark_bar_plot("64-bit hash with 64KiB payload", throughput_64bit_dataframe)?;
    let throughput_128bit_svg =
        generate_throughput_benchmark_bar_plot("128-bit hash with 64KiB payload", throughput_128bit_dataframe)?;
    let throughput_batched_svg =
        generate_throughput_benchmark_line_plot("128-bit hash with batch size of 16", throughput_batched_dataframe)?;

    write(format!("latency-32bit{postfix}.svg"), &latency_32bit_svg)?;
    write(format!("latency-64bit{postfix}.svg"), &latency_64bit_svg)?;
    write(format!("latency-128bit{postfix}.svg"), &latency_128bit_svg)?;
    write(format!("throughput-32bit{postfix}.svg"), &throughput_32bit_svg)?;
    write(format!("throughput-64bit{postfix}.svg"), &throughput_64bit_svg)?;
    write(format!("throughput-128bit{postfix}.svg"), &throughput_128bit_svg)?;
    write(format!("throughput-batched{postfix}.svg"), &throughput_batched_svg)?;

    Ok(())
}
