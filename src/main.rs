use std::env;

mod carve;
mod seam;

fn main() {
    let input_file = env::args().nth(1).expect("Expected input file");
    let output_file = env::args().nth(2).expect("Expected output file");
    let nr_seams = match env::args().nth(3) {
        Some(nr_seams) => nr_seams.parse::<i32>().unwrap(),
        None => 0,
    };

    println!("Original file: {}", input_file);
    println!("Output file: {}", output_file);
    println!("Number of seams to carve: {}", nr_seams);

    let orig_img = image::io::Reader::open(input_file)
        .unwrap()
        .decode()
        .unwrap();
    let img_buf = orig_img.as_rgb8().unwrap();

    // for s in 0..nr_seams {
    let seams = seam::get_seams(img_buf);
    let min_seam = seam::get_min_seam(&seams);
    let highlighted_img = carve::highlight_seam(img_buf, min_seam);
    highlighted_img
        .save(format!("highlight_{}", output_file.as_str()))
        .expect("could not save highlighted image");
    let carved_img = carve::carve_seam(img_buf, min_seam);
    carved_img
        .save(output_file.as_str())
        .expect("could not save carved image");
    // }
}
