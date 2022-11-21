use crate::seam::SeamHistory;
use image::{ImageBuffer, Rgb};

pub fn highlight_seam(
    img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    seam: &[SeamHistory],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (w, h) = img_buf.dimensions();
    let mut highlighted_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(w, h);

    // should this go from new_h-1 to 0 ?
    for j in 0..(h - 1) as usize {
        for i in 0..(w - 1) as usize {
            if j > 0 && i == seam[j - 1].from {
                //TODO: skip col when hit seam
                *highlighted_img.get_pixel_mut(i as u32, j as u32) = Rgb([255, 0, 0]);
            } else {
                *highlighted_img.get_pixel_mut(i as u32, j as u32) =
                    *img_buf.get_pixel(i as u32, j as u32);
            }
        }
    }

    highlighted_img
}

pub fn carve_seam(
    img_buf: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    seam: &[SeamHistory],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let new_w = img_buf.width() - 1;
    let new_h = img_buf.height();
    let mut carved_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(new_w, new_h);

    // should this go from new_h-1 to 0 ?
    for j in 0..(new_h - 1) as usize {
        for i in 0..(new_w - 1) as usize {
            if j > 0 && i < seam[j - 1].from {
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
