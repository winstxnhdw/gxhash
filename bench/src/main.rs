use std::ops::Div;
use std::ops::Mul;
use std::path::Path;

use anyhow::Result;
use polars::prelude::*;
use resvg::tiny_skia;
use resvg::usvg;
use resvg::usvg::fontdb::Database;
use tagu::build::raw;
use tagu::elem::Elem;

fn write_png_from_svg(data: &str, path: &Path, fontdb: &Database) -> Result<()> {
    let options = usvg::Options {
        fontdb: fontdb.clone().into(),
        ..Default::default()
    };

    let tree = usvg::Tree::from_str(data, &options)?;
    let size = tree.size();
    let width = 1600f32;
    let scale: f32 = width / size.width();
    let target_width = width.round() as u32;
    let target_height = (size.height() * scale).round() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height).expect("Failed to allocate pixmap");
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pixmap_mut);
    std::fs::create_dir_all(path.parent().unwrap())?;
    pixmap.save_png(path)?;

    Ok(())
}

fn load_fonts() -> Database {
    let mut fontdb = Database::new();

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
        .append(raw(".poloto0.poloto_fill{fill: #6340AC;}"))
        .append(raw(".poloto_background{fill: white;}"));

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
            .expect("Should be able to collect the filtered dataframe");

        let throughputs = data["throughput"]
            .f64()
            .expect("throughput column should be f64")
            .iter()
            .flatten();

        let points = data["payload_size"]
            .f64()
            .expect("payload_size column should be f64")
            .iter()
            .flatten()
            .map(f64::log10)
            .zip(throughputs);

        poloto::build::plot(name).line(points)
    });

    let tick_positions: Vec<f64> = (4..14).map(|i| f64::log10(4.0_f64.powi(i) / 1_048_576.0)).collect();

    let x_min = tick_positions[0];
    let x_max = tick_positions[tick_positions.len() - 1];

    let theme = poloto::render::Theme::light()
        .append(raw(".poloto_background{fill: white;}"))
        .append(raw(".poloto_text.poloto_legend{font-size:8px;}"));

    let viewbox = [1008.0, 540.0];
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
            poloto::ticks::from_iter(tick_positions).with_tick_fmt(|val: &f64| {
                let kilobyte = (1 << 10) as f64;
                let megabyte = (1 << 20) as f64;
                let bytes = 10.0_f64.powf(*val) * megabyte;

                if bytes >= megabyte {
                    format!("{:.0} MiB", bytes / megabyte)
                } else if bytes >= kilobyte {
                    format!("{:.0} KiB", bytes / kilobyte)
                } else {
                    format!("{:.0} B", bytes)
                }
            })
        })
        .build_and_label((heading, "Payload Size", "Throughput (MiB/s)"))
        .append_to(header)
        .render_string()?;

    Ok(svg)
}

fn main() -> Result<()> {
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
        .filter(col("payload_size").gt_eq(256))
        .with_column(col("payload_size").cast(DataType::Float64).div(lit(1 << 20)))
        .filter(col("payload_size").lt_eq(64));

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
