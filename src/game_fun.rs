use std::iter;
use winit::{window::Window, dpi::PhysicalSize};
use crate::logger::*;

pub struct GameFuncs {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl GameFuncs {
    pub async fn new(window: Window) -> Self {
        Logger::log_info(LogLevel::Low, "\tStart GameFuncs::new");

        let size = window.inner_size();

        let instance_desc = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        };
        let instance = wgpu::Instance::new(instance_desc);

        Logger::log_info(LogLevel::Low, "\tCreating Surface");
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        Logger::log_info(LogLevel::Low, "\tCreating adapter");
        let adapter_options = wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_options).await.unwrap();

        Logger::log_info(LogLevel::Low, "\tCreating device");
        let device_descriptor = wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };
        let (device, queue) = adapter.request_device(&device_descriptor, None).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
                             .copied()
                             .filter(|f| f.describe().srgb)
                             .next()
                             .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Logger::log_info(LogLevel::Low, "\tFinished GameFunc::new");

        return Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        };
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        Logger::log_info(LogLevel::High, "\tStart GameFunc::resize");
        
        if new_size.width > 0 && new_size.height > 0 {
            let width = new_size.width;
            Logger::log_info(LogLevel::High, &(format!("\t\tnew width {width}"))[..]);
            
            let height = new_size.height;
            Logger::log_info(LogLevel::High, &(format!("\t\tnew height {height}"))[..]);

            self.size = new_size;
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }

        Logger::log_info(LogLevel::High, "\tFinished GameFunc::resize");
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        Logger::log_info(LogLevel::High, "\tStart GameFunc::render");

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let command_encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&command_encoder_desc);

        {
            let render_pass_color_attachment = wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r:0.0,
                        g:0.0,
                        b:0.0,
                        a:1.0,
                    }),
                    store: true,
                },
            };
            
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(render_pass_color_attachment)],
                depth_stencil_attachment: None,
            };

            Logger::log_info(LogLevel::High, "\t\tbegin render pass");
            let render_pass = encoder.begin_render_pass(&render_pass_desc);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Logger::log_info(LogLevel::High, "\tFinished GameFunc::render");

        return Ok(());
    }

    pub fn window(&self) -> &Window {
        return &self.window;
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        return self.size;
    }

    pub fn update(&mut self) {}
}