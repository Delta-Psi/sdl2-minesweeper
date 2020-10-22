use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wgpu::{Surface, Device, Queue, SwapChain, SwapChainDescriptor};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

#[derive(Debug)]
pub struct Display {
    window: Window,

    surface: Surface,
    device: Device,
    queue: Queue,

    swap_chain_descriptor: SwapChainDescriptor,
    swap_chain: SwapChain,

    rect_renderer: RectRenderer,
}

impl Display {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("wgpu minesweeper")
            .with_inner_size(winit::dpi::PhysicalSize::new(
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT))
            .with_resizable(false)
            .with_visible(false)
            .build(&event_loop).unwrap();

        let (surface, device, queue) =
            futures::executor::block_on(Display::init_wgpu(&window));

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: TEXTURE_FORMAT,
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let rect_renderer = RectRenderer::new(&device);

        Self {
            window,

            surface,
            device,
            queue,

            swap_chain_descriptor,
            swap_chain,

            rect_renderer,
        }
    }

    async fn init_wgpu(window: &Window) -> (Surface, Device, Queue)
    {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe {
            instance.create_surface(window)
        };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&Default::default(), None)
            .await.unwrap();

        (surface, device, queue)
    }

    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible);
    }

    fn renderer(&mut self) -> Renderer {
        let frame = match self.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self.device.create_swap_chain(
                    &self.surface,
                    &self.swap_chain_descriptor,
                );
                self.swap_chain.get_current_frame().unwrap()
            }
        };

        Renderer {
            device: &self.device,
            queue: &self.queue,
            frame,

            rect_renderer: &self.rect_renderer,
        }
    }

    pub fn render<F: FnOnce(Renderer)>(&mut self, render_fn: F) {
        let renderer = self.renderer();
        render_fn(renderer);
    }
}

use wgpu::SwapChainFrame;

pub struct Renderer<'a> {
    device: &'a Device,
    queue: &'a Queue,
    frame: SwapChainFrame,

    rect_renderer: &'a RectRenderer,
}

impl<'a> Renderer<'a> {
    pub fn render_rect(&self, origin: (f32, f32), bounds: (f32, f32), color: (f32, f32, f32)) {
        self.rect_renderer.render_rect(&self, origin, bounds, color);
    }
}

use wgpu::{BindGroup, RenderPipeline, Buffer};

#[derive(Debug)]
pub struct RectRenderer {
    rect_buffer: Buffer,
    color_buffer: Buffer,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
}

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct Rect {
    origin: [f32; 2],
    bounds: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct Color {
    color: [f32; 3],
}

impl RectRenderer {
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
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Rect>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<Color>() as u64),
                    },
                    count: None,
                },
            ],
        });

        let rect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<Rect>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let color_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<Color>() as wgpu::BufferAddress,
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
                //&bind_group_layout,
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
            color_states: &[TEXTURE_FORMAT.into()],
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

    pub fn render_rect(&self, renderer: &Renderer, origin: (f32, f32), bounds: (f32, f32), color: (f32, f32, f32)) {
        let rect = Rect {
            origin: [origin.0, origin.1],
            bounds: [bounds.0, bounds.1],
        };
        renderer.queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&rect));

        let color = Color {
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

            //rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..4, 0..1);
        }

        renderer.queue.submit(Some(encoder.finish()));
    }
}
