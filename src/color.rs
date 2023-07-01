use crate::rtweekend::clamp;
use crate::Color;
use std::io::Write;

pub fn write_color(
    os: &mut dyn Write,
    pixel_color: &Color,
    samples_per_pixel: usize,
) -> std::io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (r * scale).sqrt();
    let g = (g * scale).sqrt();
    let b = (b * scale).sqrt();

    let s = format!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as usize,
        (256.0 * clamp(g, 0.0, 0.999)) as usize,
        (256.0 * clamp(b, 0.0, 0.999)) as usize,
    );
    os.write_all(s.as_bytes())?;
    Ok(())
}
