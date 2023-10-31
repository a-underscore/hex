use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
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
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            subpass::PipelineRenderingCreateInfo,
            vertex_input::{Vertex, VertexDefinition},
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

#[derive(Default)]
pub struct Renderer;

impl System for Renderer {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Event::RedrawRequested = ev {
            if let Some((c, ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get::<Camera>(e).and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Sprite>(e).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                for (s, t) in sprites {
                    s.draw(t);
                }
            }
        }

        Ok(())
    }
}
