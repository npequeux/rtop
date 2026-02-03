use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
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
use regex::Regex;
use std::io;
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::export::*;
use crate::monitor::*;
use crate::utils::{format_bytes, COLORS};
use crate::graphics::{GraphRenderer, GraphSymbol, MeterRenderer, BoxDrawer};
use crate::theme::{ThemeManager, Theme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewPage {
    Overview,
    Processes,
    Network,
    Storage,
}

pub struct App {
    cpu_monitor: CpuMonitor,
    memory_monitor: MemoryMonitor,
    network_monitor: NetworkMonitor,
    disk_monitor: DiskMonitor,
    process_monitor: ProcessMonitor,
    temp_monitor: TempMonitor,
    system_monitor: SystemMonitor,
    battery_monitor: BatteryMonitor,
    diskio_monitor: DiskIOMonitor,
    gpu_monitor: GpuMonitor,
    theme_manager: ThemeManager,
    last_update: Instant,
    config: Config,
    show_help: bool,
    paused: bool,
    process_filter: String,
    process_filter_regex: Option<Regex>,
    color_enabled: bool,
    current_page: ViewPage,
    process_scroll: usize,
    process_selected: Option<usize>,
    show_kill_confirm: bool,
    show_signal_menu: bool,
    mouse_enabled: bool,
    graph_symbol: GraphSymbol,
    rounded_corners: bool,
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
            battery_monitor: BatteryMonitor::new(),
            diskio_monitor: DiskIOMonitor::new(),
            gpu_monitor: GpuMonitor::new(),
            theme_manager: ThemeManager::new(),
            last_update: Instant::now(),
            config,
            show_help: false,
            paused: false,
            process_filter: String::new(),
            process_filter_regex: None,
            color_enabled: true,
            current_page: ViewPage::Overview,
            process_scroll: 0,
            process_selected: None,
            show_kill_confirm: false,
            show_signal_menu: false,
            mouse_enabled: true,
            graph_symbol: GraphSymbol::Braille,
            rounded_corners: true,
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
            self.network_monitor.update_ping();
            self.temp_monitor.update();
            self.system_monitor.update();
            self.battery_monitor.update();
            self.diskio_monitor.update();
            
            // Update GPU if available
            if self.gpu_monitor.is_enabled() {
                self.gpu_monitor.update();
            }
            
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
            match event::read()? {
                Event::Key(key) => {
                    // Handle kill confirmation first
                    if self.show_kill_confirm {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => {
                                self.kill_selected_process();
                                self.show_kill_confirm = false;
                            }
                            _ => {
                                self.show_kill_confirm = false;
                            }
                        }
                        return Ok(false);
                    }

                    // Handle help overlay
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
                        KeyCode::Char('k') if self.process_selected.is_some() => {
                            self.show_kill_confirm = true;
                        }
                        KeyCode::Char('/') => {
                            self.process_filter.clear();
                            self.process_filter_regex = None;
                        }
                        KeyCode::Backspace if !self.process_filter.is_empty() => {
                            self.process_filter.pop();
                            self.update_filter_regex();
                        }
                        KeyCode::Char(c) if c.is_alphanumeric() || c == '-' || c == '_' => {
                            if self.process_filter.len() < 30 {
                                self.process_filter.push(c);
                                self.update_filter_regex();
                            }
                        }
                        // Page navigation
                        KeyCode::F(2) => self.current_page = ViewPage::Overview,
                        KeyCode::F(3) => self.current_page = ViewPage::Processes,
                        KeyCode::F(4) => self.current_page = ViewPage::Network,
                        KeyCode::F(5) => self.current_page = ViewPage::Storage,
                        // Scroll process list
                        KeyCode::Up => {
                            if self.process_scroll > 0 {
                                self.process_scroll -= 1;
                            }
                        }
                        KeyCode::Down => {
                            let max_processes = self.process_monitor.get_sorted_processes().len();
                            if self.process_scroll < max_processes.saturating_sub(20) {
                                self.process_scroll += 1;
                            }
                        }
                        KeyCode::PageUp => {
                            self.process_scroll = self.process_scroll.saturating_sub(10);
                        }
                        KeyCode::PageDown => {
                            let max_processes = self.process_monitor.get_sorted_processes().len();
                            self.process_scroll = (self.process_scroll + 10)
                                .min(max_processes.saturating_sub(20));
                        }
                        KeyCode::Home => {
                            self.process_scroll = 0;
                        }
                        KeyCode::End => {
                            let max_processes = self.process_monitor.get_sorted_processes().len();
                            self.process_scroll = max_processes.saturating_sub(20);
                        }
                        KeyCode::Enter => {
                            if self.process_scroll < self.process_monitor.get_sorted_processes().len() {
                                self.process_selected = Some(self.process_scroll);
                            }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) if self.mouse_enabled => {
                    self.handle_mouse(mouse);
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                if self.process_scroll > 0 {
                    self.process_scroll -= 1;
                }
            }
            MouseEventKind::ScrollDown => {
                let max_processes = self.process_monitor.get_sorted_processes().len();
                if self.process_scroll < max_processes.saturating_sub(20) {
                    self.process_scroll += 1;
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                // Could add click to select process
                let row = mouse.row as usize;
                if row > 3 && row < 24 {
                    let index = row - 4 + self.process_scroll;
                    if index < self.process_monitor.get_sorted_processes().len() {
                        self.process_selected = Some(index);
                    }
                }
            }
            _ => {}
        }
    }

    fn update_filter_regex(&mut self) {
        if self.process_filter.is_empty() {
            self.process_filter_regex = None;
        } else {
            self.process_filter_regex = Regex::new(&self.process_filter).ok();
        }
    }

    fn kill_selected_process(&mut self) {
        if let Some(index) = self.process_selected {
            let processes = self.process_monitor.get_sorted_processes();
            if let Some(process) = processes.get(index) {
                #[cfg(unix)]
                {
                    use std::process::Command;
                    let _ = Command::new("kill")
                        .arg(format!("{}", process.pid))
                        .output();
                }
                self.process_selected = None;
            }
        }
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

        // Draw different content based on current page
        match self.current_page {
            ViewPage::Overview => self.draw_overview_page(frame, main_chunks[1]),
            ViewPage::Processes => self.draw_processes_page(frame, main_chunks[1]),
            ViewPage::Network => self.draw_network_page(frame, main_chunks[1]),
            ViewPage::Storage => self.draw_storage_page(frame, main_chunks[1]),
        }

        // Draw help overlay if activated
        if self.show_help {
            self.draw_help_overlay(frame, frame.area());
        }
    }

    fn draw_overview_page(&self, frame: &mut Frame, area: Rect) {
        // Adjust layout based on temperature sensor availability
        let has_temp = self.temp_monitor.has_temperature_sensors();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(22),  // CPU
                Constraint::Percentage(22),  // Memory & Swap combined
                Constraint::Percentage(56),  // Bottom section
            ])
            .split(area);

        // Top section: CPU
        self.draw_cpu(frame, chunks[0]);

        // Middle section: Memory and Swap on same graph
        self.draw_memory(frame, chunks[1]);

        // Bottom section: Left column (Network, Disk, Gauges), Right column (Temperature if available, Processes)
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(chunks[2]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(bottom_chunks[0]);

        self.draw_network(frame, left_chunks[0]);
        self.draw_disk(frame, left_chunks[1]);

        // Right column: Temperature (if available) and Processes
        if has_temp {
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
                .split(bottom_chunks[1]);
            self.draw_temperature_compact(frame, right_chunks[0]);
            self.draw_processes(frame, right_chunks[1]);
        } else {
            self.draw_processes(frame, bottom_chunks[1]);
        }
    }

    fn draw_processes_page(&self, frame: &mut Frame, area: Rect) {
        // Full-screen process list
        self.draw_processes(frame, area);
    }

    fn draw_network_page(&self, frame: &mut Frame, area: Rect) {
        // Network-focused view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),  // Network stats
                Constraint::Percentage(50),  // CPU (shows network impact)
            ])
            .split(area);

        self.draw_network(frame, chunks[0]);
        self.draw_cpu(frame, chunks[1]);
    }

    fn draw_storage_page(&self, frame: &mut Frame, area: Rect) {
        // Storage-focused view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),  // Disk usage
                Constraint::Percentage(50),  // Memory (storage related)
            ])
            .split(area);

        let disk_mem_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        self.draw_disk(frame, disk_mem_chunks[0]);
        self.draw_memory_gauges(frame, disk_mem_chunks[1]);

        self.draw_memory(frame, chunks[1]);
    }

    fn draw_header(&self, frame: &mut Frame, area: Rect) {
        let page_indicator = match self.current_page {
            ViewPage::Overview => "Overview",
            ViewPage::Processes => "Processes",
            ViewPage::Network => "Network",
            ViewPage::Storage => "Storage",
        };

        let gpu_indicator = if self.gpu_monitor.is_enabled() {
            format!(" 🎮 {}GPU ", self.gpu_monitor.gpu_count())
        } else {
            String::new()
        };

        let title = vec![
            Line::from(vec![
                Span::styled(" ⚡ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("rtop", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" v3.0", Style::default().fg(Color::Rgb(100, 200, 255)).add_modifier(Modifier::ITALIC)),
                Span::raw(" "),
                Span::styled(&gpu_indicator, Style::default().fg(Color::Green)),
                Span::raw(" │ "),
                Span::styled("◆ ", Style::default().fg(Color::Magenta)),
                Span::styled(page_indicator, Style::default().fg(Color::Rgb(255, 200, 100)).add_modifier(Modifier::BOLD)),
                Span::raw(" │ "),
                Span::styled("F2-F5", Style::default().fg(Color::Cyan)),
                Span::raw(": Pages │ "),
                Span::styled("h", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(": Help │ "),
                Span::styled("g", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(": GPU"),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(100, 150, 200)))
            .border_type(ratatui::widgets::BorderType::Double);

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

        // Calculate average CPU usage for title color
        let avg_cpu = cpu_data.iter().map(|(_, usage, _)| usage).sum::<f32>() / cpu_data.len().max(1) as f32;
        let title_color = if avg_cpu > 80.0 {
            Color::Rgb(235, 112, 112) // Red
        } else if avg_cpu > 60.0 {
            Color::Rgb(245, 166, 35) // Orange
        } else if avg_cpu > 40.0 {
            Color::Rgb(255, 195, 69) // Yellow
        } else {
            Color::Rgb(72, 151, 216) // Blue
        };

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled("⚡ ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                        Span::styled("CPU Usage ", Style::default().fg(title_color).add_modifier(Modifier::BOLD)),
                        Span::styled(format!("[{:.1}%]", avg_cpu), Style::default().fg(title_color)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(61, 123, 70)))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .x_axis(
                Axis::default()
                    .title(Span::styled("← Time (60s history)", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60s", Style::default().fg(Color::DarkGray)),
                        Span::styled("30s", Style::default().fg(Color::Gray)),
                        Span::styled("now", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("% ↑", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("  0%", Style::default().fg(Color::Rgb(72, 151, 216))),
                        Span::styled(" 50%", Style::default().fg(Color::Rgb(255, 195, 69))),
                        Span::styled("100%", Style::default().fg(Color::Rgb(235, 112, 112))),
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
            Color::Rgb(224, 92, 92) // Red
        } else if mem_percent > 70.0 {
            Color::Rgb(245, 166, 35) // Orange
        } else {
            Color::Rgb(255, 199, 69) // Yellow
        };

        let swap_color = if swap_percent > 85.0 {
            Color::Rgb(208, 92, 92)
        } else if swap_percent > 70.0 {
            Color::Rgb(232, 148, 35)
        } else {
            Color::Rgb(144, 224, 163) // Green
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
                        Span::styled("💾 ", Style::default().fg(Color::Rgb(245, 166, 35))),
                        Span::styled("Memory & Swap ", Style::default().fg(Color::Rgb(138, 136, 46)).add_modifier(Modifier::BOLD)),
                        Span::styled(format!("[{:.1}%]", mem_percent), Style::default().fg(mem_color)),
                    ])
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(138, 136, 46)))
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .x_axis(
                Axis::default()
                    .title(Span::styled("← Time (60s)", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60s", Style::default().fg(Color::DarkGray)),
                        Span::styled("30s", Style::default().fg(Color::Gray)),
                        Span::styled("now", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("% ↑", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("  0%", Style::default().fg(Color::Rgb(144, 224, 163))),
                        Span::styled(" 50%", Style::default().fg(Color::Rgb(255, 199, 69))),
                        Span::styled("100%", Style::default().fg(Color::Rgb(224, 92, 92))),
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

        // Create a more elegant visual bar
        let bar_length = 20;
        let filled = ((percent / 100.0 * bar_length as f32) as usize).min(bar_length);
        
        // Use different characters for gradient effect
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

        // Icon based on type
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
                Span::raw("  "),
                Span::styled(
                    format!("{:>5.1}%", percent),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  ["),
                Span::styled(bar, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::raw("]"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format_bytes(used, true),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" / ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(""),
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

        // Activity indicators
        let rx_indicator = if rx_sec > 1000000 { "●" } else if rx_sec > 10000 { "◐" } else { "○" };
        let tx_indicator = if tx_sec > 1000000 { "●" } else if tx_sec > 10000 { "◐" } else { "○" };

        let rx_rate = format_bytes(rx_sec, false);
        let tx_rate = format_bytes(tx_sec, false);
        let rx_total = format_bytes(total_rx, false);
        let tx_total = format_bytes(total_tx, false);

        // Get ping latency and interface name
        let ping_latency = self.network_monitor.get_ping_latency();
        let interface = self.network_monitor.get_active_interface();

        // Calculate max speed from current rates (keep track of peaks)
        let max_rate = rx_sec.max(tx_sec);
        let max_rate_str = if max_rate > 0 {
            format_bytes(max_rate, false)
        } else {
            "N/A".to_string()
        };

        let mut text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ▼ Download ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::styled(rx_indicator, Style::default().fg(Color::Blue)),
                Span::raw("  "),
                Span::styled("▲ Upload ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(tx_indicator, Style::default().fg(Color::Green)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{:>12}", rx_rate), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::styled("/s", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled(format!("{:>12}", tx_rate), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("/s", Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{:>12}", rx_total), Style::default().fg(Color::Cyan)),
                Span::styled(" total", Style::default().fg(Color::DarkGray)),
                Span::raw(" "),
                Span::styled(format!("{:>12}", tx_total), Style::default().fg(Color::Cyan)),
                Span::styled(" total", Style::default().fg(Color::DarkGray)),
            ]),
        ];

        // Add separator and additional info
        text.push(Line::from(""));
        
        // Interface and ping info
        let mut info_line = vec![
            Span::styled("  ◆ ", Style::default().fg(Color::Yellow)),
            Span::styled(interface, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::raw("  │  "),
        ];

        if let Some(latency) = ping_latency {
            let ping_color = if latency < 50.0 {
                Color::Green
            } else if latency < 100.0 {
                Color::Yellow
            } else {
                Color::Red
            };
            info_line.push(Span::styled("⚡ ", Style::default().fg(ping_color)));
            info_line.push(Span::styled(format!("{:.1} ms", latency), Style::default().fg(ping_color).add_modifier(Modifier::BOLD)));
        } else {
            info_line.push(Span::styled("⚡ ", Style::default().fg(Color::DarkGray)));
            info_line.push(Span::styled("--- ms", Style::default().fg(Color::DarkGray)));
        }

        text.push(Line::from(info_line));
        text.push(Line::from(""));

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

        // Create a compact horizontal bar
        let available_width = area.width.saturating_sub(6).max(20) as usize;
        let bar_length = available_width.min(40);
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
                Span::styled("  ● ", Style::default().fg(disk_color).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{:>5.1}%", percent),
                    Style::default().fg(disk_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw("  Used: "),
                Span::styled(
                    format!("{:<10}", format_bytes(used, true)),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(Color::Green)),
                Span::raw("Free:  "),
                Span::styled(
                    format!("{:<10}", format_bytes(available, true)),
                    Style::default().fg(Color::Green),
                ),
                Span::raw("  Total: "),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Line::from(""),
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
        let mut processes = self.process_monitor.get_sorted_processes();
        
        // Apply filter if active
        if let Some(ref regex) = self.process_filter_regex {
            processes = processes
                .into_iter()
                .filter(|p| regex.is_match(&p.name))
                .collect();
        }

        let total_processes = processes.len();
        let visible_count = (area.height as usize).saturating_sub(3).min(20);
        
        let end_index = (self.process_scroll + visible_count).min(total_processes);
        let processes_slice = &processes[self.process_scroll..end_index];
        
        let mut rows = Vec::with_capacity(processes_slice.len());
        
        for (i, p) in processes_slice.iter().enumerate() {
            let _cpu_color = if p.cpu_usage > 50.0 {
                Color::Red
            } else if p.cpu_usage > 25.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            let is_selected = Some(self.process_scroll + i) == self.process_selected;
            let style = if is_selected {
                Style::default().bg(Color::Rgb(50, 50, 80)).fg(Color::White)
            } else if i % 2 == 0 {
                Style::default()
            } else {
                Style::default().bg(Color::Rgb(20, 20, 30))
            };

            let row = Row::new(vec![
                if is_selected { "▶".to_string() } else { " ".to_string() },
                p.pid.to_string(),
                p.name.chars().take(20).collect::<String>(),
                format!("{:.1}%", p.cpu_usage),
                format_bytes(p.memory, false),
            ])
            .style(style);
            
            rows.push(row);
        }

        let scroll_info = if total_processes > visible_count {
            format!(" [{}-{}/{}] ", 
                self.process_scroll + 1, 
                end_index, 
                total_processes)
        } else {
            format!(" [{}] ", total_processes)
        };

        let title = vec![
            Span::styled("⚙ ", Style::default().fg(Color::Yellow)),
            Span::styled("Processes ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(scroll_info, Style::default().fg(Color::DarkGray)),
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
            Span::raw("Mem "),
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("k", Style::default().fg(Color::Yellow)),
            Span::styled("]", Style::default().fg(Color::DarkGray)),
            Span::raw("Kill"),
        ];

        let table = Table::new(
            rows,
            [
                Constraint::Length(2),
                Constraint::Length(7),
                Constraint::Length(20),
                Constraint::Length(7),
                Constraint::Length(10),
            ],
        )
        .header(
            Row::new(vec!["", "PID", "Process", "CPU", "Memory"])
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED),
                ),
        )
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 80))
                .add_modifier(Modifier::BOLD),
        );

        frame.render_widget(table, area);

        // Show kill confirmation dialog
        if self.show_kill_confirm {
            self.draw_kill_confirm(frame, area);
        }
    }

    fn draw_kill_confirm(&self, frame: &mut Frame, area: Rect) {
        let popup_area = Self::centered_rect(40, 20, area);

        if let Some(index) = self.process_selected {
            let processes = self.process_monitor.get_sorted_processes();
            if let Some(process) = processes.get(index) {
                let text = vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Kill process?", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("PID: "),
                        Span::styled(format!("{}", process.pid), Style::default().fg(Color::Yellow)),
                    ]),
                    Line::from(vec![
                        Span::raw("Name: "),
                        Span::styled(&process.name, Style::default().fg(Color::Cyan)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Press ", Style::default().fg(Color::DarkGray)),
                        Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::styled(" to confirm, any other key to cancel", Style::default().fg(Color::DarkGray)),
                    ]),
                ];

                let paragraph = Paragraph::new(text)
                    .block(
                        Block::default()
                            .title(" ⚠ Confirm ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Red))
                            .border_type(ratatui::widgets::BorderType::Thick)
                            .style(Style::default().bg(Color::Rgb(30, 20, 20))),
                    )
                    .alignment(ratatui::layout::Alignment::Center);

                frame.render_widget(paragraph, popup_area);
            }
        }
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

    fn draw_temperature_compact(&self, frame: &mut Frame, area: Rect) {
        let temp_data = self.temp_monitor.get_temperature_data();
        
        // If no temperature data available, show a message
        if !self.temp_monitor.has_temperature_sensors() || temp_data.is_empty() {
            return;
        }

        // Create a compact horizontal display of temperatures
        let mut lines = vec![Line::from("")];
        
        // Group temperatures into rows
        let temps_per_row = 3;
        for chunk in temp_data.chunks(temps_per_row) {
            let mut row_spans = vec![Span::raw("  ")];
            
            for (i, (label, temp, _)) in chunk.iter().enumerate() {
                if i > 0 {
                    row_spans.push(Span::raw(" │ "));
                }
                
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
                let display_label = if label.len() > 12 {
                    format!("{}...", &label[..9])
                } else {
                    label.clone()
                };

                row_spans.push(Span::styled(format!("{} ", icon), Style::default().fg(temp_color)));
                row_spans.push(Span::styled(
                    format!("{:.0}°C ", temp),
                    Style::default().fg(temp_color).add_modifier(Modifier::BOLD),
                ));
                row_spans.push(Span::styled(display_label, Style::default().fg(Color::DarkGray)));
            }
            
            lines.push(Line::from(row_spans));
        }
        
        lines.push(Line::from(""));

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .title(vec![
                    Span::styled("🌡 ", Style::default().fg(Color::Red)),
                    Span::styled("Temperature", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
                ])
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

        frame.render_widget(paragraph, area);
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) {
        let (load_1, load_5, load_15) = self.system_monitor.load_average();
        
        let status = if self.paused {
            Span::styled(" ⏸ PAUSED ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD).bg(Color::Rgb(60, 60, 0)))
        } else {
            Span::styled(" ▶ RUNNING ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        };

        let mut footer_spans = vec![
            status,
            Span::raw(" │ "),
            Span::styled("Uptime: ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.system_monitor.uptime_formatted(), Style::default().fg(Color::Cyan)),
            Span::raw(" │ "),
            Span::styled("Load: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.2} {:.2} {:.2}", load_1, load_5, load_15), Style::default().fg(Color::White)),
        ];

        // Add battery info if available
        if let Some(battery) = self.battery_monitor.get_battery_info() {
            let battery_icon = if battery.is_charging { "🔌" } else { "🔋" };
            let battery_color = if battery.percentage > 50.0 {
                Color::Green
            } else if battery.percentage > 20.0 {
                Color::Yellow
            } else {
                Color::Red
            };

            footer_spans.extend(vec![
                Span::raw(" │ "),
                Span::styled(battery_icon, Style::default()),
                Span::raw(" "),
                Span::styled(format!("{:.0}%", battery.percentage), Style::default().fg(battery_color)),
            ]);
        }

        footer_spans.extend(vec![
            Span::raw(" │ "),
            Span::styled("Processes: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", self.system_monitor.total_processes()), Style::default().fg(Color::White)),
        ]);

        if !self.process_filter.is_empty() {
            footer_spans.extend(vec![
                Span::raw(" │ "),
                Span::styled("Filter: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&self.process_filter, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]);
        }

        let footer_text = vec![Line::from(footer_spans)];

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
                Span::styled("    F2-F5           ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Switch pages (Overview/Process/Network/Storage)"),
            ]),
            Line::from(vec![
                Span::styled("    Space           ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Pause/Resume updates"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Process List:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("    ↑↓, PgUp/Dn     ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Navigate/scroll process list"),
            ]),
            Line::from(vec![
                Span::styled("    Home/End        ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Jump to first/last process"),
            ]),
            Line::from(vec![
                Span::styled("    Mouse wheel     ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Scroll processes"),
            ]),
            Line::from(vec![
                Span::styled("    k               ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Kill selected process (with confirm)"),
            ]),
            Line::from(vec![
                Span::styled("    /               ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("→ Filter processes (regex)"),
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
                Span::styled("  Features:", Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("    • Real-time CPU, Memory, Network, Disk, Battery"),
            ]),
            Line::from(vec![
                Span::raw("    • Temperature & Disk I/O monitoring"),
            ]),
            Line::from(vec![
                Span::raw("    • Process management with mouse support"),
            ]),
            Line::from(vec![
                Span::raw("    • System info: uptime, load average"),
            ]),
            Line::from(vec![
                Span::raw("    • Export to JSON/CSV formats"),
            ]),
            Line::from(vec![
                Span::raw("    • Regex filtering and multi-page navigation"),
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
