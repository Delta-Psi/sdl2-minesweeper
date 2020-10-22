use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use wgpu::{Surface, Device, Queue, SwapChain, SwapChainDescriptor, Texture, TextureView};

use crate::shaders;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub(crate) const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

#[derive(Debug)]
pub struct Display {
    window: Window,

    surface: Surface,
    device: Device,
    queue: Queue,

    swap_chain_descriptor: SwapChainDescriptor,
    swap_chain: SwapChain,

    rect_pipeline: shaders::Rect,
}

impl Display {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
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

        let rect_pipeline = shaders::Rect::new(&device);

        Self {
            window,

            surface,
            device,
            queue,

            swap_chain_descriptor,
            swap_chain,

            rect_pipeline,
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

            rect_pipeline: &self.rect_pipeline,
        }
    }

    pub fn render<F: FnOnce(Renderer)>(&mut self, render_fn: F) {
        let renderer = self.renderer();
        render_fn(renderer);
    }

    /// Expects data in row major 8-bit RGBA format (sRGB)
    pub fn create_texture(&self, data: &[u8], width: u32, height: u32) -> Texture {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &data,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width,
                rows_per_image: 0,
            },
            wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
        );

        texture
    }
}

use wgpu::SwapChainFrame;

pub struct Renderer<'a> {
    pub(crate) device: &'a Device,
    pub(crate) queue: &'a Queue,
    pub(crate) frame: SwapChainFrame,

    rect_pipeline: &'a shaders::Rect,
}

impl<'a> Renderer<'a> {
    pub fn clear(&self, color: (f32, f32, f32)) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: color.0 as f64,
                                g: color.1 as f64,
                                b: color.2 as f64,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn draw_rect(&self, origin: (f32, f32), bounds: (f32, f32), texture_view: &TextureView) {
        self.rect_pipeline.draw_rect(&self, origin, bounds, texture_view);
    }
}

