/// Theme system for rtop with color gradients and customizable color schemes
/// Inspired by btop++'s comprehensive theming system
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Color gradient for smooth transitions
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ColorGradient {
    colors: Vec<Color>,
}

#[allow(dead_code)]
impl ColorGradient {
    /// Create a gradient from start to end color
    pub fn new(start: Color, end: Color, steps: usize) -> Self {
        let colors = Self::interpolate_colors(start, end, steps);
        Self { colors }
    }

    /// Create a multi-color gradient
    pub fn from_colors(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    /// Get color at percentage (0-100)
    pub fn at(&self, percentage: u8) -> Color {
        let percentage = percentage.min(100);
        let idx = (self.colors.len() - 1) * percentage as usize / 100;
        self.colors[idx.min(self.colors.len() - 1)]
    }

    fn interpolate_colors(start: Color, end: Color, steps: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(steps);

        let (r1, g1, b1) = Self::color_to_rgb(start);
        let (r2, g2, b2) = Self::color_to_rgb(end);

        for i in 0..steps {
            let t = i as f64 / (steps - 1) as f64;
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8;
            colors.push(Color::Rgb(r, g, b));
        }

        colors
    }

    fn color_to_rgb(color: Color) -> (u8, u8, u8) {
        match color {
            Color::Rgb(r, g, b) => (r, g, b),
            Color::Black => (0, 0, 0),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Yellow => (255, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Magenta => (255, 0, 255),
            Color::Cyan => (0, 255, 255),
            Color::Gray => (128, 128, 128),
            Color::DarkGray => (64, 64, 64),
            Color::LightRed => (255, 128, 128),
            Color::LightGreen => (128, 255, 128),
            Color::LightYellow => (255, 255, 128),
            Color::LightBlue => (128, 128, 255),
            Color::LightMagenta => (255, 128, 255),
            Color::LightCyan => (128, 255, 255),
            Color::White => (255, 255, 255),
            _ => (128, 128, 128),
        }
    }
}

/// Theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,

    // Main UI colors
    pub main_fg: String,
    pub main_bg: String,
    pub title: String,
    pub hi_fg: String,
    pub selected_bg: String,
    pub selected_fg: String,
    pub inactive_fg: String,
    pub div_line: String,

    // Graph colors
    pub graph_text: String,
    pub meter_bg: String,

    // Box colors
    pub cpu_box: String,
    pub mem_box: String,
    pub net_box: String,
    pub proc_box: String,
    pub gpu_box: Option<String>,

    // Process colors
    pub proc_misc: String,
    pub proc_start: String,
    pub proc_mid: String,
    pub proc_end: String,

    // CPU gradient
    pub cpu_start: String,
    pub cpu_mid: String,
    pub cpu_end: String,

    // Memory gradient
    pub mem_start: String,
    pub mem_mid: String,
    pub mem_end: String,

    // Network gradient
    pub net_start: String,
    pub net_mid: String,
    pub net_end: String,

    // Disk gradient
    pub download_start: Option<String>,
    pub download_mid: Option<String>,
    pub download_end: Option<String>,
    pub upload_start: Option<String>,
    pub upload_mid: Option<String>,
    pub upload_end: Option<String>,

    // Temperature gradient
    pub temp_start: Option<String>,
    pub temp_mid: Option<String>,
    pub temp_end: Option<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            main_fg: "#cc".to_string(),
            main_bg: "#00".to_string(),
            title: "#ee".to_string(),
            hi_fg: "#b54040".to_string(),
            selected_bg: "#7e2626".to_string(),
            selected_fg: "#ee".to_string(),
            inactive_fg: "#40".to_string(),
            div_line: "#30".to_string(),
            graph_text: "#60".to_string(),
            meter_bg: "#40".to_string(),
            cpu_box: "#3d7b46".to_string(),
            mem_box: "#8a882e".to_string(),
            net_box: "#423ba5".to_string(),
            proc_box: "#923535".to_string(),
            gpu_box: Some("#35934d".to_string()),
            proc_misc: "#0de756".to_string(),
            proc_start: "#80d0a3".to_string(),
            proc_mid: "#26e85f".to_string(),
            proc_end: "#0de756".to_string(),
            cpu_start: "#4897d8".to_string(),
            cpu_mid: "#7ce567".to_string(),
            cpu_end: "#eb7070".to_string(),
            mem_start: "#ffc345".to_string(),
            mem_mid: "#f3a32e".to_string(),
            mem_end: "#e05a5a".to_string(),
            net_start: "#90e0b5".to_string(),
            net_mid: "#50d097".to_string(),
            net_end: "#30b572".to_string(),
            download_start: Some("#80d0a3".to_string()),
            download_mid: Some("#26e85f".to_string()),
            download_end: Some("#0de756".to_string()),
            upload_start: Some("#d08090".to_string()),
            upload_mid: Some("#e82656".to_string()),
            upload_end: Some("#e70d56".to_string()),
            temp_start: Some("#4897d8".to_string()),
            temp_mid: Some("#f3a32e".to_string()),
            temp_end: Some("#eb7070".to_string()),
        }
    }
}

