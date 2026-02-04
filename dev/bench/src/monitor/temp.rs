use std::collections::{HashMap, VecDeque};
use sysinfo::Components;

pub struct TempMonitor {
    components: Components,
    history: HashMap<String, VecDeque<f32>>, // History per component
    has_temps: bool,
}

impl TempMonitor {
    pub fn new() -> Self {
        let mut components = Components::new_with_refreshed_list();

        // Force a refresh to ensure we get data
        std::thread::sleep(std::time::Duration::from_millis(100));
        components.refresh();

        // Check if we have any components with valid temperatures
        let has_temps = components.iter().any(|c| c.temperature() > 0.0);

        Self {
            components,
            history: HashMap::new(),
            has_temps,
        }
    }

    pub fn update(&mut self) {
        if !self.has_temps {
            // Try to re-detect sensors
            self.components.refresh();
            let has_temps = self.components.iter().any(|c| c.temperature() > 0.0);
            if has_temps && !self.has_temps {
                self.has_temps = true;
            }
            if !self.has_temps {
                return;
            }
        }

        self.components.refresh();

        // Update history for each component
        for component in self.components.iter() {
            let temp = component.temperature();
            if temp > 0.0 {
                let label = component.label().to_string();
                let history = self
                    .history
                    .entry(label)
                    .or_insert_with(|| VecDeque::with_capacity(61));

                history.push_back(temp);
                if history.len() > 61 {
                    history.pop_front();
                }
            }
        }
    }

    pub fn has_temperature_sensors(&self) -> bool {
        self.has_temps
    }

    pub fn get_temperature_data(&self) -> Vec<(String, f32, &VecDeque<f32>)> {
        if !self.has_temps {
            return vec![];
        }

        self.components
            .iter()
            .filter(|c| c.temperature() > 0.0)
            .filter_map(|component| {
                let label = component.label().to_string();
                let temp = component.temperature();
                self.history
                    .get(&label)
                    .map(|history| (label, temp, history))
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_max_temp(&self) -> f32 {
        self.components
            .iter()
            .map(|c| c.temperature())
            .filter(|&t| t > 0.0)
            .fold(0.0, f32::max)
            .max(30.0) // Minimum scale of 30°C for better visibility
    }
}
