use crate::display::Renderer;
use wgpu::{Device, BindGroupLayout, RenderPipeline, Buffer, Sampler, TextureView};

#[derive(Debug)]
pub struct Rect {
    rect_buffer: Buffer,
    sampler: Sampler,
    bind_group_layout: BindGroupLayout,
    pipeline: RenderPipeline,
}

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct RectUniform {
    origin: [f32; 2],
    bounds: [f32; 2],
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
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
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
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: crate::display::TEXTURE_FORMAT,
                    alpha_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    color_blend: wgpu::BlendDescriptor {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }
            ],
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
            sampler,
            bind_group_layout,
            pipeline,
        }
    }

    pub fn draw_rect(&self,
        renderer: &Renderer,
        origin: (f32, f32),
        bounds: (f32, f32),
        texture_view: &TextureView,
    ) {
        let rect = RectUniform {
            origin: [origin.0, origin.1],
            bounds: [bounds.0, bounds.1],
        };
        renderer.queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&rect));

        let bind_group = renderer.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(self.rect_buffer.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

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

            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..4, 0..1);
        }

        renderer.queue.submit(Some(encoder.finish()));
    }
}
