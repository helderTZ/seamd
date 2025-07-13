use std::{io::Write, time::{SystemTime, UNIX_EPOCH}};
use clap::Parser;

mod carve;
mod seam;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input_file: String,

    /// Output file
    #[arg(short,long)]
    output_file: String,

    /// Number of seams to carve out
    #[arg(short, long)]
    nr_seams: i32,

    /// Flag to generate highlighted image with carved out seams
    #[arg(short, long, action)]
    seam_highlight: bool,
}

fn main() {
    let args = Args::parse();
    let highlight_file = format!("highlight_{}", args.output_file);

    println!("Input file: {}", args.input_file);
    println!("Output file: {}", args.output_file);
    if args.seam_highlight {
        println!("Highlighted file: {}", highlight_file);
    }
    println!("Number of seams to carve: {}", args.nr_seams);
    println!("Highlight enabled: {}", args.seam_highlight);

    let mut img_buf = image::ImageReader::open(args.input_file)
        .unwrap()
        .decode()
        .unwrap()
        .as_rgb8()
        .unwrap().clone();

    let mut highlighted_img = img_buf.clone();

    let mut seam_carving_timings = Vec::with_capacity(args.nr_seams as usize);
    let mut seam_highlight_timings = Vec::with_capacity(args.nr_seams as usize);

    for s in 0..args.nr_seams {
        print!("\rCarving seam {}/{} ...", s+1, args.nr_seams);
        let _ = std::io::stdout().flush();

        let start_seam_carving = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seams = seam::get_seams(&img_buf);
        let min_seam = seam::get_min_seam(&seams);
        let carved_img = carve::carve_seam(&img_buf, min_seam);
        let end_seam_carving = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        seam_carving_timings.push(end_seam_carving - start_seam_carving);

        img_buf = carved_img.clone();

        if args.seam_highlight {
            let start_seam_highlight = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            carve::highlight_seam(&mut highlighted_img, min_seam, s);
            let end_seam_highlight = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            seam_highlight_timings.push(end_seam_highlight - start_seam_highlight);
        }

        if s == args.nr_seams-1 {
            carved_img.save(args.output_file.as_str())
            .expect("could not save carved image");

            if args.seam_highlight {
                highlighted_img.save(&highlight_file)
                    .expect("could not save highlighted image");
            }
        }
    }
    println!();

    let seam_carving_avg = seam_carving_timings.iter().map(|d| d.as_millis()).sum::<u128>() as f64 / seam_carving_timings.len() as f64;
    let seam_carving_max = seam_carving_timings.iter().map(|d| d.as_micros()).max().unwrap_or(0) as f64 * 1e-3;
    let seam_carving_min = seam_carving_timings.iter().map(|d| d.as_micros()).min().unwrap_or(0) as f64 * 1e-3;
    println!("Average seam carving time: {}ms", seam_carving_avg);
    println!("Maximum seam carving time: {}ms", seam_carving_max);
    println!("Minimum seam carving time: {}ms", seam_carving_min);
    
    if args.seam_highlight {
        let seam_highlight_avg = seam_highlight_timings.iter().map(|d| d.as_millis()).sum::<u128>() as f64 / seam_highlight_timings.len() as f64;
        let seam_highlight_max = seam_highlight_timings.iter().map(|d| d.as_micros()).max().unwrap_or(0) as f64 * 1e-3;
        let seam_highlight_min = seam_highlight_timings.iter().map(|d| d.as_micros()).min().unwrap_or(0) as f64 * 1e-3;
        println!("Average seam highlight time: {}ms", seam_highlight_avg);
        println!("Maximum seam highlight time: {}ms", seam_highlight_max);
        println!("Minimum seam highlight time: {}ms", seam_highlight_min);
    }
}
