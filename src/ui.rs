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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(frame.area());

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
            .map(|(i, ((name, _, _), data))| {
                Dataset::default()
                    .name(name.clone())
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(COLORS[i % COLORS.len()]))
                    .graph_type(ratatui::widgets::GraphType::Line)
                    .data(data)
            })
            .collect();

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title("CPU History")
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60s", Style::default().fg(Color::Gray)),
                        Span::styled("30s", Style::default().fg(Color::Gray)),
                        Span::styled("0s", Style::default().fg(Color::Gray)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("0%", Style::default().fg(Color::Gray)),
                        Span::styled("50%", Style::default().fg(Color::Gray)),
                        Span::styled("100%", Style::default().fg(Color::Gray)),
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

        let datasets = vec![
            Dataset::default()
                .name(format!("Memory {:.1}%", mem_percent))
                .marker(symbols::Marker::Braille)
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().fg(COLORS[0]))
                .data(&mem_data),
            Dataset::default()
                .name(format!("Swap {:.1}%", swap_percent))
                .marker(symbols::Marker::Braille)
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().fg(COLORS[1]))
                .data(&swap_data),
        ];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title("Memory and Swap History")
                    .title_style(Style::default().add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 60.0])
                    .labels(vec![
                        Span::styled("60s", Style::default().fg(Color::Gray)),
                        Span::styled("30s", Style::default().fg(Color::Gray)),
                        Span::styled("0s", Style::default().fg(Color::Gray)),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Usage")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::styled("0%", Style::default().fg(Color::Gray)),
                        Span::styled("50%", Style::default().fg(Color::Gray)),
                        Span::styled("100%", Style::default().fg(Color::Gray)),
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

    fn create_circular_gauge(&self, title: &str, percent: f32, used: u64, total: u64, base_color: Color) -> Paragraph {
        let color = if percent > 90.0 {
            Color::Red
        } else if percent > 70.0 {
            Color::Yellow
        } else {
            base_color
        };

        // Créer une barre visuelle avec des blocs Unicode
        let bar_length = 20;
        let filled = ((percent / 100.0 * bar_length as f32) as usize).min(bar_length);
        let bar = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(bar_length - filled)
        );

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!(" {}  ", title),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(bar, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("{:.1}%", percent),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format_bytes(used, true),
                    Style::default().fg(Color::White),
                ),
                Span::raw(" / "),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ];

        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)),
            )
            .alignment(ratatui::layout::Alignment::Left)
    }

    fn draw_network(&self, frame: &mut Frame, area: Rect) {
        let (_, _, rx_sec, tx_sec, total_rx, total_tx) = self.network_monitor.get_network_data();

        let text = vec![
            Line::from(vec![
                Span::styled("↓ ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::raw("Receiving:      "),
                Span::styled(
                    format!("{}/s", format_bytes(rx_sec, false)),
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  Total received: "),
                Span::styled(
                    format_bytes(total_rx, false),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("↑ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("Transferring:      "),
                Span::styled(
                    format!("{}/s", format_bytes(tx_sec, false)),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  Total transferred: "),
                Span::styled(
                    format_bytes(total_tx, false),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title("Network History")
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
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

        // Créer une représentation visuelle du disque
        let bar_length = 30;
        let filled = ((percent / 100.0 * bar_length as f32) as usize).min(bar_length);
        let bar = format!(
            "{}{}",
            "█".repeat(filled),
            "░".repeat(bar_length - filled)
        );

        let available = total.saturating_sub(used);
        
        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(bar, Style::default().fg(disk_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    format!("{:.1}% used", percent),
                    Style::default().fg(disk_color).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ● ", Style::default().fg(disk_color)),
                Span::raw("Used: "),
                Span::styled(
                    format_bytes(used, true),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(Color::DarkGray)),
                Span::raw("Free: "),
                Span::styled(
                    format_bytes(available, true),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(vec![
                Span::raw("  ━ "),
                Span::raw("Total: "),
                Span::styled(
                    format_bytes(total, true),
                    Style::default().fg(Color::Gray),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .title("Disk usage")
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );

        frame.render_widget(paragraph, area);
    }

    fn draw_processes(&self, frame: &mut Frame, area: Rect) {
        let processes = self.process_monitor.get_sorted_processes();
        let rows: Vec<Row> = processes
            .iter()
            .take(20)
            .map(|p| {
                Row::new(vec![
                    p.pid.to_string(),
                    p.name.chars().take(24).collect::<String>(),
                    format!("{:.1}%", p.cpu_usage),
                    format_bytes(p.memory, false),
                ])
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
            Row::new(vec!["PID", "Command", "CPU%", "Memory"])
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .block(
            Block::default()
                .title("Processes (p: PID, c: CPU, m: Memory)")
                .title_style(Style::default().add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

        frame.render_widget(table, area);
    }
}
