use rand::{Rng, thread_rng};
use ratatui::style::Color;
use sysinfo::{RefreshKind, System};

use crate::windows;

pub async fn cpu() -> String {
    let windoww = windows::fetch().await;
    format!(
        "{}\n\
        Velocidad actual: {} MHz\n\
        Voltaje actual: {} v\n\
        Velocidad máxima: {} MHz\n\
        Nucleos físicos: {}\n\
        Nucleos habilitados: {}\n\
        Nucleos logicos (hilos): {}\n\
        Porcentaje de uso: {}%",
        windoww.0.name,
        windoww.0.current_clock_speed,
        windoww.0.current_voltage,
        windoww.0.max_clock_speed,
        windoww.0.number_of_cores,
        windoww.0.number_of_enabled_core,
        windoww.0.number_of_logical_processors,
        windoww.0.load_percentage
    )
}

pub fn cpu_threads() -> Vec<(String, Color)> {
    let mut system = System::new_with_specifics(RefreshKind::everything().without_memory());
    system.refresh_cpu_all();
    let cpu_usages: Vec<f32> = system
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage() / 100.0)
        .collect();
    // aca me gustaria que tenga colores diustintos anda a saber
    /*
     * en realidad lo que me gustaria es que se seleccione de una
     * entonces cuando cambia el porcentaje los colores no
     * ademas habria que almacenarlos para que despues no se tenga que volver a hacer el proceso
     * */
    let mut rng = thread_rng();
    let mut lines = Vec::new();
    for (i, usage) in cpu_usages.iter().enumerate() {
        let color = Color::Rgb(rng.r#gen(), rng.r#gen(), rng.r#gen());
        lines.push((
            format!("Nucleo Logico {}: {:.1}%", i + 1, usage * 100.0),
            color,
        ));
    }
    lines
}
