use app::App;
use color_eyre::Result;

mod app;
pub mod controllers;

#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;

    ratatui::restore();

    app_result
}
