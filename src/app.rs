pub use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Application;

#[derive(Clone)]
pub struct AppBuilder {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "enigma demo".to_string(),
        }
    }
}
pub trait Runner<R>
where
    R: Runnable + 'static,
{
    fn run(&mut self, _runable: R) -> Result<()> {
        Ok(())
    }
}

pub trait Runnable {
    fn initialize(&mut self, _app: &mut Application) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, _app: &mut Application) -> Result<()> {
        Ok(())
    }
}

pub trait Builder<T> {
    fn build(self) -> T;
}

impl Builder<Result<(EventLoop<()>, Window)>> for AppBuilder {
    fn build(self) -> Result<(EventLoop<()>, Window)> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(self.title.to_string())
            .with_inner_size(PhysicalSize::new(self.width, self.height))
            .build(&event_loop)?;

        Ok((event_loop, window))
    }
}

impl<R> Runner<R> for AppBuilder
where
    R: Runnable + 'static,
{
    fn run(&mut self, mut runable: R) -> Result<()> {
        tracing_subscriber::fmt().init();
        let (event_loop, _window) = self.clone().build()?;
        tracing::info!("Running Application");
        let mut app = Application {};
        runable.initialize(&mut app)?;

        event_loop.run(move |event, _, control_flow| {
            let mut cycle_result = || -> Result<()> {
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::MainEventsCleared => {
                        runable.update(&mut app)?;
                    }
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Event::LoopDestroyed => {
                        tracing::info!("Exited application");
                    }
                    _ => {}
                }
                Ok(())
            };
            if let Err(error) = cycle_result() {
                tracing::error!("Application Error: {}", error);
            }
        });
    }
}
impl Application {
    pub fn build() -> AppBuilder {
        AppBuilder::default()
    }
}
