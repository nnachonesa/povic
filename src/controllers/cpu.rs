use std::fs;

use rand::{Rng, rng};
use ratatui::style::Color;
use sysinfo::{CpuRefreshKind, System};

pub struct CpuInfo {
    pub name: String,
    pub freq_mhz: Option<u32>,
    pub current_voltage: Option<f32>,
    pub max_clock_speed: Option<u32>,
    pub cores: u32,
    pub enabled_cores: Option<u32>,
    pub threads: Option<u32>,
    pub load_percentage: Option<f32>,
}

impl std::fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Nombre: {}
Velocidad actual: {:.1} MHz
Voltaje actual: {} v
Velocidad máxima: {:.1} MHz
Nucleos físicos: {}
Nucleos habilitados: {}
Nucleos lógicos (hilos): {}
Porcentaje de uso: {:.1}%
"#,
            self.name,
            self.freq_mhz.unwrap_or(0) as f32,
            self.current_voltage.unwrap_or(0.0),
            self.max_clock_speed.unwrap_or(0) as f32,
            self.cores,
            self.enabled_cores.unwrap_or(0),
            self.threads.unwrap_or(0),
            self.load_percentage.unwrap_or(0.0) * 100.0
        )
    }
}

#[cfg(target_os = "linux")]
pub async fn cpu(system: &mut System) -> CpuInfo {
    system.refresh_cpu_specifics(CpuRefreshKind::everything());

    let cpu = system.cpus().first().expect("No CPU found");

    // Get CPU frequency
    let cpu_freq = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok());

    CpuInfo {
        name: cpu.name().to_string(),
        freq_mhz: Some(cpu_freq.unwrap_or(0) / 1000),
        current_voltage: None, // Voltage is not available in sysinfo for Linux
        max_clock_speed: None, // Max frequency is not available in sysinfo for Linux
        cores: system.cpus().len() as u32,
        enabled_cores: None,
        threads: None,
        load_percentage: Some(system.global_cpu_usage()),
    }
}

#[cfg(target_os = "windows")]
pub async fn cpu(_: &mut System) -> CpuInfo {
    let windoww = crate::windows::fetch().await;

    CpuInfo {
        name: windoww.0.name,
        freq_mhz: Some(windoww.0.current_clock_speed),
        current_voltage: Some(windoww.0.current_voltage as f32 / 1000.0),
        max_clock_speed: Some(windoww.0.max_clock_speed),
        cores: windoww.0.number_of_cores,
        enabled_cores: Some(windoww.0.number_of_enabled_core),
        threads: Some(windoww.0.number_of_logical_processors),
        load_percentage: Some(windoww.0.load_percentage as f32 / 100.0),
    }
}

pub fn cpu_threads(system: &mut System) -> Vec<(String, Color)> {
    system.refresh_cpu_specifics(CpuRefreshKind::everything());

    let cpu_usages: Vec<f32> = system
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage() / 100.0)
        .collect();

    // aca me gustaria que tenga colores distintos anda a saber
    // en realidad lo que me gustaria es que se seleccione de una
    // entonces cuando cambia el porcentaje los colores no
    // ademas habria que almacenarlos para que despues no se tenga que volver a
    // hacer el proceso
    let mut rng = rng();
    let mut lines = Vec::new();
    for (i, usage) in cpu_usages.iter().enumerate() {
        let color = Color::Rgb(rng.random(), rng.random(), rng.random());
        lines.push((
            format!("Nucleo Logico {}: {:.1}%", i + 1, usage * 100.0),
            color,
        ));
    }

    lines
}
