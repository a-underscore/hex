use super::{ComponentManager, ev::Control, EntityManager, SystemManager};
use std::{error::Error, sync::Arc};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        CopyBufferToImageInfo, PrimaryCommandBufferAbstract, RenderPassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
    },
    format::Format,
    image::{
        sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
        view::ImageView,
        Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator, MemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    DeviceSize, Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Context {
    pub device: Arc<Device>,
    pub memory_allocator: Arc<dyn MemoryAllocator>,
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
    pub em: EntityManager,
    pub cm: ComponentManager,
    pub sm: SystemManager,
    pub bg: [f32; 4],
}

impl Context {
    pub fn new((em, cm): (EntityManager, ComponentManager), sm: SystemManager, bg: [f32; 4]) -> anyhow::Result<Self> {
    let event_loop = EventLoop::new()?; let library = VulkanLibrary::new()?;
    let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )?;
    let window = Arc::new(WindowBuilder::new().build(&event_loop)?);
    let surface = Surface::from_window(instance.clone(), window.clone())?;
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        ?
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    ?;
    let queue = queues.next().unwrap(); let (mut swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())?;
            let image_format = device
                .physical_device()
                .surface_formats(&surface, Default::default())?[0]
                .0;
            Swapchain::new(
                device.clone(),
                surface,
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next().unwrap(),
                    ..Default::default()
                },
            )?
        };
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        Ok(Self { window, device, memory_allocator, event_loop, em, cm, sm, bg })
    }

    pub fn init(
        mut self,
    ) -> anyhow::Result<()> {
        self.sm.init(&mut self, (&mut self.em, &mut self.cm))?;

        self.event_loop.run(move |event, elwt| {
            if let Err(e) = self.update(Control::new(event, elwt), (&mut self.em, &mut self.cm), &mut self.sm) {
                eprintln!("{}", e);
            }
        });

        Ok(())
    }

    pub fn update(
        &mut self,
        mut control: Control,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
        sm: &mut SystemManager,
    ) -> anyhow::Result<()> {
        match control.event {
            Event::RedrawRequested(window_id)
                if window_id == self.display.gl_window().window().id() =>
            {
                let mut target = self.display.draw();

                target.clear_color_and_depth(
                    {
                        let [r, g, b, a] = self.bg;

                        (r, g, b, a)
                    },
                    1.0,
                );

                sm.update(&mut control, self, (em, cm))?;

                target.finish()?;
            }
            _ => {
                sm.update(&mut control, self, (em, cm))?;

                if let Event::MainEventsCleared = control.event {
                    self.display.gl_window().window().request_redraw();
                }
            }
        }

        *cf = match control {
            Control {
                flow: Some(flow),
                event: _,
            } => flow,
            _ => ControlFlow::Poll,
        };

        Ok(())
    }

    fn window_size_dependent_setup(
        images: &[Arc<Image>],
        viewport: &mut Viewport,
    ) -> Vec<Arc<ImageView>> {
        let extent = images[0].extent();

        viewport.extent = [extent[0] as f32, extent[1] as f32];

        images
            .iter()
            .map(|image| ImageView::new_default(image.clone())?)
            .collect::<Vec<_>>()
    }
}
