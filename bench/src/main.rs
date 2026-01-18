use std::ops::{Div, Mul};
use std::path::Path;

use anyhow::{Result, anyhow};
use polars::prelude::*;
use resvg::usvg::fontdb;
use tagu::prelude::*;

fn write_png_from_svg(data: &str, path: &Path, fontdb: &fontdb::Database) -> Result<()> {
    let options = resvg::usvg::Options {
        fontdb: fontdb.clone().into(),
        ..Default::default()
    };
    let tree = resvg::usvg::Tree::from_str(data, &options)?;
    let size = tree.size();
    let width = 1600f32;
    let scale: f32 = width / size.width();
    let target_width = width.round() as u32;
    let target_height = (size.height() * scale).round() as u32;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(target_width, target_height)
        .ok_or_else(|| anyhow!("failed to allocate pixmap"))?;
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pixmap_mut);
    fs_err::create_dir_all(path.parent().unwrap())?;
    pixmap.save_png(path)?;
    Ok(())
}

fn load_fonts() -> fontdb::Database {
    let mut fontdb = fontdb::Database::new();

    fontdb.load_system_fonts();
    fontdb.set_serif_family("Times New Roman");
    fontdb.set_sans_serif_family("Arial");
    fontdb.set_cursive_family("Comic Sans MS");
    fontdb.set_fantasy_family("Impact");
    fontdb.set_monospace_family("Courier New");

    fontdb
}

fn generate_benchmark_bar_plot(heading: &str, lazyframe: LazyFrame) -> Result<String> {
    let dataframe = lazyframe.select([col("name"), col("throughput")]).collect()?;
    let data = dataframe["throughput"]
        .f64()?
        .iter()
        .flatten()
        .zip(dataframe["name"].str()?.iter().flatten());

    let theme = poloto::render::Theme::light()
        .append(tagu::build::raw(".poloto0.poloto_fill{fill: #6340AC;}"))
        .append(tagu::build::raw(".poloto_background{fill: white;}"));

    let svg = poloto::build::bar::gen_simple("", data, [0.0])
        .label((heading, "Throughput (MiB/s)", ""))
        .append_to(poloto::header().with_dim([2800.0, 1500.0]).append(theme))
        .render_string()?;

    Ok(svg)
}

fn generate_benchmark_line_plot(heading: &str, lazyframe: LazyFrame) -> Result<String> {
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
            .unwrap();
        let data = data["payload_size"]
            .f64()
            .unwrap()
            .iter()
            .flatten()
            .zip(data["throughput"].f64().unwrap().iter().flatten());

        poloto::build::plot(name).line(data)
    });

    let theme = poloto::render::Theme::light().append(tagu::build::raw(".poloto_background{fill: white;}"));
    let svg = poloto::frame_build()
        .data(poloto::plots!(poloto::build::origin(), plots))
        .build_and_label((heading, "Payload Size (MiB)", "Throughput (MiB/s)"))
        .append_to(poloto::header().with_dim([2800.0, 1500.0]).append(theme))
        .render_string()?;

    Ok(svg)
}

fn main() -> Result<()> {
    let path = "benchmarks.parquet";

    let duration_to_throughput = col("payload_size")
        .mul(lit(1e9))
        .div(col("hot_duration").mul(lit(1 << 20)));

    let dataframe = LazyFrame::scan_parquet(PlPath::new(path), ScanArgsParquet::default())?
        .with_column(duration_to_throughput.alias("throughput"))
        .sort(["throughput"], SortMultipleOptions::default());

    let throughtput_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(1))
        .filter(col("payload_size").eq(64 * 1024));

    let throughtput_batched_dataframe = dataframe
        .clone()
        .filter(col("batch_size").eq(16))
        .filter(col("length").eq(128))
        .with_column(col("payload_size").cast(DataType::Float64).div(lit(1 << 20)))
        .filter(col("payload_size").lt_eq(4));

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

    let fonts = load_fonts();
    write_png_from_svg(&throughput_32bit_svg, Path::new("throughput-32bit.png"), &fonts)?;
    write_png_from_svg(&throughput_64bit_svg, Path::new("throughput-64bit.png"), &fonts)?;
    write_png_from_svg(&throughput_128bit_svg, Path::new("throughput-128bit.png"), &fonts)?;
    write_png_from_svg(&throughput_batched_svg, Path::new("throughput-batched.png"), &fonts)
}
