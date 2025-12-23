use egui::Color32;

/// Flexible color representation supporting hex strings and direct colors
#[derive(Clone, Debug)]
pub enum ChartColor {
    Hex(String),
    Rgba(Color32),
}

impl ChartColor {
    /// Parse to Color32, with fallback for invalid hex
    pub fn to_color32(&self) -> Color32 {
        match self {
            ChartColor::Rgba(c) => *c,
            ChartColor::Hex(hex) => parse_hex_color(hex).unwrap_or(Color32::GRAY),
        }
    }
}

impl From<&str> for ChartColor {
    fn from(s: &str) -> Self {
        ChartColor::Hex(s.to_string())
    }
}

impl From<String> for ChartColor {
    fn from(s: String) -> Self {
        ChartColor::Hex(s)
    }
}

impl From<Color32> for ChartColor {
    fn from(c: Color32) -> Self {
        ChartColor::Rgba(c)
    }
}

impl Default for ChartColor {
    fn default() -> Self {
        ChartColor::Rgba(Color32::from_rgb(54, 162, 235)) // Chart.js default blue
    }
}

/// Parse hex color string to Color32
/// Supports: #RGB, #RGBA, #RRGGBB, #RRGGBBAA (with or without #)
pub fn parse_hex_color(hex: &str) -> Option<Color32> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        // #RGB -> #RRGGBB
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Color32::from_rgb(r, g, b))
        }
        // #RGBA -> #RRGGBBAA
        4 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
            Some(Color32::from_rgba_unmultiplied(r, g, b, a))
        }
        // #RRGGBB
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color32::from_rgb(r, g, b))
        }
        // #RRGGBBAA
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Color32::from_rgba_unmultiplied(r, g, b, a))
        }
        _ => None,
    }
}

/// Lighten a color by a factor (for hover effects)
pub fn lighten(color: Color32, factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let factor = factor.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (r as f32 + (255.0 - r as f32) * factor) as u8,
        (g as f32 + (255.0 - g as f32) * factor) as u8,
        (b as f32 + (255.0 - b as f32) * factor) as u8,
        a,
    )
}

/// Darken a color by a factor
pub fn darken(color: Color32, factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let factor = 1.0 - factor.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        (r as f32 * factor) as u8,
        (g as f32 * factor) as u8,
        (b as f32 * factor) as u8,
        a,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing_6_digit() {
        assert_eq!(
            parse_hex_color("#ff0000"),
            Some(Color32::from_rgb(255, 0, 0))
        );
        assert_eq!(
            parse_hex_color("#36a2eb"),
            Some(Color32::from_rgb(54, 162, 235))
        );
        assert_eq!(
            parse_hex_color("00ff00"), // Without #
            Some(Color32::from_rgb(0, 255, 0))
        );
    }

    #[test]
    fn test_hex_parsing_3_digit() {
        assert_eq!(
            parse_hex_color("#fff"),
            Some(Color32::from_rgb(255, 255, 255))
        );
        assert_eq!(
            parse_hex_color("#f00"),
            Some(Color32::from_rgb(255, 0, 0))
        );
    }

    #[test]
    fn test_invalid_hex() {
        assert_eq!(parse_hex_color("invalid"), None);
        assert_eq!(parse_hex_color("#gg0000"), None);
        assert_eq!(parse_hex_color("#12345"), None); // Wrong length
    }

    #[test]
    fn test_lighten() {
        let color = Color32::from_rgb(100, 100, 100);
        let lightened = lighten(color, 0.5);
        assert!(lightened.r() > color.r());
        assert!(lightened.g() > color.g());
        assert!(lightened.b() > color.b());
    }

    #[test]
    fn test_darken() {
        let color = Color32::from_rgb(100, 100, 100);
        let darkened = darken(color, 0.5);
        assert!(darkened.r() < color.r());
        assert!(darkened.g() < color.g());
        assert!(darkened.b() < color.b());
    }
}
