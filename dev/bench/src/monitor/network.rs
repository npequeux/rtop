use std::collections::VecDeque;
use std::process::Command;
use std::time::{Duration, Instant};
use sysinfo::Networks;

const HISTORY_SIZE: usize = 61;

pub struct NetworkMonitor {
    networks: Networks,
    rx_history: VecDeque<u64>,
    tx_history: VecDeque<u64>,
    total_rx: u64,
    total_tx: u64,
    last_rx: u64,
    last_tx: u64,
    ping_latency: Option<f32>,
    last_ping_check: Instant,
    active_interface: String,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();

        let (total_rx, total_tx) = networks.iter().fold((0u64, 0u64), |(rx, tx), (_, data)| {
            (rx + data.total_received(), tx + data.total_transmitted())
        });

        Self {
            networks,
            rx_history: VecDeque::from(vec![0; HISTORY_SIZE]),
            tx_history: VecDeque::from(vec![0; HISTORY_SIZE]),
            total_rx,
            total_tx,
            last_rx: total_rx,
            last_tx: total_tx,
            ping_latency: None,
            last_ping_check: Instant::now(),
            active_interface: String::from("eth0"),
        }
    }

    pub fn update(&mut self) {
        self.networks.refresh();

        let (current_rx, current_tx) = self
            .networks
            .iter()
            .fold((0u64, 0u64), |(rx, tx), (_, data)| {
                (rx + data.total_received(), tx + data.total_transmitted())
            });

        let rx_sec = current_rx.saturating_sub(self.last_rx);
        let tx_sec = current_tx.saturating_sub(self.last_tx);

        self.rx_history.pop_front();
        self.rx_history.push_back(rx_sec);

        self.tx_history.pop_front();
        self.tx_history.push_back(tx_sec);

        self.last_rx = current_rx;
        self.last_tx = current_tx;
        self.total_rx = current_rx;
        self.total_tx = current_tx;
    }

    pub fn get_network_data(&self) -> (Vec<u64>, Vec<u64>, u64, u64, u64, u64) {
        let rx_history: Vec<u64> = self.rx_history.iter().copied().collect();
        let tx_history: Vec<u64> = self.tx_history.iter().copied().collect();
        let rx_sec = *self.rx_history.back().unwrap_or(&0);
        let tx_sec = *self.tx_history.back().unwrap_or(&0);

        (
            rx_history,
            tx_history,
            rx_sec,
            tx_sec,
            self.total_rx,
            self.total_tx,
        )
    }

    pub fn get_ping_latency(&self) -> Option<f32> {
        self.ping_latency
    }

    pub fn get_active_interface(&self) -> &str {
        // Get the first active interface
        if let Some((name, _data)) = self
            .networks
            .iter()
            .find(|(_, data)| data.total_received() > 0 || data.total_transmitted() > 0)
        {
            name
        } else {
            &self.active_interface
        }
    }

    pub fn update_ping(&mut self) {
        // Only check ping every 3 seconds to avoid overhead
        if self.last_ping_check.elapsed() < Duration::from_secs(3) {
            return;
        }

        self.last_ping_check = Instant::now();

        // Try ping to common DNS servers (Google, Cloudflare)
        let hosts = ["8.8.8.8", "1.1.1.1"];

        for host in &hosts {
            if let Some(latency) = self.ping_host(host) {
                self.ping_latency = Some(latency);
                return;
            }
        }

        self.ping_latency = None;
    }

    fn ping_host(&self, host: &str) -> Option<f32> {
        // Use ping command with 1 packet, 1 second timeout
        let output = Command::new("ping")
            .args(["-c", "1", "-W", "1", host])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse ping output for latency (works on Linux/macOS)
        // Example: "time=14.2 ms"
        for line in stdout.lines() {
            if let Some(time_pos) = line.find("time=") {
                let time_str = &line[time_pos + 5..];
                if let Some(space_pos) = time_str.find(' ') {
                    if let Ok(latency) = time_str[..space_pos].parse::<f32>() {
                        return Some(latency);
                    }
                }
            }
        }

        None
    }
}
