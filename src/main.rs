use pong_wgpu::logger::*;
use pong_wgpu::run;

fn main() {
    Logger::log_info(LogLevel::Medium, "Start Logging");

    pollster::block_on(run());
}