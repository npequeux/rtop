use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, Paragraph, Row, Table,
    },
    Frame,
};
use std::io;
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::export::*;
use crate::monitor::*;
use crate::utils::{format_bytes, COLORS};

pub struct App {
    cpu_monitor: CpuMonitor,
    memory_monitor: MemoryMonitor,
    network_monitor: NetworkMonitor,
    disk_monitor: DiskMonitor,
    process_monitor: ProcessMonitor,
    temp_monitor: TempMonitor,
    system_monitor: SystemMonitor,
    last_update: Instant,
    config: Config,
    show_help: bool,
    paused: bool,
    process_filter: String,
    color_enabled: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            cpu_monitor: CpuMonitor::new(),
            memory_monitor: MemoryMonitor::new(),
            network_monitor: NetworkMonitor::new(),
            disk_monitor: DiskMonitor::new(),
            process_monitor: ProcessMonitor::new(),
            temp_monitor: TempMonitor::new(),
            system_monitor: SystemMonitor::new(),
            last_update: Instant::now(),
            config,
            show_help: false,
            paused: false,
            process_filter: String::new(),
            color_enabled: true,
        }
    }

    pub fn set_minimal_mode(&mut self, minimal: bool) {
        if minimal {
            self.config.refresh_rates.cpu = 2000;
            self.config.refresh_rates.memory = 2000;
            self.config.refresh_rates.disk = 5000;
            self.config.refresh_rates.process = 5000;
        }
    }

    pub fn set_color_mode(&mut self, enabled: bool) {
        self.color_enabled = enabled;
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        if elapsed >= self.config.cpu_refresh_duration() {
            self.cpu_monitor.update();
            self.memory_monitor.update();
            self.network_monitor.update();
            self.temp_monitor.update();
            self.system_monitor.update();
            
            // Less frequent updates for disk and processes
            if elapsed >= self.config.disk_refresh_duration() {
                self.disk_monitor.update();
            }
            if elapsed >= self.config.process_refresh_duration() {
                self.process_monitor.update();
            }
            
            self.last_update = now;
        }
    }

    pub fn handle_input(&mut self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Handle help overlay first
                if self.show_help {
                    self.show_help = false;
                    return Ok(false);
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true)
                    }
                    KeyCode::Char('h') | KeyCode::F(1) => {
                        self.show_help = !self.show_help;
                    }
                    KeyCode::Char(' ') => {
                        self.paused = !self.paused;
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
                    KeyCode::Char('/') => {
                        // Start process search mode (simplified - just toggle for now)
                        self.process_filter.clear();
                    }
                    KeyCode::Backspace if !self.process_filter.is_empty() => {
                        self.process_filter.pop();
                    }
                    KeyCode::Char(c) if !self.process_filter.is_empty() || c.is_alphanumeric() => {
                        if self.process_filter.len() < 20 {
                            self.process_filter.push(c);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    pub fn collect_metrics(&self) -> Metrics {
        let timestamp = chrono::Local::now().to_rfc3339();
        
        let cpu_data = self.cpu_monitor.get_all_cpu_data();
        let cores: Vec<CoreMetric> = cpu_data
            .iter()
            .enumerate()
            .map(|(i, (_, usage, _))| CoreMetric {
                id: i,
                usage: *usage,
            })
            .collect();
        let cpu_avg = cores.iter().map(|c| c.usage).sum::<f32>() / cores.len() as f32;

        let (mem_percent, _, mem_used, mem_total) = self.memory_monitor.get_memory_data();
        let (swap_percent, _, swap_used, swap_total) = self.memory_monitor.get_swap_data();

        let (_, _, rx_rate, tx_rate, total_rx, total_tx) = self.network_monitor.get_network_data();

        let (disk_percent, disk_used, disk_total) = self.disk_monitor.get_disk_data();

        let processes = self.process_monitor.get_sorted_processes();

        let temp_data = self.temp_monitor.get_temperature_data();
        let temperature = if !temp_data.is_empty() {
            Some(TempMetrics {
                sensors: temp_data
                    .iter()
                    .map(|(name, temp, _)| SensorMetric {
                        name: name.clone(),
                        temperature: *temp,
                    })
                    .collect(),
                average: temp_data.iter().map(|(_, t, _)| t).sum::<f32>() / temp_data.len() as f32,
                max: temp_data.iter().map(|(_, t, _)| *t).fold(0.0, f32::max),
            })
        } else {
            None
        };

        Metrics {
            timestamp,
            cpu: CpuMetrics {
                cores,
                average: cpu_avg,
            },
            memory: MemoryMetrics {
                total: mem_total,
                used: mem_used,
                available: mem_total - mem_used,
                percent: mem_percent,
                swap_total,
                swap_used,
                swap_percent,
            },
            network: NetworkMetrics {
                received: total_rx,
                transmitted: total_tx,
                rx_rate: rx_rate as f64,
                tx_rate: tx_rate as f64,
            },
            disk: vec![DiskMetrics {
                name: "root".to_string(),
                mount_point: "/".to_string(),
                total: disk_total,
                available: disk_total - disk_used,
                percent: disk_percent,
            }],
            processes: processes
                .iter()
                .take(20)
                .map(|p| ProcessMetrics {
                    pid: p.pid,
                    name: p.name.clone(),
                    cpu: p.cpu_usage,
                    memory: p.memory,
                    memory_percent: (p.memory as f32 / mem_total as f32) * 100.0,
                })
                .collect(),
            temperature,
            system: SystemMetrics {
                hostname: self.system_monitor.hostname(),
                os: self.system_monitor.os_version(),
                kernel: self.system_monitor.kernel_version(),
                uptime: self.system_monitor.uptime(),
                load_average: self.system_monitor.load_average(),
            },
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        // Apply blue background to entire frame
        let full_area = frame.area();
        let background = Block::default()
            .style(Style::default().bg(Color::Rgb(10, 20, 40)));
        frame.render_widget(background, full_area);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),    // Header
                Constraint::Min(0),       // Content
                Constraint::Length(2),    // Footer/Status bar
            ])
            .split(frame.area());

        // Draw header
        self.draw_header(frame, main_chunks[0]);

        // Draw footer/status bar
        self.draw_footer(frame, main_chunks[2]);

        // Adjust layout based on temperature sensor availability
        let has_temp = self.temp_monitor.has_temperature_sensors();
        
        let chunks = if has_temp {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),  // CPU
                    Constraint::Percentage(25),  // Memory
                    Constraint::Percentage(25),  // Temperature
                    Constraint::Percentage(25),  // Bottom section
                ])
                .split(main_chunks[1])
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ])
                .split(main_chunks[1])
        };

        // Top section: CPU
        self.draw_cpu(frame, chunks[0]);

        // Middle section: Memory and Network
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
            .split(chunks[1]);

        self.draw_memory(frame, middle_chunks[0]);
        self.draw_memory_gauges(frame, middle_chunks[1]);

        // Temperature section (if available)
        if has_temp {
            self.draw_temperature(frame, chunks[2]);
        }

        // Bottom section: Network, Disk, and Processes
        let bottom_idx = if has_temp { 3 } else { 2 };
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[bottom_idx]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(bottom_chunks[0]);

        self.draw_network(frame, left_chunks[0]);
        self.draw_disk(frame, left_chunks[1]);
        self.draw_processes(frame, bottom_chunks[1]);

        // Draw help overlay if activated
        if self.show_help {
            self.draw_help_overlay(frame, frame.area());
        }
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let title = vec![
            Line::from(vec![
                Span::styled(" ▓▓ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("rtop", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
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
        
        // Pre-allocate with known capacity to avoid reallocations
        let mut all_data: Vec<Vec<(f64, f64)>> = Vec::with_capacity(cpu_data.len());
        
        for (_, _, history) in &cpu_data {
            let mut data = Vec::with_capacity(history.len());
            for (x, &y) in history.iter().enumerate() {
                data.push((x as f64, y as f64));
            }
            all_data.push(data);
        }
        
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

        let mut mem_data = Vec::with_capacity(mem_history.len());
        for (x, &y) in mem_history.iter().enumerate() {
            mem_data.push((x as f64, y as f64));
        }

        let mut swap_data = Vec::with_capacity(swap_history.len());
        for (x, &y) in swap_history.iter().enumerate() {
            swap_data.push((x as f64, y as f64));
        }

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
        let mut rows = Vec::with_capacity(20.min(processes.len()));
        
        for (i, p) in processes.iter().take(20).enumerate() {
                let _cpu_color = if p.cpu_usage > 50.0 {
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

            let row = Row::new(vec![
                p.pid.to_string(),
                p.name.chars().take(24).collect::<String>(),
                format!("{:.1}%", p.cpu_usage),
                format_bytes(p.memory, false),
            ])
            .style(style);
            
            rows.push(row);
        }

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

    fn draw_temperature(&self, frame: &mut Frame, area: Rect) {
        let temp_data = self.temp_monitor.get_temperature_data();
        
        // If no temperature data available, show a message
        if !self.temp_monitor.has_temperature_sensors() || temp_data.is_empty() {
            let text = vec![
                Line::from(""),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ⚠ ", Style::default().fg(Color::Yellow)),
                    Span::styled("No temperature sensors detected", Style::default().fg(Color::Gray)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("    Sensors may not be available on this system"),
                ]),
                Line::from(vec![
                    Span::raw("    or may require additional kernel modules"),
                ]),
            ];

            let paragraph = Paragraph::new(text).block(
                Block::default()
                    .title(vec![
                        Span::styled("🌡 ", Style::default().fg(Color::DarkGray)),
                        Span::styled("Temperature ", Style::default().add_modifier(Modifier::BOLD).fg(Color::DarkGray)),
                        Span::styled("(unavailable)", Style::default().fg(Color::DarkGray)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            );

            frame.render_widget(paragraph, area);
            return;
        }

        // Split area: graph on left, list on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(area);
        
        // Prepare datasets for each temperature sensor
        let all_data: Vec<Vec<(f64, f64)>> = temp_data
            .iter()
            .map(|(_, _, history)| {
                history
                    .iter()
                    .enumerate()
                    .map(|(x, &y)| (x as f64, y as f64))
                    .collect()
            })
            .collect();
        
        let datasets: Vec<Dataset> = temp_data
            .iter()
            .zip(all_data.iter())
            .enumerate()
            .map(|(_i, ((label, temp, _), data))| {
                // Determine color based on temperature
                let temp_color = if *temp > 80.0 {
                    Color::Red
                } else if *temp > 65.0 {
                    Color::Yellow
                } else if *temp > 50.0 {
                    Color::Green
                } else {
                    Color::Cyan
                };

                // Shorten label if too long
                let short_label = if label.len() > 20 {
                    format!("{}.. {:.1}°C", &label[..17], temp)
                } else {
                    format!("{}: {:.1}°C", label, temp)
                };

                Dataset::default()
                    .name(short_label)
                    .marker(symbols::Marker::Braille)
                    .graph_type(ratatui::widgets::GraphType::Line)
                    .style(Style::default().fg(temp_color).add_modifier(Modifier::BOLD))
                    .data(data)
            })
            .collect();

        // Determine Y-axis bounds dynamically
        let max_temp = self.temp_monitor.get_max_temp();
        let y_max = ((max_temp / 10.0).ceil() * 10.0).max(100.0) as f64;

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled("🌡 ", Style::default().fg(Color::Red)),
                        Span::styled("Temperature History", Style::default().add_modifier(Modifier::BOLD)),
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
                    .title(Span::styled("°C", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, y_max])
                    .labels(vec![
                        Span::styled("0", Style::default().fg(Color::Cyan)),
                        Span::styled(format!("{:.0}", y_max / 2.0), Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{:.0}", y_max), Style::default().fg(Color::Red)),
                    ]),
            )
            .legend_position(Some(ratatui::widgets::LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 5), Constraint::Ratio(1, 5)));

        frame.render_widget(chart, chunks[0]);

        // Draw temperature list on the right
        let mut lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(" Current Temperatures", Style::default().fg(Color::White).add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
        ];

        for (label, temp, _) in temp_data.iter() {
            let temp_color = if *temp > 80.0 {
                Color::Red
            } else if *temp > 65.0 {
                Color::Yellow
            } else if *temp > 50.0 {
                Color::Green
            } else {
                Color::Cyan
            };

            let icon = if *temp > 80.0 {
                "🔥"
            } else if *temp > 65.0 {
                "🌡"
            } else {
                "❄"
            };

            // Truncate long labels
            let display_label = if label.len() > 18 {
                format!("{}...", &label[..15])
            } else {
                label.clone()
            };

            lines.push(Line::from(vec![
                Span::styled(format!(" {} ", icon), Style::default().fg(temp_color)),
                Span::styled(
                    format!("{:.1}°C", temp),
                    Style::default().fg(temp_color).add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(display_label, Style::default().fg(Color::Gray)),
            ]));
            lines.push(Line::from(""));
        }

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .title(vec![
                    Span::styled("📊 ", Style::default().fg(Color::Cyan)),
                    Span::styled("Sensors", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        frame.render_widget(paragraph, chunks[1]);
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let (load_1, load_5, load_15) = self.system_monitor.load_average();
        
        let status = if self.paused {
            Span::styled(" ⏸ PAUSED ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD).bg(Color::Rgb(60, 60, 0)))
        } else {
            Span::styled(" ▶ RUNNING ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        };

        let footer_text = vec![
            Line::from(vec![
                status,
                Span::raw(" │ "),
                Span::styled("Uptime: ", Style::default().fg(Color::DarkGray)),
                Span::styled(self.system_monitor.uptime_formatted(), Style::default().fg(Color::Cyan)),
                Span::raw(" │ "),
                Span::styled("Load: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.2} {:.2} {:.2}", load_1, load_5, load_15), Style::default().fg(Color::White)),
                Span::raw(" │ "),
                Span::styled("Processes: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}", self.system_monitor.total_processes()), Style::default().fg(Color::White)),
                Span::raw(" │ Press "),
                Span::styled("h", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" for help"),
            ]),
        ];

        let paragraph = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::DarkGray)))
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    fn draw_help_overlay(&self, frame: &mut Frame, area: Rect) {
        // Create centered popup
        let popup_area = Self::centered_rect(60, 70, area);

        // Clear the popup area
        let clear_block = Block::default()
            .style(Style::default().bg(Color::Rgb(20, 30, 50)));
        frame.render_widget(clear_block, popup_area);

        let help_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("                   ⚡ rtop - Help ⚡", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Navigation & Control:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("    q, Esc, Ctrl+C  ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Quit application"),
            ]),
            Line::from(vec![
                Span::styled("    h, F1           ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Toggle this help screen"),
            ]),
            Line::from(vec![
                Span::styled("    Space           ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Pause/Resume updates"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Process Sorting:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("    p               ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Sort by PID"),
            ]),
            Line::from(vec![
                Span::styled("    c               ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Sort by CPU usage"),
            ]),
            Line::from(vec![
                Span::styled("    m               ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Sort by Memory usage"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Command Line Options:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("rtop --help            ", Style::default().fg(Color::Cyan)),
                Span::raw("→ Show all options"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("rtop --export out.json ", Style::default().fg(Color::Cyan)),
                Span::raw("→ Export metrics"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("rtop --minimal         ", Style::default().fg(Color::Cyan)),
                Span::raw("→ Minimal mode"),
            ]),
            Line::from(vec![
                Span::raw("    "),
                Span::styled("rtop --generate-config ", Style::default().fg(Color::Cyan)),
                Span::raw("→ Create config"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Features:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("    • Real-time CPU, Memory, Network, Disk monitoring"),
            ]),
            Line::from(vec![
                Span::raw("    • Temperature sensors (auto-detected)"),
            ]),
            Line::from(vec![
                Span::raw("    • Process management with sorting"),
            ]),
            Line::from(vec![
                Span::raw("    • System info: uptime, load average"),
            ]),
            Line::from(vec![
                Span::raw("    • Export to JSON/CSV formats"),
            ]),
            Line::from(vec![
                Span::raw("    • Configurable refresh rates and thresholds"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Config: "),
                Span::styled("~/.config/rtop/config.toml", Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Press any key to close this help", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)),
            ]),
            Line::from(""),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled(" ❓ Help ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .border_type(ratatui::widgets::BorderType::Thick)
                    .style(Style::default().bg(Color::Rgb(20, 30, 50))),
            )
            .alignment(ratatui::layout::Alignment::Left);

        frame.render_widget(help_paragraph, popup_area);
    }

    // Helper function to create centered rectangle
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
