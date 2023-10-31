use super::{ev::Control, ComponentManager, EntityManager, SystemManager};
use std::{error::Error, sync::Arc};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::{CommandBufferAllocator, StandardCommandBufferAllocator},
        AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage,
        CopyBufferToImageInfo, PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{
        sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
        view::ImageView,
        Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
    },
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
    sync::{self, GpuFuture, NowFuture},
    DeviceSize, Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Context {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub uploads: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>, StandardCommandBufferAllocator>,
    pub event_loop: EventLoop<()>,
    pub swapchain: Arc<Swapchain>,
    pub window: Arc<Window>,
    pub viewport: Arc<Viewport>,
    pub em: EntityManager,
    pub cm: ComponentManager,
    pub sm: SystemManager,
    pub bg: [f32; 4],
}

impl Context {
    pub fn new(
        (em, cm): (EntityManager, ComponentManager),
        sm: SystemManager,
        bg: [f32; 4],
    ) -> anyhow::Result<Self> {
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
        let (mut swapchain, images) = {
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
            },
            pass: {
                color: [color],
                depth_stencil: {},
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
        let uploads = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        Ok(Self {
            uploads,
            command_buffer_allocator,
            descriptor_set_allocator,
            window,
            device,
            queue,
            memory_allocator,
            swapchain,
            event_loop,
            em,
            cm,
            sm,
            bg,
        })
    }

    pub fn init(mut self) -> anyhow::Result<()> {
        self.sm.init(&mut self, (&mut self.em, &mut self.cm))?;

        let mut recreate_swapchain = false;
        let mut previous_frame_end =
            Some(self.uploads.build()?.execute(self.queue.clone())?.boxed());
        let thing: String = previous_frame_end;

        self.event_loop.run(move |event, elwt| {
            if let Err(e) = self.update(
                Control::new(event, elwt),
                &mut recreate_swapchain,
                &mut previous_frame_end,
            ) {
                eprintln!("{}", e);
            }
        });

        Ok(())
    }

    pub fn update(
        &mut self,
        mut control: Control,
        recreate_swapchain: &mut bool,
        previous_frame_end: &mut Option<CommandBufferExecFuture<NowFuture>>,
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
                *recreate_swapchain = true;
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let image_extent: [u32; 2] = self.window.inner_size().into();

                if image_extent.contains(&0) {
                    return Ok(());
                }

                previous_frame_end.as_mut()?.cleanup_finished();

                if *recreate_swapchain {
                    let (new_swapchain, new_images) = self
                        .swapchain
                        .recreate(SwapchainCreateInfo {
                            image_extent,
                            ..self.swapchain.create_info()
                        })
                        .expect("failed to recreate swapchain");

                    self.swapchain = new_swapchain;
                    self.framebuffers = Self::window_size_dependent_setup(
                        &new_images,
                        self.render_pass.clone(),
                        &mut self.viewport,
                    );
                    recreate_swapchain = false;
                }
                /*

                let (image_index, suboptimal, acquire_future) =
                    match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            recreate_swapchain = true;

                            return Ok(());
                        }
                        Err(e) => return Err(e.into()),
                    };

                if suboptimal {
                    *recreate_swapchain = true;
                }

                let mut builder = AutoCommandBufferBuilder::primary(
                    &self.command_buffer_allocator,
                    self.queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                ?;
                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                            ..RenderPassBeginInfo::framebuffer(
                                self.framebuffers[image_index as usize].clone(),
                            )
                        },
                        Default::default(),
                    )
                    ?
                    .set_viewport(0, [self.viewport.clone()].into_iter().collect())
                    ?
                    .bind_pipeline_graphics(self.pipeline.clone())
                    ?
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        set.clone(),
                    )
                    ?
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    ?
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    ?
                    .end_render_pass(Default::default())
                    ?;
                let command_buffer = builder.build()?;

                let future = previous_frame_end
                    .take()
                    ?
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    ?
                    .then_swapchain_present(
                        queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                    )
                    .then_signal_fence_and_flush();

                match future.map_err(Validated::unwrap) {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(VulkanError::OutOfDate) => {
                        *recreate_swapchain = true;

                        previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                    Err(e) => {
                        previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                }
                */
            }
            Event::AboutToWait => self.window.request_redraw(),
            _ => (),
        }

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
            .map(|image| ImageView::new_default(image.clone()).unwrap())
            .collect()
    }
}