#[allow(dead_code)]
impl Theme {
    /// Load theme from TOML file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let theme: Theme = toml::from_str(&contents)?;
        Ok(theme)
    }

    /// Save theme to TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    /// Parse color from hex string (#RGB or #RRGGBB format)
    pub fn parse_color(hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');

        // Handle short format (#RGB -> #RRGGBB)
        let hex = if hex.len() == 2 {
            // Grayscale format (like #cc)
            let val = u8::from_str_radix(hex, 16).unwrap_or(128);
            return Color::Rgb(val, val, val);
        } else if hex.len() == 3 {
            format!("{0}{0}{1}{1}{2}{2}", &hex[0..1], &hex[1..2], &hex[2..3])
        } else if hex.len() == 6 {
            hex.to_string()
        } else {
            return Color::Gray;
        };

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);

        Color::Rgb(r, g, b)
    }

    /// Get color from theme
    pub fn get_color(&self, name: &str) -> Color {
        let hex = match name {
            "main_fg" => &self.main_fg,
            "main_bg" => &self.main_bg,
            "title" => &self.title,
            "hi_fg" => &self.hi_fg,
            "selected_bg" => &self.selected_bg,
            "selected_fg" => &self.selected_fg,
            "inactive_fg" => &self.inactive_fg,
            "div_line" => &self.div_line,
            "graph_text" => &self.graph_text,
            "meter_bg" => &self.meter_bg,
            "cpu_box" => &self.cpu_box,
            "mem_box" => &self.mem_box,
            "net_box" => &self.net_box,
            "proc_box" => &self.proc_box,
            _ => &self.main_fg,
        };
        Self::parse_color(hex)
    }

    /// Create CPU gradient
    pub fn cpu_gradient(&self) -> ColorGradient {
        ColorGradient::from_colors(vec![
            Self::parse_color(&self.cpu_start),
            Self::parse_color(&self.cpu_mid),
            Self::parse_color(&self.cpu_end),
        ])
    }

    /// Create memory gradient
    pub fn mem_gradient(&self) -> ColorGradient {
        ColorGradient::from_colors(vec![
            Self::parse_color(&self.mem_start),
            Self::parse_color(&self.mem_mid),
            Self::parse_color(&self.mem_end),
        ])
    }

    /// Create network gradient
    pub fn net_gradient(&self) -> ColorGradient {
        ColorGradient::from_colors(vec![
            Self::parse_color(&self.net_start),
            Self::parse_color(&self.net_mid),
            Self::parse_color(&self.net_end),
        ])
    }

    /// Create temperature gradient
    pub fn temp_gradient(&self) -> ColorGradient {
        let start = self.temp_start.as_ref().unwrap_or(&self.cpu_start);
        let mid = self.temp_mid.as_ref().unwrap_or(&self.cpu_mid);
        let end = self.temp_end.as_ref().unwrap_or(&self.cpu_end);

        ColorGradient::from_colors(vec![
            Self::parse_color(start),
            Self::parse_color(mid),
            Self::parse_color(end),
        ])
    }
}

/// Theme manager to handle multiple themes
pub struct ThemeManager {
    #[allow(dead_code)]
    themes: HashMap<String, Theme>,
    #[allow(dead_code)]
    current_theme: String,
}

#[allow(dead_code)]
impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), Theme::default());

        Self {
            themes,
            current_theme: "default".to_string(),
        }
    }

    /// Load themes from directory
    pub fn load_themes_from_dir(
        &mut self,
        dir: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Ok(theme) = Theme::load_from_file(&path) {
                    self.themes.insert(theme.name.clone(), theme);
                }
            }
        }

        Ok(())
    }

    /// Get current theme
    pub fn current(&self) -> &Theme {
        self.themes
            .get(&self.current_theme)
            .unwrap_or_else(|| self.themes.get("default").unwrap())
    }

    /// Set current theme
    pub fn set_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    /// List available themes
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    /// Add a theme
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_gradient() {
        let gradient = ColorGradient::new(Color::Blue, Color::Red, 101);
        let color_at_50 = gradient.at(50);
        // Should be somewhere between blue and red
        assert!(matches!(color_at_50, Color::Rgb(_, _, _)));
    }

    #[test]
    fn test_parse_color() {
        let color1 = Theme::parse_color("#ff0000");
        assert_eq!(color1, Color::Rgb(255, 0, 0));

        let color2 = Theme::parse_color("#f00");
        assert_eq!(color2, Color::Rgb(255, 0, 0));

        let color3 = Theme::parse_color("#80");
        assert_eq!(color3, Color::Rgb(128, 128, 128));
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        let theme_name = manager.current().name.clone();
        assert_eq!(theme_name, "default");

        let custom_theme = Theme {
            name: "custom".to_string(),
            ..Theme::default()
        };
        manager.add_theme(custom_theme);

        assert!(manager.set_theme("custom"));
        assert_eq!(manager.current().name, "custom");
    }
}
