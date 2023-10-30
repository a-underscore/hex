use super::{ComponentManager, Control, EntityManager, SystemManager};
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo,
        QueueFlags,
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
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Clone)]
pub struct Context {
    pub display: Display,
    pub bg: [f32; 4],
}

impl Context {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self { display, bg }
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        (mut em, mut cm): (EntityManager, ComponentManager),
        mut sm: SystemManager,
    ) -> anyhow::Result<()> {
        sm.init(&mut self, (&mut em, &mut cm))?;

        event_loop.run(move |event, _, cf| {
            if let Err(e) = self.update(Control::new(event), cf, (&mut em, &mut cm), &mut sm) {
                eprintln!("{}", e);
            }
        })
    }

    pub fn update(
        &mut self,
        mut control: Control,
        cf: &mut ControlFlow,
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

                sm.update(&mut Ev::Draw((&mut control, &mut target)), self, (em, cm))?;

                target.finish()?;
            }
            _ => {
                sm.update(&mut Ev::Event(&mut control), self, (em, cm))?;

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
}
