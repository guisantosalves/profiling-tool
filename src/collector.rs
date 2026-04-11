use std::{ time::Duration };
use serde::Serialize;
use sysinfo::{ Components, Disks, System };

// u -> positivo
// i -> ambos
#[derive(Serialize)]
pub struct CpuCore {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
}

#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub uptime: u64,
    pub temperature: Option<f32>,
    pub cpus: Vec<CpuCore>,
    pub physical_cores: Option<usize>,
    pub processes: Vec<ProcessInfo>,
    pub total_swap: u64,
    pub used_swap: u64,
    pub disks: Vec<DiskInfo>,
    pub system: SystemInfo,
}

pub fn collect() -> SystemStats {
    // por padrão no rust toda variável é imutáel
    // tem de especificar quando quer usar ela mutável
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu_usage();

    let disk_lst = Disks::new_with_refreshed_list();
    let disks = disk_lst
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            available_space: d.available_space(),
            mount_point: d.mount_point().to_string_lossy().to_string(),
            total_space: d.total_space(),
        })
        .collect();

    let components = Components::new_with_refreshed_list();
    let temperature = components
        .iter()
        // filter_map descarta os None e extrai os valores f32 dos Some
        .filter_map(|c| c.temperature())
        .reduce(|a, b| a.max(b));

    let cpus: Vec<CpuCore> = sys
        .cpus()
        .iter()
        .map(|c| CpuCore {
            name: c.name().to_string(),
            frequency: c.frequency(),
            usage: c.cpu_usage(),
        })
        .collect(); // tipo o toList()

    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        // |(pid, p)| — sys.processes() retorna um HashMap<Pid, Process>
        .map(|(pid, pr)| ProcessInfo {
            pid: pid.as_u32(),
            name: pr.name().to_string_lossy().to_string(),
            cpu_usage: pr.cpu_usage(),
            memory: pr.memory(),
        })
        .collect();

    // filtering the processes
    /*
      - Crescente (menor → maior): a.cmp(&b)
      - Decrescente (maior → menor): b.cmp(&a) — inverte
    */
    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    processes.truncate(20);

    SystemStats {
        cpu_usage: sys.global_cpu_usage(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        uptime: System::uptime(),
        temperature,
        cpus,
        physical_cores: sys.physical_core_count(),
        processes,
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        disks,
        system: SystemInfo {
            name: System::name(),
            kernel_version: System::kernel_version(),
            os_version: System::os_version(),
            host_name: System::host_name(),
        },
    }
}
