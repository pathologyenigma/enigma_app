
use anyhow::Result;
pub struct Application;

pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Obsidian Application".to_string(),
        }
    }
}

pub trait Runner<App> {
    type App = App;
    fn initialize(&mut self, _application: &mut Self::App) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, _application: &mut Self::App) -> Result<()> {
        Ok(())
    }
}
impl Runner<Application> for Application {}

pub fn run<App>(mut runner: impl Runner<App> + 'static, configuration: AppConfig) -> Result<()> {
    // TODO ...
    Ok(())
}