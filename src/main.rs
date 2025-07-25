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
use std::{io::{stdout, Result}, string};
use std::time::Duration;
use sysinfo::{RefreshKind, System};
extern crate sysinfo;
use crate::controllers::{cpu::{cpu, cpu_threads}, disk::disk, gpu::gpu, memory::memory, os::os};
mod controllers;
mod windows;

fn render(
    frame: &mut Frame,
    detailed_cpu: bool,
    osrametc: u8,
    cpua:String,
    gpua:String
) {
    // ete de la cpu
    let cpuph = if detailed_cpu {
        let text = String::new();
        text
    } else {
       cpua
    };
    // aca va a ir el de la computadora en general
    // falta hacer lo de la temperatura

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
        Paragraph::new(gpua).block(
            Block::default()
                .borders(Borders::ALL)
                .title("GPU Info")
                .style(Style::default().fg(Color::White)),
        ),
        left_col[0],
    );
    let mut textoram = String::new();
    match osrametc {
        0 => textoram = memory(),
        1 => textoram = disk(),
        2 => textoram = os(),
        _ => textoram = memory(),
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
        let text = cpu_threads()
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
    let mut show_detailed_cpu = false;
    let mut osrametc = 0;

    loop {

        let cpua = cpu().await;
        let gpua = gpu().await;
        terminal.draw(|f| {
            render(f, show_detailed_cpu, osrametc, cpua, gpua);
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
