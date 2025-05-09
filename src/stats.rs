use std::{error::Error, path::Path};
use plotters::prelude::*;
use crate::converter::{from_csv, from_json, from_tsv, from_xml};
use crate::parser;
use crate::models::Record;
use crate::gc::gc_content;

pub fn run_stats<P: AsRef<Path>>(input: P) -> Result<(), Box<dyn Error>> {
    let path = input.as_ref();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Detect file type
    let records: Vec<Record> = match ext.as_str() {
        "json" => from_json(path.to_str().unwrap())
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?,
        "csv" => from_csv(path.to_str().unwrap())
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?,
        "tsv" => from_tsv(path.to_str().unwrap())
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?,
        "xml" => from_xml(path.to_str().unwrap())
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?,
        "fasta" | "fa" | "fna" => parser::parse_fasta(path)
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?,
        _ => return Err(Box::<dyn Error>::from(format!("Unsupported format: {}", ext))),
    };

    let n = records.len();
    if n == 0 {
        eprintln!("No sequences found in {:?}", path);
        return Ok(());
    }

    // Output prefix
    let prefix = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // Collect lengths and GC%
    let lengths: Vec<usize> = records.iter().map(|r| r.seq.len()).collect();
    let gc_percs: Vec<f64> = records.iter().map(|r| gc_content(&r.seq)).collect();

    // Convert to discrete bins for histograms
    let lengths_u32: Vec<u32> = lengths.iter().map(|&l| l as u32).collect();
    let gc_bins: Vec<u32> = gc_percs
        .iter()
        .map(|&g| {
            let b = g.round() as i32;
            if b < 0 { 0 }
            else if b > 100 { 100 }
            else { b as u32 }
        })
        .collect();

    // Summary statistics
    let sum: usize = lengths.iter().sum();
    let min = *lengths.iter().min().unwrap();
    let max = *lengths.iter().max().unwrap();
    let mean = sum as f64 / n as f64;
    let mut sorted = lengths.clone();
    sorted.sort_unstable();
    let median = if n % 2 == 0 {
        (sorted[n/2 - 1] + sorted[n/2]) as f64 / 2.0
    } else {
        sorted[n/2] as f64
    };
    let mean_gc = gc_percs.iter().sum::<f64>() / n as f64;

    println!("Sequences: {}", n);
    println!(
        "Length → min: {}  max: {}  mean: {:.2}  median: {:.2}",
        min, max, mean, median
    );
    println!("Mean GC%: {:.2}", mean_gc);

    // Length histogram
    {
        let out = format!("{}_length_hist.png", prefix);
        let root = BitMapBackend::new(&out, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let min_u = min as u32;
        let max_u = max as u32;
        let mut chart = ChartBuilder::on(&root)
            .caption("Sequence Length Distribution", ("sans-serif", 24))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(min_u..max_u, 0u32..n as u32)?;

        chart.configure_mesh()
            .x_desc("Sequence Length (nt)")
            .y_desc("Count")
            .draw()?;
        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.filled())
                .data(lengths_u32.iter().map(|&v| (v, 1)))
        )?;
        root.present()?;
    }

    // GC% histogram
    {
        let out = format!("{}_gc_hist.png", prefix);
        let root = BitMapBackend::new(&out, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("GC Content Distribution", ("sans-serif", 24))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0u32..100u32, 0u32..n as u32)?;

        chart.configure_mesh()
            .x_desc("GC Content (%)")
            .y_desc("Count")
            .draw()?;
        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.filled())
                .data(gc_bins.iter().map(|&v| (v, 1)))
        )?;
        root.present()?;
    }

    Ok(())
}