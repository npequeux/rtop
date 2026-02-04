use sysinfo::Disks;

pub struct DiskMonitor {
    disks: Disks,
}

impl DiskMonitor {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        Self { disks }
    }

    pub fn update(&mut self) {
        self.disks.refresh();
    }

    pub fn get_disk_data(&self) -> (f32, u64, u64) {
        // Get the first disk or aggregate all disks
        let (total_space, available_space) = self
            .disks
            .iter()
            .fold((0u64, 0u64), |(total, avail), disk| {
                (total + disk.total_space(), avail + disk.available_space())
            });

        if total_space == 0 {
            return (0.0, 0, 0);
        }

        let used = total_space.saturating_sub(available_space);
        let percent = (used as f64 / total_space as f64 * 100.0) as f32;

        (percent, used, total_space)
    }
}
