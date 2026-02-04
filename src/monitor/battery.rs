use battery::{Battery, Manager, State};
use std::time::Duration;

pub struct BatteryMonitor {
    manager: Option<Manager>,
    battery: Option<Battery>,
    last_update: std::time::Instant,
}

impl BatteryMonitor {
    pub fn new() -> Self {
        let manager = Manager::new().ok();
        let battery = manager.as_ref().and_then(|m| {
            m.batteries()
                .ok()
                .and_then(|mut batteries| batteries.next().and_then(|b| b.ok()))
        });

        Self {
            manager,
            battery,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if self.last_update.elapsed() < Duration::from_secs(2) {
            return;
        }

        if let Some(ref manager) = self.manager {
            if let Ok(mut batteries) = manager.batteries() {
                if let Some(Ok(battery)) = batteries.next() {
                    self.battery = Some(battery);
                }
            }
        }
        self.last_update = std::time::Instant::now();
    }

    #[allow(dead_code)]
    pub fn has_battery(&self) -> bool {
        self.battery.is_some()
    }

    pub fn get_battery_info(&self) -> Option<BatteryInfo> {
        self.battery.as_ref().map(|b| {
            let state = b.state();
            let percentage = b.state_of_charge().value * 100.0;
            let time_to_full = b.time_to_full();
            let time_to_empty = b.time_to_empty();
            let energy_rate = b.energy_rate().value;

            BatteryInfo {
                percentage,
                state: match state {
                    State::Charging => "Charging".to_string(),
                    State::Discharging => "Discharging".to_string(),
                    State::Full => "Full".to_string(),
                    State::Empty => "Empty".to_string(),
                    _ => "Unknown".to_string(),
                },
                is_charging: matches!(state, State::Charging),
                time_remaining: if matches!(state, State::Charging) {
                    time_to_full.map(|t| t.get::<battery::units::time::second>() as u64)
                } else {
                    time_to_empty.map(|t| t.get::<battery::units::time::second>() as u64)
                },
                power_consumption: energy_rate,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub percentage: f32,
    #[allow(dead_code)]
    pub state: String,
    pub is_charging: bool,
    #[allow(dead_code)]
    pub time_remaining: Option<u64>,
    #[allow(dead_code)]
    pub power_consumption: f32,
}

impl BatteryInfo {
    #[allow(dead_code)]
    pub fn time_remaining_formatted(&self) -> String {
        match self.time_remaining {
            Some(secs) => {
                let hours = secs / 3600;
                let minutes = (secs % 3600) / 60;
                if hours > 0 {
                    format!("{}h {}m", hours, minutes)
                } else {
                    format!("{}m", minutes)
                }
            }
            None => "Calculating...".to_string(),
        }
    }
}
