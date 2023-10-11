use std::env;
use std::thread;
use std::io::Write;

use image::ImageBuffer;
use::image::Rgb;

mod carve;
mod seam;

fn main() {
    let input_file = env::args().nth(1).expect("Expected input file");
    let output_file = env::args().nth(2).expect("Expected output file"); 
    let nr_seams = match env::args().nth(3) {
        Some(nr_seams) => nr_seams.parse::<i32>().unwrap(),
        None => 1,
    };
    let highlight_carved_seams = match env::args().nth(4) {
        Some(highlight) => if highlight == "--highlight" {
            true
        } else {
            false
        },
        None => false,
    };

    println!("Original file: {}", input_file);
    println!("Output file: {}", output_file);
    println!("Number of seams to carve: {}", nr_seams);

    let mut img_buf = image::io::Reader::open(input_file)
        .unwrap()
        .decode()
        .unwrap()
        .as_rgb8()
        .unwrap().clone();

    let mut highlighted_img = img_buf.clone();

    for s in 0..nr_seams {
        print!("\rCarving seam {}/{} ...", s+1, nr_seams);
        let _ = std::io::stdout().flush();

        let seams = seam::get_seams(&img_buf);
        let min_seam = seam::get_min_seam(&seams);
        let carved_img = carve::carve_seam(&img_buf, min_seam);
        img_buf = carved_img.clone();

        if highlight_carved_seams {
            highlighted_img = carve::highlight_seam(&highlighted_img, min_seam);
        }

        if s == nr_seams-1 {
            carved_img.save(output_file.as_str())
            .expect("could not save carved image");

            if highlight_carved_seams {
                highlighted_img.save(format!("highlight_{}", output_file))
                    .expect("could not save highlighted image");
            }
        }
    }

    println!();
}
