use pong_wgpu::run;

fn main() {
    pollster::block_on(run());
}