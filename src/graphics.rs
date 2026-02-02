/// Advanced graphics and symbol rendering for rtop
/// Inspired by btop++'s sophisticated graphing capabilities

use std::collections::HashMap;

/// Unicode symbols for drawing boxes and UI elements
pub mod symbols {
    pub const H_LINE: &str = "─";
    pub const V_LINE: &str = "│";
    pub const DOTTED_V_LINE: &str = "╎";
    pub const LEFT_UP: &str = "┌";
    pub const RIGHT_UP: &str = "┐";
    pub const LEFT_DOWN: &str = "└";
    pub const RIGHT_DOWN: &str = "┘";
    pub const ROUND_LEFT_UP: &str = "╭";
    pub const ROUND_RIGHT_UP: &str = "╮";
    pub const ROUND_LEFT_DOWN: &str = "╰";
    pub const ROUND_RIGHT_DOWN: &str = "╯";
    pub const TITLE_LEFT_DOWN: &str = "┘";
    pub const TITLE_RIGHT_DOWN: &str = "└";
    pub const TITLE_LEFT: &str = "┐";
    pub const TITLE_RIGHT: &str = "┌";
    pub const DIV_RIGHT: &str = "┤";
    pub const DIV_LEFT: &str = "├";
    pub const DIV_UP: &str = "┬";
    pub const DIV_DOWN: &str = "┴";
    
    pub const UP: &str = "↑";
    pub const DOWN: &str = "↓";
    pub const LEFT: &str = "←";
    pub const RIGHT: &str = "→";
    pub const ENTER: &str = "↵";
    
    pub const METER: &str = "■";
    
    pub const SUPERSCRIPT: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];
    
    /// Braille patterns for upward-facing graphs (25 symbols for 5x5 resolution)
    pub const BRAILLE_UP: [&str; 25] = [
        " ", "⢀", "⢠", "⢰", "⢸",
        "⡀", "⣀", "⣠", "⣰", "⣸",
        "⡄", "⣄", "⣤", "⣴", "⣼",
        "⡆", "⣆", "⣦", "⣶", "⣾",
        "⡇", "⣇", "⣧", "⣷", "⣿"
    ];
    
    /// Braille patterns for downward-facing graphs
    pub const BRAILLE_DOWN: [&str; 25] = [
        " ", "⠈", "⠘", "⠸", "⢸",
        "⠁", "⠉", "⠙", "⠹", "⢹",
        "⠃", "⠋", "⠛", "⠻", "⢻",
        "⠇", "⠏", "⠟", "⠿", "⢿",
        "⡇", "⡏", "⡟", "⡿", "⣿"
    ];
    
    /// Block patterns for upward-facing graphs
    pub const BLOCK_UP: [&str; 25] = [
        " ", "▗", "▗", "▐", "▐",
        "▖", "▄", "▄", "▟", "▟",
        "▖", "▄", "▄", "▟", "▟",
        "▌", "▙", "▙", "█", "█",
        "▌", "▙", "▙", "█", "█"
    ];
    
    /// Block patterns for downward-facing graphs
    pub const BLOCK_DOWN: [&str; 25] = [
        " ", "▝", "▝", "▐", "▐",
        "▘", "▀", "▀", "▜", "▜",
        "▘", "▀", "▀", "▜", "▜",
        "▌", "▛", "▛", "█", "█",
        "▌", "▛", "▛", "█", "█"
    ];
    
    /// TTY-compatible patterns for upward graphs
    pub const TTY_UP: [&str; 25] = [
        " ", "░", "░", "▒", "▒",
        "░", "░", "▒", "▒", "█",
        "░", "▒", "▒", "▒", "█",
        "▒", "▒", "▒", "█", "█",
        "▒", "█", "█", "█", "█"
    ];
    
    /// TTY-compatible patterns for downward graphs
    pub const TTY_DOWN: [&str; 25] = [
        " ", "░", "░", "▒", "▒",
        "░", "░", "▒", "▒", "█",
        "░", "▒", "▒", "▒", "█",
        "▒", "▒", "▒", "█", "█",
        "▒", "█", "█", "█", "█"
    ];
}

/// Symbol type for graph rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphSymbol {
    Braille,
    Block,
    Tty,
}

impl GraphSymbol {
    pub fn get_symbols(&self, inverted: bool) -> &'static [&'static str; 25] {
        match (self, inverted) {
            (GraphSymbol::Braille, false) => &symbols::BRAILLE_UP,
            (GraphSymbol::Braille, true) => &symbols::BRAILLE_DOWN,
            (GraphSymbol::Block, false) => &symbols::BLOCK_UP,
            (GraphSymbol::Block, true) => &symbols::BLOCK_DOWN,
            (GraphSymbol::Tty, false) => &symbols::TTY_UP,
            (GraphSymbol::Tty, true) => &symbols::TTY_DOWN,
        }
    }
}

/// Advanced graph renderer with braille/unicode support
pub struct GraphRenderer {
    width: usize,
    height: usize,
    symbol: GraphSymbol,
    inverted: bool,
    no_zero: bool,
}

impl GraphRenderer {
    pub fn new(width: usize, height: usize, symbol: GraphSymbol, inverted: bool) -> Self {
        Self {
            width,
            height,
            symbol,
            inverted,
            no_zero: true,
        }
    }
    
