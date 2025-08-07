use crate::windows;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct GpuInfo {
    pub name: String,
    pub horizontal_resolution: Option<u32>,
    pub vertical_resolution: Option<u32>,
    pub refresh_rate: Option<u32>,
    pub vram: u64,
}

impl std::fmt::Display for GpuInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
Nombre: {}
Resolución horizontal: {} px
Resolución vertical: {} px
Frecuencia de actualización: {} Hz
VRAM: {} MB
"#,
            self.name,
            self.horizontal_resolution.unwrap_or(0),
            self.vertical_resolution.unwrap_or(0),
            self.refresh_rate.unwrap_or(0),
            (self.vram / 1024) / 1024
        )
    }
}

#[cfg(target_os = "linux")]
pub async fn gpu() -> GpuInfo {
    use std::process::Command;

    let mut name = String::from("Desconocido");

    if let Ok(output) = Command::new("lspci").arg("-v").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                if let Some(gpu_info) = line.split(':').nth(2) {
                    name = gpu_info.trim().to_owned();
                    break;
                }
            }
        }
    }

    let mut horizontal = None;
    let mut vertical = None;
    let mut refresh_rate = None;

    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let name_str = entry.file_name().to_string_lossy().to_string();

            if name_str.contains("card") && name_str.contains("connected") {
                let modes_path = path.join("modes");
                if modes_path.exists() {
                    if let Ok(file) = File::open(&modes_path) {
                        let reader = BufReader::new(file);
                        if let Some(Ok(line)) = reader.lines().next() {
                            if let Some((res_part, hz_part)) = line.split_once('@') {
                                if let Some((w, h)) = res_part.split_once('x') {
                                    horizontal = w.parse::<u32>().ok();
                                    vertical = h.parse::<u32>().ok();
                                }
                                refresh_rate = hz_part.trim_end_matches("Hz").parse::<u32>().ok();
                            } else if let Some((w, h)) = line.split_once('x') {
                                horizontal = w.parse::<u32>().ok();
                                vertical = h.parse::<u32>().ok();
                                refresh_rate = Some(60); // default
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    let vram = fallback_vram();

    GpuInfo {
        name,
        horizontal_resolution: horizontal,
        vertical_resolution: vertical,
        refresh_rate,
        vram,
    }
}

#[cfg(target_os = "linux")]
fn fallback_vram() -> u64 {
    let amd_path = "/sys/class/drm/card1/device/mem_info_vram_total";
    if Path::new(amd_path).exists() {
        if let Ok(content) = fs::read_to_string(amd_path) {
            if let Ok(bytes) = content.trim().parse::<u64>() {
                return bytes / 1024 / 1024;
            }
        }
    }

    0
}

#[cfg(target_os = "windows")]
pub async fn gpu() -> GpuInfo {
    let windoww = windows::fetch().await;

    GpuInfo {
        name: windoww.1.name,
        horizontal_resolution: Some(windoww.1.current_horizontal_resolution.into()),
        vertical_resolution: Some(windoww.1.current_vertical_resolution.into()),
        refresh_rate: Some(windoww.1.current_refresh_rate.into()),
        vram: windoww.1.adapter_ram.into(),
    }
}
