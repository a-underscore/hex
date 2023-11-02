use super::{ev::Control, ComponentManager, EntityManager, Ev, SystemManager};

use std::sync::Arc;
use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo,
    },
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::GpuFuture,
    Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Context {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub render_pass: Arc<RenderPass>,
    pub images: Vec<Arc<Image>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub swapchain: Arc<Swapchain>,
    pub window: Arc<Window>,
    pub viewport: Viewport,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub bg: [f32; 4],
}

impl Context {
    pub fn new(bg: [f32; 4]) -> anyhow::Result<(EventLoop<()>, Self)> {
        let event_loop = EventLoop::new()?;
        let library = VulkanLibrary::new()?;
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
            .enumerate_physical_devices()?
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
        )?;
        let queue = queues.next().unwrap();
        let (swapchain, images) = {
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
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )?
        };
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth_stencil: {
                    format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {depth_stencil},
            },
        )?;
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [0.0, 0.0],
            depth_range: 0.0..=1.0,
        };

        Ok((
            event_loop,
            Self {
                framebuffers: Self::window_size_dependent_setup(
                    memory_allocator.clone(),
                    &images,
                    render_pass.clone(),
                )?,
                images,
                recreate_swapchain: false,
                previous_frame_end: None,
                render_pass,
                viewport,
                command_buffer_allocator,
                descriptor_set_allocator,
                window,
                device,
                queue,
                memory_allocator,
                swapchain,
                bg,
            },
        ))
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        mut sm: SystemManager,
        (mut em, mut cm): (EntityManager, ComponentManager),
    ) -> anyhow::Result<()> {
        sm.init(&mut self, (&mut em, &mut cm))?;

        event_loop.run(move |event, elwt| {
            if let Err(e) = self.update(&mut sm, (&mut em, &mut cm), Control::new(event, elwt)) {
                eprintln!("{}", e);
            }
        })?;

        Ok(())
    }

    pub fn update(
        &mut self,
        sm: &mut SystemManager,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
        mut control: Control,
    ) -> anyhow::Result<()> {
        match control.event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control.elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                self.recreate_swapchain = true;
            }
            Event::AboutToWait => self.window.request_redraw(),
            _ => {}
        }

        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = control.event
        {
            let (image_index, suboptimal, acquire_future) =
                match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        self.recreate_swapchain = true;

                        return Ok(());
                    }
                    Err(e) => return Err(e.into()),
                };

            if suboptimal {
                self.recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                &self.command_buffer_allocator,
                self.queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )?;

            builder.begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(self.bg.into())],
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[image_index as usize].clone(),
                    )
                },
                Default::default(),
            )?;

            sm.update(&mut Ev::Draw((&mut control, &mut builder)), self, (em, cm))?;

            let command_buffer = builder.build()?;
            let _future = self
                .previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(self.queue.clone(), command_buffer)?
                .then_swapchain_present(
                    self.queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(
                        self.swapchain.clone(),
                        image_index,
                    ),
                )
                .then_signal_fence_and_flush();
        }

        Ok(())
    }

    fn window_size_dependent_setup(
        memory_allocator: Arc<StandardMemoryAllocator>,
        images: &[Arc<Image>],
        render_pass: Arc<RenderPass>,
    ) -> anyhow::Result<Vec<Arc<Framebuffer>>> {
        let depth_buffer = ImageView::new_default(
            Image::new(
                memory_allocator,
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format: Format::D16_UNORM,
                    extent: images[0].extent(),
                    usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap(),
        )
        .unwrap();
        let framebuffers = images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_buffer.clone()],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect();

        Ok(framebuffers)
    }
}
