use image::{codecs::gif::{self, GifDecoder, GifEncoder}, imageops::FilterType::Lanczos3, AnimationDecoder, Frame, GenericImageView, ImageError};
use itertools::Itertools;
use png::{Encoder as ApngEncoder, Decoder as ApngDecoder};
use reqwest::blocking::get;
use tokio::task;
use std::io::Cursor;
use rayon::prelude::*;

pub enum AnimationFrame<'a> {
    Gif(image::Frames<'a>),
    Apng(image::Frames<'a>),
}

pub fn load_animation_from_url(url: &str) -> Result<AnimationFrame, Box<dyn std::error::Error>> {
    task::block_in_place(|| {
        let response = get(url)?;
        let bytes = response.bytes()?;
        
        // Check if the animation is larger than 10MB
        if bytes.len() > 10 * 1024 * 1024 {
            return Err("File too large".into());
        }

        let cursor = Cursor::new(bytes);
        
        if url.contains(".png") || url.contains(".apng") {
            // TODO: Implement APNG support
            return Err("APNG support is not implemented yet".into());
        } else {
            // Decode the GIF
            let mut decoder = GifDecoder::new(cursor)?;
            let mut reader = decoder.into_frames();
            
            // Read the first frame
            if let Some(frame) = reader.next() {
                Ok(AnimationFrame::Gif(reader))
            } else {
                Err("No frames found in the GIF".into())
            }
        }
    })
}

pub fn overlay_image_to_all_frames(animation_frames: &mut AnimationFrame, overlay: &image::DynamicImage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match animation_frames {
        AnimationFrame::Gif(frames) => {
            let mut new_frames = Vec::new();
            let frames_vec: Vec<_> = frames.collect();
            new_frames = frames_vec
                .par_iter()
                .map(|frame| {
                    let frame = frame.as_ref().unwrap().clone();
                    let mut frame_image = image::DynamicImage::ImageRgba8(frame.clone().into_buffer());
                    frame_image = frame_image.resize_to_fill(overlay.dimensions().0, overlay.dimensions().1, Lanczos3);
                    image::imageops::overlay(&mut frame_image, overlay, 0, 0);
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(image::Frame::from_parts(frame_image.to_rgba8(), 0, 0, frame.delay()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let new_frames = image::Frames::new(Box::new(new_frames.into_iter().map(|f| Ok(f.clone()))));
            *animation_frames = AnimationFrame::Gif(new_frames);
        },
        AnimationFrame::Apng(frames) => {
            // TODO: Implement APNG support
            return Err("APNG support is not implemented yet".into());
        }
    }
    Ok(())
}

pub fn save_animation_to_file(animation_frames: AnimationFrame, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(file_path)?;
    let w = &mut std::io::BufWriter::with_capacity(8192, file); // Increased buffer size for better performance

    match animation_frames {
        AnimationFrame::Gif(frames) => {
            let frames_vec: Vec<_> = frames.collect();
            let total_size: usize = frames_vec.iter().map(|frame| frame.as_ref().unwrap().buffer().len()).sum();
            if total_size > 124 * 1024 * 1024 {
                return Err("Total frame size exceeds 12MB".into());
            }
            let mut encoder = GifEncoder::new_with_speed(w, 10); // Reduced speed for faster encoding
            encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;
            encoder.try_encode_frames(frames_vec.into_iter().map(|frame| Ok(frame.unwrap())))?;
        },
        AnimationFrame::Apng(frames) => {
            // TODO: Implement APNG support
            return Err("APNG support is not implemented yet".into());
        }
    }
    Ok(())
}