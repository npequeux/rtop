pub fn format_bytes(bytes: u64, decimal: bool) -> String {
    if bytes == 0 {
        return "0.00 B".to_string();
    }

    let base = if decimal { 1000.0 } else { 1024.0 };
    let units = if decimal {
        vec!["B", "KB", "MB", "GB", "TB", "PB"]
    } else {
        vec!["B", "KiB", "MiB", "GiB", "TiB", "PiB"]
    };

    let exp = (bytes as f64).log(base).floor() as usize;
    let exp = exp.min(units.len() - 1);

    let value = bytes as f64 / base.powi(exp as i32);

    format!("{:.2} {}", value, units[exp])
}

pub const COLORS: [ratatui::style::Color; 6] = [
    ratatui::style::Color::Magenta,
    ratatui::style::Color::Cyan,
    ratatui::style::Color::Blue,
    ratatui::style::Color::Yellow,
    ratatui::style::Color::Green,
    ratatui::style::Color::Red,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_zero() {
        assert_eq!(format_bytes(0, false), "0.00 B");
        assert_eq!(format_bytes(0, true), "0.00 B");
    }

    #[test]
    fn test_format_bytes_binary() {
        assert_eq!(format_bytes(1024, false), "1.00 KiB");
        assert_eq!(format_bytes(1048576, false), "1.00 MiB");
        assert_eq!(format_bytes(1073741824, false), "1.00 GiB");
        assert_eq!(format_bytes(512, false), "512.00 B");
        assert_eq!(format_bytes(2048, false), "2.00 KiB");
    }

    #[test]
    fn test_format_bytes_decimal() {
        assert_eq!(format_bytes(1000, true), "1.00 KB");
        assert_eq!(format_bytes(1000000, true), "1.00 MB");
        assert_eq!(format_bytes(1000000000, true), "1.00 GB");
        assert_eq!(format_bytes(500, true), "500.00 B");
        assert_eq!(format_bytes(2000, true), "2.00 KB");
    }

    #[test]
    fn test_format_bytes_large() {
        assert_eq!(format_bytes(1099511627776, false), "1.00 TiB");
        assert_eq!(format_bytes(1125899906842624, false), "1.00 PiB");
    }

    #[test]
    fn test_format_bytes_fractional() {
        // 1.5 KiB
        assert_eq!(format_bytes(1536, false), "1.50 KiB");
        // 2.25 MiB
        assert_eq!(format_bytes(2359296, false), "2.25 MiB");
    }

    #[test]
    fn test_colors_array() {
        assert_eq!(COLORS.len(), 6);
        assert_eq!(COLORS[0], ratatui::style::Color::Magenta);
        assert_eq!(COLORS[1], ratatui::style::Color::Cyan);
        assert_eq!(COLORS[5], ratatui::style::Color::Red);
    }
}
