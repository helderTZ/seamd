use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub struct SeamHistory {
    pub energy: u32,
    pub from: usize,
}

impl SeamHistory {
    pub fn new(energy: u32, from: usize) -> Self {
        Self { energy, from }
    }
}

pub fn get_pixel_energy(img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32) -> u32 {
    let (w, h) = img_buf.dimensions();

    let x0 = if x == 0 { x } else { x - 1 };
    let x1 = if x == w - 1 { x } else { x + 1 };
    let y0 = if y == 0 { y } else { y - 1 };
    let y1 = if y == h - 1 { y } else { y + 1 };

    let delta_xr = ((img_buf.get_pixel(x0, y).0[0] as i32) - (img_buf.get_pixel(x1, y).0[0] as i32)) as u32;
    let delta_xg = ((img_buf.get_pixel(x0, y).0[1] as i32) - (img_buf.get_pixel(x1, y).0[1] as i32)) as u32;
    let delta_xb = ((img_buf.get_pixel(x0, y).0[2] as i32) - (img_buf.get_pixel(x1, y).0[2] as i32)) as u32;
    let delta_x = delta_xr.wrapping_mul(delta_xr) + delta_xg.wrapping_mul(delta_xg) + delta_xb.wrapping_mul(delta_xb);

    let delta_yr = ((img_buf.get_pixel(x, y0).0[0] as i32) - (img_buf.get_pixel(x, y1).0[0] as i32)) as u32;
    let delta_yg = ((img_buf.get_pixel(x, y0).0[1] as i32) - (img_buf.get_pixel(x, y1).0[1] as i32)) as u32;
    let delta_yb = ((img_buf.get_pixel(x, y0).0[2] as i32) - (img_buf.get_pixel(x, y1).0[2] as i32)) as u32;
    let delta_y = delta_yr.wrapping_mul(delta_yr) + delta_yg.wrapping_mul(delta_yg) + delta_yb.wrapping_mul(delta_yb);

    delta_x + delta_y
}

pub fn get_image_energy(img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u32> {
    let (w, h) = img_buf.dimensions();
    let energy_data: Vec<u32> = (0..(w*h)).into_par_iter().map(|i| {
        get_pixel_energy(img_buf, i/w, i%h)
    }).collect();

    energy_data
}

fn min<T: Ord>(a: T, b: T, c: T) -> T {
    std::cmp::min(std::cmp::min(a, b), c)
}

fn get_transition(from: u32, to: u32) -> u32 {
    (from as i32 - to as i32).unsigned_abs()
}

fn get_seam_direction(
    energy_data: &[u32],
    start: usize,
    left: usize,
    middle: usize,
    right: usize,
    x: usize,
    w: usize,
) -> (u32, usize) {
    let min_energy;
    let min_index;
    if x == 0 {
        min_energy = std::cmp::min(
            get_transition(energy_data[start], energy_data[middle]),
            get_transition(energy_data[start], energy_data[right]),
        );
        min_index = if min_energy == get_transition(energy_data[start], energy_data[middle]) {
            x
        } else {
            x + 1
        };
    } else if x == w - 1 {
        min_energy = std::cmp::min(
            get_transition(energy_data[start], energy_data[left]),
            get_transition(energy_data[start], energy_data[middle]),
        );
        min_index = if min_energy == get_transition(energy_data[start], energy_data[middle]) {
            x
        } else {
            x - 1
        };
    } else {
        min_energy = min(
            get_transition(energy_data[start], energy_data[left]),
            get_transition(energy_data[start], energy_data[middle]),
            get_transition(energy_data[start], energy_data[right]),
        );
        min_index = if min_energy == get_transition(energy_data[start], energy_data[middle]) {
            x
        } else if min_energy == get_transition(energy_data[start], energy_data[left]) {
            x - 1
        } else {
            x + 1
        };
    }
    (min_energy, min_index)
}

pub fn get_seam_starting_at(energy_data: &[u32], w: usize, h: usize, x: usize) -> Vec<SeamHistory> {
    let mut seam: Vec<SeamHistory> = Vec::with_capacity(h);
    seam.push(SeamHistory::new(energy_data[x], usize::MAX));
    for j in 1..(h - 1) as usize {
        let (min_energy, min_index) = get_seam_direction(
            energy_data,
            j * w + x,
            (j + 1) * w + x - 1,
            (j + 1) * w + x,
            (j + 1) * w + x + 1,
            x,
            w,
        );
        seam.push(SeamHistory::new(min_energy, min_index));
    }

    // let mut seam: Vec<SeamHistory> = Vec::with_capacity(h);
    // seam.push(SeamHistory::new(energy_data[x], usize::MAX));
    // let mut seam_rest: Vec<SeamHistory> = (1..h-1).into_par_iter().map(|j| {
    //     let (min_energy, min_index) = get_seam_direction(energy_data, 
    //         j*w + x,
    //         (j + 1) * w + x - 1,
    //         (j + 1) * w + x,
    //         (j + 1) * w + x + 1,
    //         x, w);
    //     SeamHistory::new(min_energy, min_index)
    // }).collect();
    // seam.append(&mut seam_rest);

    // let seam: Vec<SeamHistory> = (0..h-1).into_par_iter().map(|j| {
    //     if j == 0 {
    //         SeamHistory::new(energy_data[x], usize::MAX)
    //     } else {
    //         let (min_energy, min_index) = get_seam_direction(energy_data, 
    //             j*w + x,
    //             (j + 1) * w + x - 1,
    //             (j + 1) * w + x,
    //             (j + 1) * w + x + 1,
    //             x, w);
    //         SeamHistory::new(min_energy, min_index)
    //     }
    // }).collect();

    seam
}

pub fn get_seams(img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<Vec<SeamHistory>> {
    let (w, h) = img_buf.dimensions();
    let energy_data = get_image_energy(img_buf);

    let mut seams: Vec<Vec<SeamHistory>> = Vec::with_capacity(w as usize);

    for i in 0..w as usize {
        seams.push(get_seam_starting_at(
            &energy_data,
            w as usize,
            h as usize,
            i,
        ));
    }

    seams
}

pub fn get_min_seam(seams: &Vec<Vec<SeamHistory>>) -> &Vec<SeamHistory> {
    let mut min_seam: &Vec<SeamHistory> = &seams[0];
    for i in 1..seams.len() as usize {
        if min_seam.last().unwrap().energy > seams[i].last().unwrap().energy {
            min_seam = &seams[i];
        }
    }

    min_seam
}
