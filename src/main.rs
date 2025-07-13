use std::io::Write;
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

    println!("Original file: {}", args.input_file);
    println!("Output file: {}", args.output_file);
    println!("Number of seams to carve: {}", args.nr_seams);
    println!("Highlight enabled: {}", args.seam_highlight);

    let mut img_buf = image::ImageReader::open(args.input_file)
        .unwrap()
        .decode()
        .unwrap()
        .as_rgb8()
        .unwrap().clone();

    let mut highlighted_img = img_buf.clone();

    for s in 0..args.nr_seams {
        print!("\rCarving seam {}/{} ...", s+1, args.nr_seams);
        let _ = std::io::stdout().flush();

        let seams = seam::get_seams(&img_buf);
        let min_seam = seam::get_min_seam(&seams);
        let carved_img = carve::carve_seam(&img_buf, min_seam);
        img_buf = carved_img.clone();

        if args.seam_highlight {
            highlighted_img = carve::highlight_seam(&highlighted_img, min_seam);
        }

        if s == args.nr_seams-1 {
            carved_img.save(args.output_file.as_str())
            .expect("could not save carved image");

            if args.seam_highlight {
                highlighted_img.save(format!("highlight_{}", args.output_file))
                    .expect("could not save highlighted image");
            }
        }
    }

    println!();
}
