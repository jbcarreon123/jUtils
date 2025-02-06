use image::imageops::FilterType::Lanczos3;
use image::{DynamicImage, Frames, GenericImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use ab_glyph::{Font, FontArc, FontVec};
use rusttype::{point, Scale};
use serenity::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tracing::{debug, info};
use std::io::Cursor;
use crate::utils::number_utils::Int64Helper;
use crate::CONFIG;
use rayon::prelude::*;
use crate::database::XpPerMessage;

use super::animation_utils::{load_animation_from_url, overlay_image_to_all_frames, AnimationFrame};
use super::dynamic_image_utils::DynamicImageUtils;
use font_kit::source::SystemSource;
use font_kit::properties::Properties;
use font_kit::handle::Handle;
use std::sync::Arc;

#[derive(Clone)]
pub enum Background {
    Url(&'static str),
    Color([u8; 4]),
}

pub fn generate_image(
    username: &str,
    display_name: &str,
    level: i64,
    xp: i64,
    xp_needed: i64,
    rank: i64,
    guild_name: &str,
    secondary_color: [u8; 4],
    avatar_url: &str,    
    background: Background,
    warning: &mut Option<String>
) -> DynamicImage {
    let mut image = image::DynamicImage::ImageRgba8(generate_solid_color_image([0, 0, 0, 0], 1250, 400));
    let avatar = DynamicImage::from_url(avatar_url).unwrap();
    let mut background = match background {
        Background::Url(url) => DynamicImage::from_url(url).unwrap(),
        Background::Color(color) => image::DynamicImage::ImageRgba8(generate_solid_color_image(color, 1250, 400)),
    };

    let inter_bytes = include_bytes!("fonts/Inter-Regular.ttf");
    let inter_semibold_bytes = include_bytes!("fonts/Inter-Semibold.ttf");

    let inter = FontArc::try_from_vec(inter_bytes.to_vec()).unwrap();
    let inter_semibold = FontArc::try_from_vec(inter_semibold_bytes.to_vec()).unwrap();

    let white = [255, 255, 255, 255];

    let avatar = avatar.resize_to_fill(125, 125, Lanczos3)
        .add_rounded_corners(62);
    background = background.resize_to_fill(1250, 400, Lanczos3).set_transparency(128);

    *warning = None;
    overlay_centered(&mut image, &background);
    overlay_images(&mut image, &avatar, 25, 235);
    draw_text_with_fallback(&mut image, white, &inter_semibold, inter_semibold_bytes, 80.0, 175, 235, 600, display_name, warning);
    draw_text(&mut image, secondary_color, &inter, 50.0, 175, 305, 600, &("@".to_string() + username), warning);
    draw_text(&mut image, secondary_color, &inter, 45.0, 25, 20, 250, guild_name, warning);
    draw_text_level_rank(&mut image, white, secondary_color, &inter_semibold, &inter, 80.0, 40.0, 50.0, 25, 20, 500, level, rank);
    draw_text_xp(&mut image, white, secondary_color, &inter_semibold, &inter, 80.0, 45.0, 30, 65, 500, xp, xp_needed);
    let progress = draw_progress_bar(1250, 10, xp as f32 / xp_needed as f32, secondary_color);
    overlay_images(&mut image, &progress, 0, 390);

    image
}

pub fn generate_gif<'a>(
    username: &'a str,
    display_name: &'a str,
    level: i64,
    xp: i64,
    xp_needed: i64,
    rank: i64,
    guild_name: &'a str,
    secondary_color: [u8; 4],
    avatar_url: &'a str,    
    background: Background,
    warning: &'a mut Option<String>
) -> Result<AnimationFrame<'a>, Error> {
    let image = generate_image(username, display_name, level, xp, xp_needed, rank, guild_name, secondary_color, avatar_url, Background::Color([0, 0, 0, 0]), warning);
    match background {
        Background::Url(url) => {
            let mut frames = load_animation_from_url(url).unwrap();
            overlay_image_to_all_frames(&mut frames, &image).unwrap();
            match frames {
                AnimationFrame::Gif(frames) => Ok(AnimationFrame::Gif(frames)),
                AnimationFrame::Apng(_) => {
                    return Err(serenity::Error::Other("APNG support is not implemented yet"));
                }
            }
        },
        Background::Color(_) => {
            return Err(serenity::Error::Other("Color background is not supported for GIFs"));
        }
    }
}

fn generate_solid_color_image(color: [u8; 4], width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_pixel(width, height, Rgba(color))
}

fn overlay_images(base: &mut DynamicImage, overlay: &DynamicImage, x: i64, y: i64) {
    image::imageops::overlay(base, overlay, x, y);
}

