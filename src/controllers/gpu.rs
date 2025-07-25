use crate::windows;

pub async fn gpu() -> String {
    let windoww = windows::fetch().await;
    format!(
        "{}\nResolución: {}x{} | {}Hz\nVRAM: {} MB",
        windoww.1.name,
        windoww.1.current_horizontal_resolution,
        windoww.1.current_vertical_resolution,
        windoww.1.current_refresh_rate,
        (windoww.1.adapter_ram / 1024) / 1024
    )
}