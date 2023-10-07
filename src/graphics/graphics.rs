// src/graphics/graphics.rs

use std::{borrow::Cow, sync::{Arc, Mutex}};

use wgpu::{InstanceDescriptor, SurfaceConfiguration, DeviceDescriptor, util::DeviceExt};
use winit::{window::Window, event::WindowEvent};

use crate::{simulation::{particle::Particle, simulation::Simulation}, math::vector::Vector2d, SCALE_FACTOR};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceData {
    position: [f32; 2],
    scale: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ParticleVertex {
    position: [f32; 2]
}

pub struct GraphicsContext {
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    queue: wgpu::Queue,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl GraphicsContext {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // Initialise wgpu
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // Initialise surface
        let surface = unsafe {
            instance.create_surface(&window)
        }.expect("[GRAPHICS] Failed to create surface.");

        // Initialise adapter
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.expect("[GRAPHICS] Failed to request adapter.");

        // Initialise device and queue
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None
        }, None).await.expect("[GRAPHICS] Failed to request device.");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            alpha_mode: surface_caps.alpha_modes[0],
            present_mode: surface_caps.present_modes[0],
            format: surface_format,
            height: size.height,
            width: size.width,
            view_formats: vec![],
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
        };

        surface.configure(&device, &config);

        let frag_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../res/shaders/fragment.wgsl").into())
        });

        let vert_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../res/shaders/vertex.wgsl").into())
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ParticleVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                },
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![
                        // Assuming that the InstanceData has a vec2<f32> position and a float scale
                        1 => Float32x2,  // instance position
                        2 => Float32,    // instance scale
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline
        }

    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self, simulation: Arc<Mutex<Simulation>>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    
        let scale_factor = (SCALE_FACTOR * SCALE_FACTOR) / self.size.width as f32;
    
        let quad_vertices = [
            ParticleVertex { position: [-0.5, -0.5] },
            ParticleVertex { position: [ 0.5, -0.5] },
            ParticleVertex { position: [-0.5,  0.5] },
            ParticleVertex { position: [ 0.5,  0.5] },
            ParticleVertex { position: [-0.5,  0.5] },
            ParticleVertex { position: [ 0.5, -0.5] },
        ];
    
        // Prepare the particle instances data
        let mut instance_data = Vec::new();
    
        for (x, column) in simulation.lock().unwrap().grid.iter().enumerate() {
            for (y, cell) in column.iter().enumerate() {
                if let Some(particle) = cell {
                    let position = Vector2d::new(x as f32, y as f32);
    
                    // Append the position to instance_data
                    instance_data.push(InstanceData {
                        position: position.to_ndc(self.size.width as f32, self.size.height as f32).into(),
                        scale: scale_factor, // or vary scale based on particle type
                    });
                }
            }
        }
    
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
    
        let instance_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
    
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });
    
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.draw(0..quad_vertices.len() as u32, 0..instance_data.len() as u32);
        }
    
        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    
        output.present();
    
        Ok(())
    }    
}