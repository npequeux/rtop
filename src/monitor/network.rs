use sysinfo::Networks;
use std::collections::VecDeque;

const HISTORY_SIZE: usize = 61;

pub struct NetworkMonitor {
    networks: Networks,
    rx_history: VecDeque<u64>,
    tx_history: VecDeque<u64>,
    total_rx: u64,
    total_tx: u64,
    last_rx: u64,
    last_tx: u64,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        
        let (total_rx, total_tx) = networks.iter()
            .fold((0u64, 0u64), |(rx, tx), (_, data)| {
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
        }
    }
    
    pub fn update(&mut self) {
        self.networks.refresh();
        
        let (current_rx, current_tx) = self.networks.iter()
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
        
        (rx_history, tx_history, rx_sec, tx_sec, self.total_rx, self.total_tx)
    }
}
