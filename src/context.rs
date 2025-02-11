use crate::{Control, World};
use nalgebra::Vector4;
use parking_lot::RwLock;
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
        acquire_next_image, PresentMode, Surface, Swapchain, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::Window,
};

pub struct Context {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub render_pass: Arc<RenderPass>,
    pub surface: Arc<Surface>,
    pub images: Vec<Arc<Image>>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub swapchain: Arc<Swapchain>,
    pub window: Arc<Window>,
    pub viewport: Viewport,
    pub previous_frame_end: Option<Box<dyn GpuFuture + Send + Sync>>,
    pub bg: Vector4<f32>,
}

impl Context {
    pub fn new(
        event_loop: &EventLoop<()>,
        window: Arc<Window>,
        bg: Vector4<f32>,
    ) -> anyhow::Result<Arc<RwLock<Self>>> {
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
                surface.clone(),
                SwapchainCreateInfo {
                    present_mode: PresentMode::Immediate,
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
        let (framebuffers, viewport) = Self::window_size_dependent_setup(
            memory_allocator.clone(),
            &images,
            render_pass.clone(),
        )?;

        Ok(Arc::new(RwLock::new(Self {
            framebuffers,
            images,
            surface,
            previous_frame_end: Some(sync::now(device.clone()).boxed_send_sync()),
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
        })))
    }

    pub fn init(
        context: Arc<RwLock<Self>>,
        event_loop: EventLoop<()>,
        world: Arc<RwLock<World>>,
    ) -> anyhow::Result<()> {
        {
            let sm = world.read().sm.clone();

            sm.write().init(context.clone(), world.clone())?;
        }

        let mut recreate_swapchain = false;

        event_loop.run(move |event, elwt| {
            if let Err(e) = Self::update(
                context.clone(),
                world.clone(),
                Control::new(event),
                (elwt, &mut recreate_swapchain),
            ) {
                eprintln!("{}", e);
            }
        })?;

        Ok(())
    }

    pub fn update(
        context: Arc<RwLock<Self>>,
        world: Arc<RwLock<World>>,
        control: Arc<RwLock<Control>>,
        (elwt, recreate_swapchain): (&EventLoopWindowTarget<()>, &mut bool),
    ) -> anyhow::Result<()> {
        let sm = world.read().sm.clone();

        sm.write()
            .update(control.clone(), context.clone(), world.clone())?;

        let event = control.read().event.clone();

        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == { context.read().window.id() } => {
                let (mut builder, rs, suboptimal, acquire_future, image_index) = {
                    let mut context = context.write();
                    let image_extent: [u32; 2] = context.window.inner_size().into();

                    if image_extent.contains(&0) {
                        return Ok(());
                    }

                    context
                        .previous_frame_end
                        .as_mut()
                        .unwrap()
                        .cleanup_finished();

                    let mut builder = AutoCommandBufferBuilder::primary(
                        &context.command_buffer_allocator,
                        context.queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    )?;

                    let rs = *recreate_swapchain;

                    if *recreate_swapchain {
                        let (new_swapchain, new_images) =
                            context.swapchain.recreate(SwapchainCreateInfo {
                                image_extent,
                                ..context.swapchain.create_info()
                            })?;
                        context.swapchain = new_swapchain;
                        context.images = new_images;

                        let (framebuffers, viewport) = Self::window_size_dependent_setup(
                            context.memory_allocator.clone(),
                            &context.images,
                            context.render_pass.clone(),
                        )?;

                        context.framebuffers = framebuffers;
                        context.viewport = viewport;

                        *recreate_swapchain = false;
                    }

                    let (image_index, suboptimal, acquire_future) =
                        match acquire_next_image(context.swapchain.clone(), None)
                            .map_err(Validated::unwrap)
                        {
                            Ok(r) => r,
                            Err(VulkanError::OutOfDate) => {
                                *recreate_swapchain = true;

                                return Ok(());
                            }
                            Err(e) => return Err(e.into()),
                        };

                    builder
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![
                                    Some(<[f32; 4]>::from(context.bg).into()),
                                    Some(1f32.into()),
                                ],
                                ..RenderPassBeginInfo::framebuffer(
                                    context.framebuffers[image_index as usize].clone(),
                                )
                            },
                            Default::default(),
                        )?
                        .set_viewport(0, [context.viewport.clone()].into_iter().collect())?;

                    (builder, rs, suboptimal, acquire_future, image_index)
                };

                let rm = world.read().rm.clone();

                rm.write().draw(
                    &mut (control.clone(), &mut builder, rs),
                    context.clone(),
                    world.clone(),
                )?;

                builder.end_render_pass(Default::default())?;

                let command_buffer = builder.build()?;
                let mut context = context.write();

                if suboptimal {
                    *recreate_swapchain = true;
                }

                {
                    let future = context
                        .previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(context.queue.clone(), command_buffer)?
                        .then_swapchain_present(
                            context.queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(
                                context.swapchain.clone(),
                                image_index,
                            ),
                        )
                        .then_signal_fence_and_flush();

                    match future.map_err(Validated::unwrap) {
                        Ok(future) => {
                            context.previous_frame_end = Some(future.boxed_send_sync());
                        }
                        Err(VulkanError::OutOfDate) => {
                            *recreate_swapchain = true;

                            context.previous_frame_end =
                                Some(sync::now(context.device.clone()).boxed_send_sync());
                        }
                        Err(_) => {
                            context.previous_frame_end =
                                Some(sync::now(context.device.clone()).boxed_send_sync());
                        }
                    }
                }
            }

            _ => {}
        }

        let control = control.read();

        if control.exit {
            elwt.exit();
        }

        {
            let id = context.read().window.id();

            match control.event {
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    window_id,
                } if window_id == id => {
                    *recreate_swapchain = true;
                }
                Event::AboutToWait => context.read().window.request_redraw(),
                _ => {}
            }
        }

        Ok(())
    }

    fn window_size_dependent_setup(
        memory_allocator: Arc<StandardMemoryAllocator>,
        images: &[Arc<Image>],
        render_pass: Arc<RenderPass>,
    ) -> anyhow::Result<(Vec<Arc<Framebuffer>>, Viewport)> {
        let depth_buffer = ImageView::new_default(Image::new(
            memory_allocator,
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )?)?;

        let images: anyhow::Result<Vec<Arc<Framebuffer>>> = images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone())?;

                Ok(Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_buffer.clone()],
                        ..Default::default()
                    },
                )?)
            })
            .collect();
        let images = images?;
        let extent = images[0].extent();
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [extent[0] as f32, extent[1] as f32],
            depth_range: 0.0..=1.0,
        };

        Ok((images, viewport))
    }
}
