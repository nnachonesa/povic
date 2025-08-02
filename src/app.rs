use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tokio_stream::StreamExt;

use crate::controllers::cpu::{cpu, cpu_threads};
use crate::controllers::disk::disk;
use crate::controllers::gpu::gpu;
use crate::controllers::memory::memory;
use crate::controllers::os::os;

#[derive(Debug)]
pub struct App {
    should_quit: bool,

    show_detailed_cpu: bool,

    /// 0: memory, 1: disk, 2: os
    osrametc: u8,

    /// Thread safe system information
    system: Box<sysinfo::System>,
}

impl Default for App {
    fn default() -> Self {
        let system = sysinfo::System::new_all();

        Self {
            should_quit: false,
            show_detailed_cpu: false,
            osrametc: 0,
            system: Box::new(system),
        }
    }
}

impl App {
    const FRAMES_PER_SECOND: f32 = 23.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => {
                    let cpua = cpu(&mut self.system).await;
                    let gpua = gpu().await;

                    terminal.draw(|frame|
                        self.render(frame, cpua.to_string(), gpua.to_string())
                    )?;
                },

                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, cpua: String, gpua: String) {
        // ete de la cpu
        let cpuph = if self.show_detailed_cpu {
            String::new()
        } else {
            cpua
        };
        // aca va a ir el de la computadora en general
        // falta hacer lo de la temperatura

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(frame.area());

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

        let texto_ram = match self.osrametc {
            0 => memory(&mut self.system),
            1 => disk(),
            2 => os(),
            _ => memory(&mut self.system),
        };

        frame.render_widget(
            Paragraph::new(texto_ram).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Os/Memoria/Discos - Hotkey: o, m, d")
                    .style(Style::default().fg(Color::White)),
            ),
            left_col[1],
        );

        let cpu_widget = if self.show_detailed_cpu {
            let text = cpu_threads(&mut self.system)
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

        // en esta parete me gustaria que haya un grafico tipo osciloscopio que sea
        // 0-100% de cpu, gpu
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

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('c') => self.show_detailed_cpu = true,
                KeyCode::Char('C') => self.show_detailed_cpu = false,
                KeyCode::Char('m') => self.osrametc = 0,
                KeyCode::Char('d') => self.osrametc = 1,
                KeyCode::Char('o') => self.osrametc = 2,
                _ => {}
            }
        }
    }
}
