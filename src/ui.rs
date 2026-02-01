use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph, Row, Table,
    },
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use crate::monitor::*;
use crate::utils::{format_bytes, COLORS};

pub struct App {
    cpu_monitor: CpuMonitor,
    memory_monitor: MemoryMonitor,
    network_monitor: NetworkMonitor,
    disk_monitor: DiskMonitor,
    process_monitor: ProcessMonitor,
    last_update: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            cpu_monitor: CpuMonitor::new(),
            memory_monitor: MemoryMonitor::new(),
            network_monitor: NetworkMonitor::new(),
            disk_monitor: DiskMonitor::new(),
            process_monitor: ProcessMonitor::new(),
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(1000) {
            self.cpu_monitor.update();
            self.memory_monitor.update();
            self.network_monitor.update();
            self.disk_monitor.update();
            self.process_monitor.update();
            self.last_update = now;
        }
    }

    pub fn handle_input(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true)
                    }
                    KeyCode::Char('p') => {
                        self.process_monitor.set_sort_order(SortOrder::Pid);
                    }
                    KeyCode::Char('c') => {
                        self.process_monitor.set_sort_order(SortOrder::Cpu);
                    }
                    KeyCode::Char('m') => {
                        self.process_monitor.set_sort_order(SortOrder::Memory);
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    pub fn draw(&self, frame: &mut Frame) {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),    // Header
                Constraint::Min(0),       // Content
            ])
            .split(frame.area());

        // Draw header
        self.draw_header(frame, main_chunks[0]);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(main_chunks[1]);

        // Top section: CPU
        self.draw_cpu(frame, chunks[0]);

        // Middle section: Memory and Network
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
            .split(chunks[1]);

        self.draw_memory(frame, middle_chunks[0]);
        self.draw_memory_gauges(frame, middle_chunks[1]);

        // Bottom section: Network, Disk, and Processes
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(bottom_chunks[0]);

        self.draw_network(frame, left_chunks[0]);
        self.draw_disk(frame, left_chunks[1]);
        self.draw_processes(frame, bottom_chunks[1]);
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let title = vec![
            Line::from(vec![
                Span::styled(" ▓▓ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("mytop", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(" v2.0 ", Style::default().fg(Color::DarkGray)),
                Span::raw("  │  "),
                Span::styled("◆", Style::default().fg(Color::Green)),
                Span::raw(" System Monitor  "),
                Span::styled("│", Style::default().fg(Color::DarkGray)),
                Span::raw("  Press "),
                Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" to quit"),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Cyan));

        let paragraph = Paragraph::new(title)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    fn draw_cpu(&self, frame: &mut Frame, area: Rect) {
        let cpu_data = self.cpu_monitor.get_all_cpu_data();
        
        // Collect all data points first to ensure proper lifetimes
        let all_data: Vec<Vec<(f64, f64)>> = cpu_data
            .iter()
            .map(|(_, _, history)| {
                history
                    .iter()
                    .enumerate()
                    .map(|(x, &y)| (x as f64, y as f64))
                    .collect()
            })
            .collect();
        
        let datasets: Vec<Dataset> = cpu_data
            .iter()
            .zip(all_data.iter())
            .enumerate()
            .map(|(i, ((name, usage, _), data))| {
                // Couleur dynamique selon l'utilisation
                let color = if *usage > 80.0 {
                    Color::Red
                } else if *usage > 60.0 {
                    Color::Yellow
                } else {
                    COLORS[i % COLORS.len()]
                };

                Dataset::default()
                    .name(name.clone())
                    .marker(symbols::Marker::Braille)
                    .graph_type(ratatui::widgets::GraphType::Line)
                    .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                    .data(data)
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
                        Span::styled("CPU Usage", Style::default().add_modifier(Modifier::BOLD)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .x_axis(
                Axis::default()
                    .title(Span::styled("← Time (seconds)", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60", Style::default().fg(Color::DarkGray)),
                        Span::styled("30", Style::default().fg(Color::DarkGray)),
                        Span::styled("0", Style::default().fg(Color::White)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Usage %", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("0", Style::default().fg(Color::Green)),
                        Span::styled("50", Style::default().fg(Color::Yellow)),
                        Span::styled("100", Style::default().fg(Color::Red)),
                    ]),
            )
            .legend_position(Some(ratatui::widgets::LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)));

        frame.render_widget(chart, area);
    }

    fn draw_memory(&self, frame: &mut Frame, area: Rect) {
        let (mem_percent, mem_history, _, _) = self.memory_monitor.get_memory_data();
        let (swap_percent, swap_history, _, _) = self.memory_monitor.get_swap_data();

        let mem_data: Vec<(f64, f64)> = mem_history
            .iter()
            .enumerate()
            .map(|(x, &y)| (x as f64, y as f64))
            .collect();

        let swap_data: Vec<(f64, f64)> = swap_history
            .iter()
            .enumerate()
            .map(|(x, &y)| (x as f64, y as f64))
            .collect();

        let mem_color = if mem_percent > 85.0 {
            Color::Red
        } else if mem_percent > 70.0 {
            Color::Yellow
        } else {
            COLORS[0]
        };

        let swap_color = if swap_percent > 85.0 {
            Color::Red
        } else if swap_percent > 70.0 {
            Color::Yellow
        } else {
            COLORS[1]
        };

        let datasets = vec![
            Dataset::default()
                .name(format!("RAM {:.1}%", mem_percent))
                .marker(symbols::Marker::Braille)
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().fg(mem_color).add_modifier(Modifier::BOLD))
                .data(&mem_data),
            Dataset::default()
                .name(format!("Swap {:.1}%", swap_percent))
                .marker(symbols::Marker::Braille)
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().fg(swap_color).add_modifier(Modifier::BOLD))
                .data(&swap_data),
        ];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled("💾 ", Style::default().fg(Color::Blue)),
                        Span::styled("Memory Usage", Style::default().add_modifier(Modifier::BOLD)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .x_axis(
                Axis::default()
                    .title(Span::styled("← Time", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60", Style::default().fg(Color::DarkGray)),
                        Span::styled("30", Style::default().fg(Color::DarkGray)),
                        Span::styled("0", Style::default().fg(Color::White)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Usage %", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("0", Style::default().fg(Color::Green)),
                        Span::styled("50", Style::default().fg(Color::Yellow)),
                        Span::styled("100", Style::default().fg(Color::Red)),
                    ]),
            )
            .legend_position(Some(ratatui::widgets::LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)));

        frame.render_widget(chart, area);
    }

    fn draw_memory_gauges(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let (mem_percent, _, mem_used, mem_total) = self.memory_monitor.get_memory_data();
        let mem_text = self.create_circular_gauge("Memory", mem_percent, mem_used, mem_total, COLORS[0]);
        frame.render_widget(mem_text, chunks[0]);

        let (swap_percent, _, swap_used, swap_total) = self.memory_monitor.get_swap_data();
        let swap_text = self.create_circular_gauge("Swap", swap_percent, swap_used, swap_total, COLORS[1]);
        frame.render_widget(swap_text, chunks[1]);
    }

    fn create_circular_gauge<'a>(&self, title: &'a str, percent: f32, used: u64, total: u64, base_color: Color) -> Paragraph<'a> {
        let color = if percent > 90.0 {
            Color::Red
        } else if percent > 70.0 {
            Color::Yellow
        } else {
            base_color
        };

        // Créer une barre visuelle plus élégante
        let bar_length = 22;
        let filled = ((percent / 100.0 * bar_length as f32) as usize).min(bar_length);
        
        // Utiliser différents caractères pour un effet de dégradé
        let mut bar = String::new();
        for i in 0..bar_length {
            if i < filled {
                bar.push('█');
            } else if i == filled {
                bar.push('▓');
            } else {
                bar.push('░');
            }
        }

        // Icône selon le type
        let icon = match title {
            "Memory" => "▓",
            "Swap" => "▒",
            _ => "■",
        };

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!(" {}  ", icon),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    title,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  ["),
                Span::styled(bar, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::raw("]"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    format!("{:.1}%", percent),
                    Style::default().fg(color).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
                ),
                Span::raw(" used"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format_bytes(used, true),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" / ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ];

        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .alignment(ratatui::layout::Alignment::Left)
    }

    fn draw_network(&self, frame: &mut Frame, area: Rect) {
        let (_, _, rx_sec, tx_sec, total_rx, total_tx) = self.network_monitor.get_network_data();

        // Indicateurs d'activité
        let rx_indicator = if rx_sec > 1000000 { "●" } else if rx_sec > 10000 { "◐" } else { "○" };
        let tx_indicator = if tx_sec > 1000000 { "●" } else if tx_sec > 10000 { "◐" } else { "○" };

        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ▼ ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::styled("Download ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(rx_indicator, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format_bytes(rx_sec, false),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                ),
                Span::styled("/s", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::raw("    Total: "),
                Span::styled(
                    format_bytes(total_rx, false),
                    Style::default().fg(Color::Gray),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ▲ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("Upload ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(tx_indicator, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format_bytes(tx_sec, false),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled("/s", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::raw("    Total: "),
                Span::styled(
                    format_bytes(total_tx, false),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title(vec![
                    Span::styled("🌐 ", Style::default().fg(Color::Cyan)),
                    Span::styled("Network", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        frame.render_widget(paragraph, area);
    }

    fn draw_disk(&self, frame: &mut Frame, area: Rect) {
        let (percent, used, total) = self.disk_monitor.get_disk_data();

        let disk_color = if percent > 90.0 {
            Color::Red
        } else if percent > 80.0 {
            Color::Yellow
        } else {
            COLORS[5]
        };

        // Créer une barre horizontale élégante
        let bar_length = 32;
        let filled = ((percent / 100.0 * bar_length as f32) as usize).min(bar_length);
        let mut bar = String::new();
        for i in 0..bar_length {
            if i < filled {
                bar.push('█');
            } else if i == filled {
                bar.push('▓');
            } else {
                bar.push('░');
            }
        }

        let available = total.saturating_sub(used);
        
        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  ["),
                Span::styled(bar, Style::default().fg(disk_color).add_modifier(Modifier::BOLD)),
                Span::raw("]"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    format!("{:.1}%", percent),
                    Style::default().fg(disk_color).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED),
                ),
                Span::raw(" used"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ● ", Style::default().fg(disk_color).add_modifier(Modifier::BOLD)),
                Span::raw("Used:  "),
                Span::styled(
                    format_bytes(used, true),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(Color::DarkGray)),
                Span::raw("Free:  "),
                Span::styled(
                    format_bytes(available, true),
                    Style::default().fg(Color::Gray),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ─ ", Style::default().fg(Color::Gray)),
                Span::raw("Total: "),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .title(vec![
                    Span::styled("💿 ", Style::default().fg(Color::Magenta)),
                    Span::styled("Storage", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        frame.render_widget(paragraph, area);
    }

    fn draw_processes(&self, frame: &mut Frame, area: Rect) {
        let processes = self.process_monitor.get_sorted_processes();
        let rows: Vec<Row> = processes
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, p)| {
                let cpu_color = if p.cpu_usage > 50.0 {
                    Color::Red
                } else if p.cpu_usage > 25.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                let style = if i % 2 == 0 {
                    Style::default()
                } else {
                    Style::default().bg(Color::Rgb(20, 20, 30))
                };

                Row::new(vec![
                    p.pid.to_string(),
                    p.name.chars().take(24).collect::<String>(),
                    format!("{:.1}%", p.cpu_usage),
                    format_bytes(p.memory, false),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(7),
                Constraint::Length(24),
                Constraint::Length(7),
                Constraint::Length(10),
            ],
        )
        .header(
            Row::new(vec!["PID", "Process", "CPU", "Memory"])
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                ),
        )
        .block(
            Block::default()
                .title(vec![
                    Span::styled("⚙ ", Style::default().fg(Color::Yellow)),
                    Span::styled("Processes ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("[", Style::default().fg(Color::DarkGray)),
                    Span::styled("p", Style::default().fg(Color::Yellow)),
                    Span::styled("]", Style::default().fg(Color::DarkGray)),
                    Span::raw("PID "),
                    Span::styled("[", Style::default().fg(Color::DarkGray)),
                    Span::styled("c", Style::default().fg(Color::Yellow)),
                    Span::styled("]", Style::default().fg(Color::DarkGray)),
                    Span::raw("CPU "),
                    Span::styled("[", Style::default().fg(Color::DarkGray)),
                    Span::styled("m", Style::default().fg(Color::Yellow)),
                    Span::styled("]", Style::default().fg(Color::DarkGray)),
                    Span::raw("Mem"),
                ])
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 80))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

        frame.render_widget(table, area);
    }
}
