use crate::animation::Interpolator;
use std::fmt;

/// Describes a color.
///
/// This type is part of the prelude.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone)]
pub struct Color {
    pub data: u32,
}

impl Color {
    // Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0x0000_00FF }
    }

    // Create a new color from RGBA
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) }
    }

    /// Returns color value specified by hue, saturation and lightness.
    /// HSL values are all in range [0..1], alpha will be set to 1.0.
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 1.0)
    }

    /// Returns color value specified by hue, saturation, lightness and alpha.
    /// All values are in range [0..1]
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let a = (a * 255.0) as u8;
        let mut h = h % 1.0;

        if h < 0.0 {
            h += 1.0;
        }

        let s = s.max(0.0).min(1.0);
        let l = l.max(0.0).min(1.0);

        let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let m1 = 2.0 * l - m2;

        let r = (hue(h + 1.0 / 3.0, m1, m2).max(0.0).min(1.0) * 255.0) as u8;
        let g = (hue(h, m1, m2).max(0.0).min(1.0) * 255.0) as u8;
        let b = (hue(h - 1.0 / 3.0, m1, m2).max(0.0).min(1.0) * 255.0) as u8;

        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) }
    }

    // Get the red value
    pub fn r(self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }

    // Get the green value
    pub fn g(self) -> u8 {
        ((self.data & 0x00FF_0000) >> 16) as u8
    }

    // Get the blue value
    pub fn b(self) -> u8 {
        ((self.data & 0x0000_FF00) >> 8) as u8
    }

    // Get the alpha value
    pub fn a(self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }

    // Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        (end_color as f64 - start_color as f64).mul_add(scale, start_color as f64) as u8
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        if self.a() == 0 {
            return String::from("transparent");
        }

        let data = self.data;

        format!("#{:x}", data)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Color {
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            3 | 4 => {
                let hex = clean_hex.as_bytes();
                let r = (hex[0] as char).to_digit(16).unwrap() as u8 * 17;
                let g = (hex[1] as char).to_digit(16).unwrap() as u8 * 17;
                let b = (hex[2] as char).to_digit(16).unwrap() as u8 * 17;

                let mut data = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8);

                if clean_hex.len() == 3 {
                    data |= 0x00_000_0FF;
                } else {
                    let a = (hex[0] as char).to_digit(16).unwrap() as u8 * 17;
                    data |= a as u32;
                }

                Color { data }
            }

            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x = (x << 8) | 0x00_000_0FF;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }
}

impl From<String> for Color {
    fn from(s: String) -> Color {
        Color::from(s.as_str())
    }
}

impl From<Color> for femtovg::Color {
    fn from(src: Color) -> femtovg::Color {
        femtovg::Color::rgba(src.r(), src.g(), src.b(), src.a())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r() == other.r()
            && self.g() == other.g()
            && self.b() == other.b()
            && self.a() == other.a()
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r(), self.g(), self.b(), self.a())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgba(0, 0, 0, 0)
    }
}

impl Color {
    pub const fn transparent() -> Self {
        Self { data: 0x00000000 }
    }

    pub const fn black() -> Self {
        Self { data: 0x000000FF }
    }

    pub const fn white() -> Self {
        Self { data: 0xFFFFFFFF }
    }

    pub const fn red() -> Self {
        Self { data: 0xFF0000FF }
    }

    pub const fn green() -> Self {
        Self { data: 0x008000FF }
    }

    pub const fn blue() -> Self {
        Self { data: 0x0000FFFF }
    }

    pub const fn yellow() -> Self {
        Self { data: 0xFFFF00FF }
    }

    pub const fn cyan() -> Self {
        Self { data: 0x00FFFFFF }
    }

    pub const fn magenta() -> Self {
        Self { data: 0xFF00FFFF }
    }
}

impl Interpolator for Color {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Color::interpolate(start.clone(), end.clone(), t as f64)
    }
}

fn hue(mut h: f32, m1: f32, m2: f32) -> f32 {
    if h < 0.0 {
        h += 1.0;
    }
    if h > 1.0 {
        h -= 1.0;
    }

    if h < 1.0 / 6.0 {
        return m1 + (m2 - m1) * h * 6.0;
    }
    if h < 3.0 / 6.0 {
        return m2;
    }
    if h < 4.0 / 6.0 {
        return m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0;
    }

    m1
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_hex() {
        let hex_color = "#FF00FF88";
        let color = Color::from(hex_color);

        assert_eq!(color, Color::rgba(255, 0, 255, 136));
    }

    #[test]
    fn test_short_hex() {
        let hex_color = "#FFF";
        let color = Color::from(hex_color);

        assert_eq!(color, Color::rgba(255, 255, 255, 255));
    }
}