fn overlay_centered(base: &mut DynamicImage, overlay: &DynamicImage) {
    let (base_width, base_height) = base.dimensions();
    let (overlay_width, overlay_height) = overlay.dimensions();
    let x = (base_width - overlay_width) / 2;
    let y = (base_height - overlay_height) / 2;
    overlay_images(base, overlay, x as i64, y as i64);
}

fn draw_text(image: &mut DynamicImage, color: [u8; 4], font: &FontArc, scale: f32, x: i64, y: i64, width: u32, text: &str, warning: &mut Option<String>) {
    let mut text_dimensions = text_size(scale, font, text);
    let mut truncated_text = text.to_string();
    let mut text_dimensions = text_size(scale, font, &truncated_text);

    while text_dimensions.0 > width {
        if truncated_text.is_empty() {
            break;
        }
        if (truncated_text[..].chars().last().unwrap() == '…') {
            truncated_text.pop();
        }
        truncated_text.pop();
        truncated_text.push('…');
        text_dimensions = text_size(scale, font, &truncated_text);
    }

    let mut image_temp = generate_solid_color_image([0, 0, 0, 0], width + 32 as u32, text_dimensions.1 + 32 as u32);
    for c in text.chars() {
        if !font_contains_glyph(font, c) {
            *warning = Some("The font does not contain the character: ".to_string() + &c.to_string() + ". \nYour username/display name may not display correctly.");
        }
    }
    draw_text_mut(&mut image_temp, Rgba(color), 0, 0, scale, font, &truncated_text);
    let mut text_image = DynamicImage::ImageRgba8(image_temp);
    overlay_images(image, &text_image, x, y);
}

fn draw_text_level_rank(image: &mut DynamicImage, color: [u8; 4], color_secondary: [u8; 4], font: &FontArc, font_secondary: &FontArc, scale: f32, scale_secondary: f32, gap: f32, x: i64, y: i64, width: u32, level: i64, rank: i64) {
    let mut level_dimensions = text_size(scale, font, level.pretty_format().as_str());
    let mut rank_dimensions = text_size(scale, font, rank.to_string().as_str());
    let mut level_name_dimensions = text_size(scale_secondary, font_secondary, "LEVEL");
    let mut rank_name_dimensions = text_size(scale_secondary, font_secondary, "RANK");

    let text_x = (image.dimensions().0 as i64 - (level_dimensions.0 + rank_dimensions.0 + level_name_dimensions.0 + rank_name_dimensions.0 + gap as u32 + 20) as i64) - x;
    let mut image_temp = generate_solid_color_image([0, 0, 0, 0], (level_dimensions.0 + rank_dimensions.0 + level_name_dimensions.0 + rank_name_dimensions.0 + gap as u32 + 20), (level_dimensions.1 + rank_dimensions.1 + level_name_dimensions.1 + rank_name_dimensions.1 + 32) as u32);
    draw_text_mut(&mut image_temp, Rgba(color_secondary), 0, (level_dimensions.1 - level_name_dimensions.1 + 5).try_into().unwrap(), scale_secondary, font_secondary, &"LEVEL");
    draw_text_mut(&mut image_temp, Rgba(color), level_name_dimensions.0 as i32 + 10, 0, scale, font, &level.pretty_format());
    draw_text_mut(&mut image_temp, Rgba(color_secondary), level_name_dimensions.0 as i32 + level_dimensions.0 as i32 + gap as i32, (rank_dimensions.1 - rank_name_dimensions.1 + 5).try_into().unwrap(), scale_secondary, font_secondary, &"RANK");
    draw_text_mut(&mut image_temp, Rgba(color), level_name_dimensions.0 as i32 + level_dimensions.0 as i32 + gap as i32 + rank_name_dimensions.0 as i32 + 10, 0, scale, font, &rank.to_string());
    let mut text_image = DynamicImage::ImageRgba8(image_temp);
    overlay_images(image, &text_image, text_x, y);
}

