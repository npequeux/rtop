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
    fn test_format_bytes() {
        assert_eq!(format_bytes(0, false), "0.00 B");
        assert_eq!(format_bytes(1024, false), "1.00 KiB");
        assert_eq!(format_bytes(1048576, false), "1.00 MiB");
        assert_eq!(format_bytes(1000, true), "1.00 KB");
    }
}
