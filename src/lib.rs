pub mod logger;
pub mod custom_vec;
pub mod balls;
mod game_fun;

use game_fun::GameFuncs;
use logger::*;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

pub async fn run() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    Logger::log_info(LogLevel::Low, "Creating GameFuncs");
    let mut game_funcs = GameFuncs::new(window).await;
    Logger::log_info(LogLevel::Low, "Created GameFuncs");

    Logger::log_info(LogLevel::Low, "Starting event_loop");
    event_loop.run(move |event, _, control_flow| {
        // Make sure the event_loop runs continuously (not just if events are dispatched)
        control_flow.set_poll();

        match event {
            Event::WindowEvent{event, window_id} => {
                match event {
                    WindowEvent::CloseRequested => {
                        Logger::log_info(LogLevel::Medium, "Closing Window due to X-Button-Press.");
                        control_flow.set_exit();
                    },
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }, .. } => {
                        Logger::log_info(LogLevel::Medium, "Closing Window due to Escape-Press.");
                        control_flow.set_exit();
                    },
                    WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                        game_funcs.add_ball();
                    }
                    WindowEvent::Resized(physical_size) => {
                        Logger::log_info(LogLevel::High, "Window resized.");
                        game_funcs.resize(physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        Logger::log_info(LogLevel::High, "Window rescaled.");
                        game_funcs.resize(*new_inner_size);
                    },
                    _ => ()
                }
            },
            Event::RedrawRequested(window_id) => {
                game_funcs.update();

                match game_funcs.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        Logger::log_warning(LogLevel::Low, "Surface lost or outdated");
                        game_funcs.resize(game_funcs.size());
                    },
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        Logger::log_error(LogLevel::Low, "Surface Out of Memory");
                        control_flow.set_exit();
                    },
                    Err(wgpu::SurfaceError::Timeout) => {
                        Logger::log_warning(LogLevel::Medium, "Surface timeout");
                    },
                }
            }
            Event::RedrawEventsCleared => {
                game_funcs.window().request_redraw();
            },
            _ => ()
        }
    });
}