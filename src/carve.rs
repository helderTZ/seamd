use crate::seam::FullSeam;
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub fn highlight_seam(
    img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    seam: &FullSeam,
    iteration: i32
) {
    img_buf.par_enumerate_pixels_mut().for_each(|(i,j, pixel)| {
        if j > 0 && (i + iteration as u32) as usize == seam.seam_path[(j-1) as usize].from {
            //TODO: skip col when hit seam
            *pixel = Rgb([255, 0, 0]);
        }
    });

    // should this go from new_h-1 to 0 ?
    // let (w, h) = img_buf.dimensions();
    // for j in 0..(h - 1) as usize {
    //     for i in 0..(w - 1) as usize {
    //         if j > 0 && i == seam.seam_path[j - 1].from {
    //             //TODO: skip col when hit seam
    //             *img_buf.get_pixel_mut(i as u32 + iteration as u32, j as u32) = Rgb([255, 0, 0]);
    //         }
    //     }
    // }
}

pub fn carve_seam(
    img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    seam: &FullSeam,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let new_w = img_buf.width() - 1;
    let new_h = img_buf.height();
    let mut carved_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(new_w, new_h);

    // should this go from new_h-1 to 0 ?
    for j in 0..(new_h - 1) as usize {
        for i in 0..(new_w - 1) as usize {
            if j > 0 && i < seam.seam_path[j - 1].from {
                //TODO: skip col when hit seam
                *carved_img.get_pixel_mut(i as u32, j as u32) =
                    *img_buf.get_pixel(i as u32, j as u32);
            } else {
                *carved_img.get_pixel_mut(i as u32, j as u32) =
                    *img_buf.get_pixel((i + 1) as u32, j as u32);
            }
        }
    }

    carved_img
}