    /// Render a graph from data points (0-100 scale)
    /// Returns a vector of strings, one per height level
    pub fn render(&self, data: &[f64]) -> Vec<String> {
        let mut output = vec![String::new(); self.height];
        let symbols = self.symbol.get_symbols(self.inverted);
        
        // Calculate how many data points fit in the width
        let data_len = data.len();
        let data_offset = if data_len > self.width * 2 {
            data_len - self.width * 2
        } else {
            0
        };
        
        let mut last_value = if data_offset > 0 { data[data_offset - 1] } else { 0.0 };
        
        // Process data points in pairs for braille/block rendering
        for i in (data_offset..data_len).step_by(2) {
            let v1 = data[i].clamp(0.0, 100.0);
            let v2 = if i + 1 < data_len { data[i + 1].clamp(0.0, 100.0) } else { v1 };
            
            // For each height level
            for h in 0..self.height {
                let cur_high = if self.height > 1 {
                    100.0 * (self.height - h) as f64 / self.height as f64
                } else {
                    100.0
                };
                let cur_low = if self.height > 1 {
                    100.0 * (self.height - (h + 1)) as f64 / self.height as f64
                } else {
                    0.0
                };
                
                // Calculate symbol indices for both values
                let idx1 = self.value_to_symbol_index(v1, cur_low, cur_high);
                let idx2 = self.value_to_symbol_index(v2, cur_low, cur_high);
                
                // Combine into 5x5 symbol grid index
                let symbol_idx = (idx1 * 5 + idx2).min(24);
                output[h].push_str(symbols[symbol_idx]);
            }
            
            last_value = v2;
        }
        
        output
    }
    
    fn value_to_symbol_index(&self, value: f64, cur_low: f64, cur_high: f64) -> usize {
        let clamp_min = if self.no_zero && value > 0.0 { 1 } else { 0 };
        
        if value >= cur_high {
            4
        } else if value <= cur_low {
            clamp_min
        } else {
            let range = cur_high - cur_low;
            let normalized = (value - cur_low) / range * 4.0;
            (normalized.round() as usize).clamp(clamp_min, 4)
        }
    }
}

/// Meter/gauge renderer with color gradients
pub struct MeterRenderer {
    width: usize,
}

impl MeterRenderer {
    pub fn new(width: usize) -> Self {
        Self { width }
    }
    
    /// Render a horizontal meter bar (0-100)
    pub fn render(&self, value: u8) -> String {
        let value = value.min(100);
        let filled = (self.width * value as usize) / 100;
        
        symbols::METER.repeat(filled)
    }
    
    /// Render with segments for more detailed view
    pub fn render_segmented(&self, value: u8) -> String {
        let value = value.min(100);
        let segments = self.width;
        let filled = (segments * value as usize) / 100;
        
        let mut output = String::new();
        for i in 0..segments {
            if i < filled {
                output.push_str(symbols::METER);
            } else {
                output.push_str("░");
            }
        }
        output
    }
}

/// Box drawing utilities
pub struct BoxDrawer {
    pub rounded_corners: bool,
    pub double_lines: bool,
}

impl BoxDrawer {
    pub fn new(rounded_corners: bool) -> Self {
        Self {
            rounded_corners,
            double_lines: false,
        }
    }
    
    pub fn draw_box(&self, width: usize, height: usize, title: Option<&str>) -> Vec<String> {
        let mut lines = Vec::new();
        
        let (lu, ru, ld, rd) = if self.rounded_corners {
            (symbols::ROUND_LEFT_UP, symbols::ROUND_RIGHT_UP, 
             symbols::ROUND_LEFT_DOWN, symbols::ROUND_RIGHT_DOWN)
        } else {
            (symbols::LEFT_UP, symbols::RIGHT_UP, 
             symbols::LEFT_DOWN, symbols::RIGHT_DOWN)
        };
        
        // Top line with optional title
        let mut top = String::from(lu);
        if let Some(t) = title {
            let title_len = t.len().min(width - 4);
            top.push_str(symbols::H_LINE);
            top.push_str(&t[..title_len]);
            top.push_str(symbols::H_LINE);
            let remaining = width.saturating_sub(title_len + 4);
            top.push_str(&symbols::H_LINE.repeat(remaining));
        } else {
            top.push_str(&symbols::H_LINE.repeat(width - 2));
        }
        top.push_str(ru);
        lines.push(top);
        
        // Middle lines
        for _ in 1..height - 1 {
            let mut middle = String::from(symbols::V_LINE);
            middle.push_str(&" ".repeat(width - 2));
            middle.push_str(symbols::V_LINE);
            lines.push(middle);
        }
        
        // Bottom line
        let mut bottom = String::from(ld);
        bottom.push_str(&symbols::H_LINE.repeat(width - 2));
        bottom.push_str(rd);
        lines.push(bottom);
        
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_graph_renderer() {
        let renderer = GraphRenderer::new(20, 4, GraphSymbol::Braille, false);
        let data: Vec<f64> = (0..40).map(|i| ((i as f64 * 2.5).sin() * 50.0 + 50.0)).collect();
        let graph = renderer.render(&data);
        assert_eq!(graph.len(), 4);
    }
    
    #[test]
    fn test_meter_renderer() {
        let meter = MeterRenderer::new(10);
        let result = meter.render(50);
        assert_eq!(result.len(), 5 * symbols::METER.len());
    }
    
    #[test]
    fn test_box_drawer() {
        let drawer = BoxDrawer::new(true);
        let boxx = drawer.draw_box(20, 5, Some("Test"));
        assert_eq!(boxx.len(), 5);
        assert!(boxx[0].contains("Test"));
    }
}