fn draw_text_xp(image: &mut DynamicImage, color: [u8; 4], color_secondary: [u8; 4], font: &FontArc, font_secondary: &FontArc, scale: f32, scale_secondary: f32, x: i64, y: i64, width: u32, xp: i64, xp_needed: i64) {
    let mut xp_dimensions = text_size(scale, font, xp.pretty_format().as_str());
    let mut xp_needed_dimensions = text_size(scale_secondary, font_secondary, &("/".to_string() + xp_needed.pretty_format().as_str()));

    let text_x = (image.dimensions().0 as i64 - (xp_dimensions.0 + xp_needed_dimensions.0) as i64) - x;
    let text_y: i64 = (image.dimensions().1 as i64 - xp_dimensions.1 as i64) - y;
    let mut image_temp = generate_solid_color_image([0, 0, 0, 0], (xp_dimensions.0 + xp_needed_dimensions.0) as u32, (xp_dimensions.1 + 32) as u32);
    draw_text_mut(&mut image_temp, Rgba(color), 0, 0, scale, font, &xp.pretty_format());
    draw_text_mut(&mut image_temp, Rgba(color_secondary), xp_dimensions.0 as i32, (xp_dimensions.1 - xp_needed_dimensions.1 + 10).try_into().unwrap(), scale_secondary, font_secondary, &("/".to_string() + xp_needed.pretty_format().as_str()));
    let mut text_image = DynamicImage::ImageRgba8(image_temp);
    overlay_images(image, &text_image, text_x, text_y);
}

fn draw_progress_bar(
    width: u32,
    height: u32,
    progress: f32,
    foreground_color: [u8; 4],
) -> DynamicImage {
    let mut img = generate_solid_color_image([0, 0, 0, 0], width, height); // Transparent background
    let progress_width = (width as f32 * progress).round() as u32;

    for y in 0..height {
        for x in 0..progress_width {
            if x >= width {
                break;
            }
            img.put_pixel(x, y, Rgba(foreground_color));
        }
    }

    DynamicImage::ImageRgba8(img)
}

fn font_contains_glyph(font: &FontArc, c: char) -> bool {
    font.glyph_id(c).0 != 0
}

fn draw_text_with_fallback(
    image: &mut DynamicImage,
    color: [u8; 4],
    font: &FontArc,
    font_bytes: &[u8],
    scale: f32,
    x: i64,
    y: i64,
    width: u32,
    text: &str,
    warning: &mut Option<String>,
) {
    let mut text_dimensions = text_size(scale, font, text);
    let mut truncated_text = text.to_string();
    let mut text_dimensions = text_size(scale, font, &truncated_text);

    while text_dimensions.0 > width {
        if truncated_text.is_empty() {
            break;
        }
        if truncated_text[..].chars().last().unwrap() == '…' {
            truncated_text.pop();
        }
        truncated_text.pop();
        truncated_text.push('…');
        text_dimensions = text_size(scale, font, &truncated_text);
    }

    let mut image_temp = generate_solid_color_image([0, 0, 0, 0], width + 32 as u32, text_dimensions.1 + 32 as u32);
    
    let fallback_fonts: Arc<Mutex<Vec<FontArc>>> = Arc::new(Mutex::new(Vec::new()));

    let system_source = SystemSource::new();
    let handles: Vec<Handle> = system_source.all_fonts().unwrap();

    text.chars().collect::<Vec<_>>().par_iter().for_each(|&c| {
        let mut warning = &warning;
        let found = Arc::new(AtomicBool::new(false));
        if !font_contains_glyph(font, c) {
            let fallback_fonts = Arc::clone(&fallback_fonts);
            let found_clone = Arc::clone(&found);
            handles.par_iter().for_each(|handle| {
                if let Handle::Path { path, .. } = handle {
                    let data = std::fs::read(path).unwrap();
                    let test_font = FontArc::try_from_vec(data).unwrap();
                    if font_contains_glyph(&test_font, c) {
                        fallback_fonts.lock().unwrap().push(test_font);
                        println!("Found fallback font for character: {}, font: {}", c, path.to_str().unwrap());
                        found_clone.store(true, Ordering::Relaxed);
                    }
                }
            });
            if !found.load(Ordering::Relaxed) {
                warning = &&mut Some("The font does not contain the character: ".to_string() + &c.to_string() + ". \nYour username/display name may not display correctly.");
            }
        }
    });

    let mut current_x = 0;
    for c in truncated_text.chars() {
        let fallback_fonts_locked = fallback_fonts.lock().unwrap();
        let font_to_use = fallback_fonts_locked.iter().find(|f| font_contains_glyph(f, c)).unwrap_or(font);
        let glyph = font_to_use.glyph_id(c).with_scale_and_position(scale, (0.0, 0.0));
        let glyph_bounds = font_to_use.glyph_bounds(&glyph);
        let glyph_width = glyph_bounds.width() as u32;
        draw_text_mut(&mut image_temp, Rgba(color), current_x, 0, scale, font_to_use, &c.to_string());
        current_x += glyph_width as i32;
    }

    let mut text_image = DynamicImage::ImageRgba8(image_temp);
    overlay_images(image, &text_image, x, y);
}