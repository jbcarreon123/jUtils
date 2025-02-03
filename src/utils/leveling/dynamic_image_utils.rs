use image::{DynamicImage, ImageBuffer, ImageError, Rgba};
use imageproc::drawing::Canvas;
use reqwest::blocking::get;
use tokio::task;
use std::io::Cursor;

pub trait DynamicImageUtils {
    fn from_url(url: &str) -> Result<DynamicImage, ImageError>;
    fn add_rounded_corners(self, radius: u32) -> DynamicImage;
    fn set_transparency(self, alpha: u8) -> DynamicImage;
}

impl DynamicImageUtils for DynamicImage {
    fn from_url(url: &str) -> Result<DynamicImage, ImageError> {
        task::block_in_place(|| {
            let response = get(url).map_err(|_| ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch image")))?;
            let bytes = response.bytes().map_err(|_| ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to read image bytes")))?;
            let format = image::guess_format(&bytes).map_err(|_| ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to guess image format")))?;
            let cursor = Cursor::new(bytes);
            image::load(cursor, format)
        })
    }

    fn add_rounded_corners(self, radius: u32) -> DynamicImage {
        let image = self;
        let (width, height) = image.dimensions();
        let mut mask = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 0]));
    
        for y in 0..height {
            for x in 0..width {
                let dx = if x < radius { radius - x } else if x >= width - radius { x - (width - radius - 1) } else { 0 };
                let dy = if y < radius { radius - y } else if y >= height - radius { y - (height - radius - 1) } else { 0 };
                if dx * dx + dy * dy <= radius * radius {
                    mask.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                }
            }
        }
    
        let mut new_image = image.to_rgba8();
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            let mask_pixel = mask.get_pixel(x, y);
            if mask_pixel[3] == 0 {
                *pixel = Rgba([0, 0, 0, 0]);
            }
        }
    
        image::DynamicImage::ImageRgba8(new_image)
    }
    
    fn set_transparency(self, alpha: u8) -> DynamicImage {
        let image = self;
        let (width, height) = image.dimensions();
        let mut new_image = image.to_rgba8();
    
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            let mut new_pixel = *pixel;
            new_pixel[3] = alpha;
            *pixel = new_pixel;
        }
    
        DynamicImage::ImageRgba8(new_image)
    }    
}