use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

pub struct Monitor {
    sys: System,
}

impl Monitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Monitor { sys }
    }

    pub fn update(&mut self) {
        self.sys.refresh_all();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
    }

    pub fn get_host_name(&self) -> String {
        System::host_name().unwrap_or_default()
    }

    pub fn get_os_name(&self) -> String {
        System::long_os_version().unwrap_or_default()
    }

    pub fn get_kernel_version(&self) -> String {
        System::kernel_version().unwrap_or_default()
    }

    pub fn get_uptime(&self) -> u64 {
        System::uptime()
    }

    pub fn get_cpu_name(&self) -> String {
        self.sys.cpus()[0].brand().to_string()
    }

    pub fn get_global_cpu_usage(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    pub fn get_per_cpu_usage(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|p| p.cpu_usage()).collect()
    }

    pub fn get_total_memory(&self) -> (u64, u64) {
        (self.sys.total_memory(), self.sys.total_swap())
    }

    pub fn get_used_memory(&self) -> (u64, u64) {
        (self.sys.used_memory(), self.sys.used_swap())
    }

    pub fn get_processes(&self) -> Vec<(u32, String, f32, u64, u64, ProcessStatus)> {
        self.sys
            .processes()
            .values()
            .map(|p| {
                (
                    p.pid().as_u32(),
                    p.name().to_string_lossy().into_owned(),
                    p.cpu_usage(),
                    p.memory(),
                    p.run_time(),
                    p.status(),
                )
            })
            .collect()
    }

    pub fn terminate_process(&mut self, pid: u32) {
        if let Some(process) = self.sys.process(Pid::from(pid as usize)) {
            process.kill();
        }
    }

    pub fn get_process_status_by_pid(&self, pid: u32) -> Option<(f32, f32, ProcessStatus)> {
        self.sys.process(Pid::from(pid as usize)).map(|p| {
            (
                p.cpu_usage(),
                (p.memory() as f32 / self.sys.total_memory() as f32) * 100.0,
                p.status(),
            )
        })
    }
}
