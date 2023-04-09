pub mod logger;

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

    Logger::log_info(LogLevel::Medium, "Starting event_loop");

    event_loop.run(move |event, _, control_flow| {
        // Make sure the event_loop runs continuously (not just if events are dispatched)
        control_flow.set_poll();

        match event {
            Event::WindowEvent{event, window_id} => {
                match event {
                    WindowEvent::CloseRequested => {
                        Logger::log_info(LogLevel::High, "Closing Window due to X-Button-Press.");
                        control_flow.set_exit();
                    },
                    WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }, .. } => {
                        Logger::log_info(LogLevel::High, "Closing Window due to Escape-Press.");
                        control_flow.set_exit();
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    });
}