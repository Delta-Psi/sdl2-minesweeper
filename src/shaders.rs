use crate::display::Renderer;
use wgpu::{Device, BindGroup, RenderPipeline, Buffer};

#[derive(Debug)]
pub struct Rect {
    rect_buffer: Buffer,
    color_buffer: Buffer,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
}

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct RectUniform {
    origin: [f32; 2],
    bounds: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct ColorUniform {
    color: [f32; 3],
}

impl Rect {
    pub fn new(device: &Device) -> Self {
        let vertex_shader = device.create_shader_module(
            wgpu::include_spirv!("shaders/rect.vert.spv")
        );
        let fragment_shader = device.create_shader_module(
            wgpu::include_spirv!("shaders/rect.frag.spv")
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let rect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<RectUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let color_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<ColorUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(rect_buffer.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(color_buffer.slice(..)),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_shader,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fragment_shader,
                entry_point: "main",
            }),
            rasterization_state: None,
            primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
            color_states: &[crate::display::TEXTURE_FORMAT.into()],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            rect_buffer,
            color_buffer,
            bind_group,
            pipeline,
        }
    }

    pub fn draw_rect(&self, renderer: &Renderer, origin: (f32, f32), bounds: (f32, f32), color: (f32, f32, f32)) {
        let rect = RectUniform {
            origin: [origin.0, origin.1],
            bounds: [bounds.0, bounds.1],
        };
        renderer.queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&rect));

        let color = ColorUniform {
            color: [color.0, color.1, color.2],
        };
        renderer.queue.write_buffer(&self.color_buffer, 0, bytemuck::bytes_of(&color));

        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &renderer.frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });

            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..4, 0..1);
        }

        renderer.queue.submit(Some(encoder.finish()));
    }
}
