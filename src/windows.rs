#![allow(non_camel_case_types)]

use serde::Deserialize;
use tokio::join;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;
use wmi::{COMLibrary, WMIConnection};

const HKLM: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);

type Info = (Win32_Processor, Win32_VideoController);

pub async fn fetch<'a>() -> Info {
    let con = get_wmi_con();

    let cpu_info_future = con.async_raw_query::<Win32_Processor>(
        "SELECT Name, Architecture, CurrentClockSpeed, CurrentVoltage, MaxClockSpeed, \
         NumberOfCores, NumberOfEnabledCore, NumberOfLogicalProcessors, LoadPercentage, \
         ThreadCount FROM Win32_Processor",
    );

    let video_controller_info_future = con.async_raw_query::<Win32_VideoController>(
        "SELECT Name, AdapterRAM, CurrentHorizontalResolution, CurrentVerticalResolution, \
         CurrentRefreshRate FROM Win32_VideoController",
    );

    let (cpu_info, video_controller_info) = join!(cpu_info_future, video_controller_info_future,);
    let cpu = cpu_info
        .expect("No se pudo obtener info de CPU")
        .into_iter()
        .next()
        .expect("Sin resultados CPU");
    let gpu = video_controller_info
        .expect("No se pudo obtener info de GPU")
        .into_iter()
        .next()
        .expect("Sin resultados GPU");

    (cpu, gpu)
}

fn get_wmi_con() -> WMIConnection {
    WMIConnection::new(COMLibrary::without_security().unwrap()).unwrap()
}

pub fn fetch_latest_ps_version() -> String {
    if let Ok(installed_versions) =
        HKLM.open_subkey("SOFTWARE\\Microsoft\\PowerShellCore\\InstalledVersions")
    {
        let keys: Vec<_> = installed_versions
            .enum_keys()
            .filter_map(Result::ok)
            .collect();
        let mut latest_version = String::new();
        for guid in keys.iter() {
            if let Ok(version) = installed_versions.open_subkey(&guid) {
                if let Ok(semantic_version) = version.get_value("SemanticVersion") {
                    if semantic_version > latest_version {
                        latest_version = semantic_version;
                    }
                }
            }
        }
        if !latest_version.is_empty() {
            return format!("PowerShell {}", latest_version);
        }
    }

    match std::env::var("PSModulePath") {
        Ok(var) => {
            let lowercased = var.to_lowercase();
            let paths: Vec<&str> = lowercased.split(';').collect();
            for path in paths {
                if path.contains("powershell_") {
                    let version: Vec<&str> = path.split('_').collect();
                    if version.len() > 1 {
                        return format!("PowerShell {}", version[1]);
                    }
                }
            }
        }
        Err(_) => {}
    }

    match fetch_legacy_ps_version() {
        Ok(v) => format!("PowerShell {}", v),
        Err(_) => "Console Host".to_string(),
    }
}

fn fetch_legacy_ps_version() -> Result<String, std::io::Error> {
    let regkey = HKLM
        .open_subkey("SOFTWARE\\Microsoft\\PowerShell\\3\\PowerShellEngine")
        .unwrap_or(HKLM.open_subkey("SOFTWARE\\Microsoft\\PowerShell\\1\\PowerShellEngine")?);
    regkey.get_value("RunTimeVersion")
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_Processor {
    pub name: String,
    pub current_clock_speed: u32,
    pub current_voltage: u16,
    pub max_clock_speed: u32,
    pub number_of_cores: u32,
    pub number_of_enabled_core: u32,
    pub number_of_logical_processors: u32,
    pub load_percentage: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_VideoController {
    pub name: String,
    pub current_horizontal_resolution: u16,
    pub current_vertical_resolution: u16,
    pub current_refresh_rate: u8,
    pub adapter_ram: u32,
}
