use rand::{Rng, thread_rng};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{self, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{Result, stdout};
use std::time::Duration;
use sysinfo::{Disks, RefreshKind, System};
extern crate sysinfo;

mod windows;

fn render(
    frame: &mut Frame,
    windoww: &(windows::Win32_Processor, windows::Win32_VideoController),
    detailed_cpu: bool,
    cpu_usages: &[f32],
    osrametc: u8,
) {
    // ete e de la gpu gato
    let gpuph = format!(
        "{}\nResolución: {}x{} | {}Hz\nVRAM: {} MB",
        windoww.1.name,
        windoww.1.current_horizontal_resolution,
        windoww.1.current_vertical_resolution,
        windoww.1.current_refresh_rate,
        (windoww.1.adapter_ram / 1024) / 1024
    );
    // ete de la cpu
    let cpuph = if detailed_cpu {
        let mut text = String::new();
        for (i, usage) in cpu_usages.iter().enumerate() {
            text += &format!(
                "Nucleo  - Hotkey: g (off){}: {:.1}%\n",
                i + 1,
                usage * 100.0
            );
        }
        text
    } else {
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
    };
    // aca va a ir el de la computadora en general
    // falta hacer lo de la temperatura
    let info: os_info::Info = os_info::get();
    let binding: Option<&str> = info.edition();
    let osname: &str = binding.as_deref().expect("wtf");
    let binding: Option<&str> = info.architecture();
    let architecture: &str = binding.as_deref().expect("wtf");
    let binding: Option<String> = System::host_name();
    let hostname: &str = binding.as_deref().expect("wtf");
    let mut sys: System = System::new_all();
    let mut disks_info: String = String::new();
    let disks: Disks = Disks::new_with_refreshed_list();
    sys.refresh_all();
    for (index, disk) in disks.iter().enumerate() {
        if index > 0 {
            disks_info.push_str(" | ");
        }
        let mount: std::borrow::Cow<str> = disk.mount_point().to_string_lossy();
        let mount2: String = mount.to_string().replace(r"\", "");
        let space: u64 = disk.total_space() / 1024 / 1024 / 1024;
        disks_info.push_str(&format!("({}) {} GB", mount2, space));
    }
    let osph = format!(
        "OS: {} {}\nKernel: {}\nHost: {}\nShell: {}",
        osname,
        architecture,
        info.version().to_string().trim_matches('"'),
        hostname,
        windows::fetch_latest_ps_version()
    );

    let diskph = format!("Discos:\n{}", disks_info);
    let ramph = format!(
        "Memoria: {} MB / {} MB",
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024
    );

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.size());

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(vertical_chunks[0]);

    let left_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_row[0]);

    frame.render_widget(
        Paragraph::new(gpuph).block(
            Block::default()
                .borders(Borders::ALL)
                .title("GPU Info")
                .style(Style::default().fg(Color::White)),
        ),
        left_col[0],
    );
    let mut textoram = String::new();
    match osrametc {
        0 => textoram = ramph,
        1 => textoram = diskph,
        2 => textoram = osph,
        _ => textoram = ramph,
    }
    frame.render_widget(
        Paragraph::new(textoram).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Os/Memoria/Discos - Hotkey: o, m, d")
                .style(Style::default().fg(Color::White)),
        ),
        left_col[1],
    );

    let cpu_widget = if detailed_cpu {
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

        let text = lines
            .into_iter()
            .map(|(line, color)| ratatui::text::Line::styled(line, Style::default().fg(color)))
            .collect::<Vec<_>>();

        Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("CPU Threads - Hotkey: shift+c (info)")
                .style(Style::default()),
        )
    } else {
        Paragraph::new(cpuph).block(
            Block::default()
                .borders(Borders::ALL)
                .title("CPU Info - Hotkey: c (threads)")
                .style(Style::default().fg(Color::White)),
        )
    };

    frame.render_widget(cpu_widget, top_row[1]);

    /*
     * en esta parete me gustaria que haya un grafico tipo osciloscopio que sea 0-100% de cpu, gpu
     * */
    frame.render_widget(
        Paragraph::new("blablabla blebleble blublublu").block(
            Block::default()
                .borders(Borders::ALL)
                .title("Is this thing on?")
                .style(Style::default().fg(Color::White)),
        ),
        vertical_chunks[1],
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // no se como anda pero anda
    let mut system = System::new_with_specifics(RefreshKind::everything().without_memory());

    let mut show_detailed_cpu = false;
    let mut osrametc = 0;

    loop {
        system.refresh_cpu_all();
        let cpu_usages: Vec<f32> = system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage() / 100.0)
            .collect();

        let windoww = windows::fetch().await;
        terminal.draw(|f| {
            render(f, &windoww, show_detailed_cpu, &cpu_usages, osrametc);
        })?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') => show_detailed_cpu = true,
                    KeyCode::Char('C') => show_detailed_cpu = false,
                    KeyCode::Char('m') => osrametc = 0,
                    KeyCode::Char('d') => osrametc = 1,
                    KeyCode::Char('o') => osrametc = 2,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
