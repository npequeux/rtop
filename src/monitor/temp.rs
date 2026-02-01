use sysinfo::Components;
use std::collections::VecDeque;

pub struct TempMonitor {
    components: Components,
    history: VecDeque<f32>,
    has_temps: bool,
}

impl TempMonitor {
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();
        let has_temps = !components.is_empty();
        
        Self {
            components,
            history: VecDeque::with_capacity(61),
            has_temps,
        }
    }

    pub fn update(&mut self) {
        if !self.has_temps {
            return;
        }

        self.components.refresh();
        
        // Get average temperature or max temperature
        let temp = if !self.components.is_empty() {
            let temps: Vec<f32> = self.components
                .iter()
                .map(|component| component.temperature())
                .collect();
            
            if !temps.is_empty() {
                temps.iter().sum::<f32>() / temps.len() as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        self.history.push_back(temp);
        if self.history.len() > 61 {
            self.history.pop_front();
        }
    }

    pub fn has_temperature_sensors(&self) -> bool {
        self.has_temps
    }

    pub fn get_temperature_data(&self) -> (f32, Vec<(String, f32)>, &VecDeque<f32>) {
        if !self.has_temps {
            return (0.0, vec![], &self.history);
        }

        let current = self.history.back().copied().unwrap_or(0.0);
        
        let components_data: Vec<(String, f32)> = self.components
            .iter()
            .map(|component| {
                (component.label().to_string(), component.temperature())
            })
            .collect();

        (current, components_data, &self.history)
    }

    pub fn get_max_temp(&self) -> f32 {
        self.components
            .iter()
            .map(|c| c.temperature())
            .fold(0.0, f32::max)
    }
}
