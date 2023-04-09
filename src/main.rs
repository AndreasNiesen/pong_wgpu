use pong_wgpu::logger::*;
use pong_wgpu::run;

fn main() {
    Logger::log_info(LogLevel::Low, "Start Logging");

    pollster::block_on(run());

    Logger::log_info(LogLevel::Low, "Finished Logging");
}