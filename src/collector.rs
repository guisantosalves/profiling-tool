use std::time::Duration;
use serde::Serialize;
use sysinfo::{ Components, Disks, System };

// u -> positivo
// i -> ambos

#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uptime: u64,
    pub total_disk: u64,
    pub used_disk: u64,
    pub temperature: Option<f32>,
}

pub fn collect() -> SystemStats {
    // por padrão no rust toda variável é imutáel
    // tem de especificar quando quer usar ela mutável
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu_usage();

    let disks = Disks::new_with_refreshed_list();

    let total_disk: u64 = disks
        .iter()
        .map(|d| d.total_space())
        .sum();

    let available_disk: u64 = disks
        .iter()
        .map(|d| d.available_space())
        .sum();

    let components = Components::new_with_refreshed_list();
    let temperature = components
        .iter()
        // filter_map descarta os None e extrai os valores f32 dos Some
        .filter_map(|c| c.temperature())
        .reduce(|a, b| a.max(b));

    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        uptime: System::uptime(),
        total_disk: total_disk,
        used_disk: total_disk - available_disk,
        temperature,
    }
}
