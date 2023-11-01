pub mod vertex2d;

pub use vertex2d::Vertex2d;

use crate::{ecs::Context, math::Vec2d};
use std::sync::Arc;
use vulkano::{
    buffer::{subbuffer::Subbuffer, Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderingAttachmentInfo, RenderingInfo,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Features,
        QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            subpass::PipelineRenderingCreateInfo,
            vertex_input::VertexDefinition,
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{AttachmentLoadOp, AttachmentStoreOp},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, Version, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Clone)]
pub struct Shape {
    pub vertices: Arc<Subbuffer<[Vertex2d]>>,
}

impl Shape {
    pub fn new(context: &mut Context, vertices: &[Vertex2d]) -> anyhow::Result<Self> {
        Ok(Self {
            vertices: Arc::new(Buffer::from_iter(
                context.memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                vertices.iter().cloned(),
            )?),
        })
    }

    pub fn rect(context: &mut Context, dims: Vec2d) -> anyhow::Result<Self> {
        let vertices = {
            let dims = dims / 2.0;

            [
                Vertex2d::new(Vec2d::new(-dims.x(), -dims.y()), Default::default()),
                Vertex2d::new(Vec2d::new(dims.x(), -dims.y()), Vec2d::new(1.0, 0.0)),
                Vertex2d::new(Vec2d::new(dims.x(), dims.y()), Vec2d::new(1.0, 1.0)),
                Vertex2d::new(Vec2d::new(-dims.x(), dims.y()), Vec2d::new(0.0, 1.0)),
            ]
        };

        Self::new(context, &vertices)
    }
}
