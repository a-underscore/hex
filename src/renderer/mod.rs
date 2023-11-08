pub mod fragment;
pub mod vertex;

use crate::{
    assets::shape::Vertex2d,
    components::{Camera, Sprite, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use std::sync::Arc;
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        BufferUsage,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    memory::allocator::MemoryTypeFilter,
    padded::Padded,
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
    shader::EntryPoint,
};

pub struct Renderer {
    pub vertex: EntryPoint,
    pub fragment: EntryPoint,
    pub pipeline: Arc<GraphicsPipeline>,
}

impl Renderer {
    pub fn new(context: &Context) -> anyhow::Result<Self> {
        let vertex = vertex::load(context.device.clone())?
            .entry_point("main")
            .unwrap();
        let fragment = fragment::load(context.device.clone())?
            .entry_point("main")
            .unwrap();
        let pipeline = Self::pipeline(context, vertex.clone(), fragment.clone())?;

        Ok(Self {
            vertex,
            fragment,
            pipeline,
        })
    }

    pub fn pipeline(
        context: &Context,
        vertex: EntryPoint,
        fragment: EntryPoint,
    ) -> anyhow::Result<Arc<GraphicsPipeline>> {
        let vertex_input_state =
            Vertex2d::per_vertex().definition(&vertex.info().input_interface)?;
        let stages = [
            PipelineShaderStageCreateInfo::new(vertex),
            PipelineShaderStageCreateInfo::new(fragment),
        ];
        let layout = PipelineLayout::new(
            context.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(context.device.clone())?,
        )?;
        let subpass = Subpass::from(context.render_pass.clone(), 0).unwrap();
        let extent = context.images[0].extent();

        Ok(GraphicsPipeline::new(
            context.device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState {
                    topology: PrimitiveTopology::TriangleFan,
                    ..Default::default()
                }),
                viewport_state: Some(ViewportState {
                    viewports: [Viewport {
                        offset: [0.0, 0.0],
                        extent: [extent[0] as f32, extent[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend::alpha()),
                        ..Default::default()
                    },
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )?)
    }
}

impl System for Renderer {
    fn update(
        &mut self,
        ev: &mut Ev,
        context: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, builder)) = ev {
            if context.recreate_swapchain {
                self.pipeline =
                    Self::pipeline(context, self.vertex.clone(), self.fragment.clone())?;
            }

            if let Some((c, ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get_ref::<Camera>(e)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get_ref::<Transform>(e)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((
                                cm.get_ref::<Sprite>(e)
                                    .and_then(|s| s.active.then_some(s))?,
                                cm.get_ref::<Transform>(e)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                builder.bind_pipeline_graphics(self.pipeline.clone())?;

                for (s, t) in sprites {
                    let view = {
                        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();
                        let subbuffer_allocator = SubbufferAllocator::new(
                            context.memory_allocator.clone(),
                            SubbufferAllocatorCreateInfo {
                                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                                ..Default::default()
                            },
                        );
                        let subbuffer = subbuffer_allocator.allocate_sized()?;

                        *subbuffer.write()? = vertex::View {
                            z: Padded(s.z),
                            transform: t.matrix().0.map(Padded),
                            camera_transform: ct.matrix().0.map(Padded),
                            camera_proj: c.proj().0,
                        };

                        PersistentDescriptorSet::new(
                            &context.descriptor_set_allocator,
                            layout.clone(),
                            [WriteDescriptorSet::buffer(0, subbuffer)],
                            [],
                        )?
                    };
                    let texture = {
                        let layout = self.pipeline.layout().set_layouts().get(1).unwrap();

                        PersistentDescriptorSet::new(
                            &context.descriptor_set_allocator,
                            layout.clone(),
                            [
                                WriteDescriptorSet::sampler(0, s.texture.sampler.clone()),
                                WriteDescriptorSet::image_view(1, s.texture.image.clone()),
                            ],
                            [],
                        )?
                    };
                    let color = {
                        let layout = self.pipeline.layout().set_layouts().get(2).unwrap();
                        let subbuffer_allocator = SubbufferAllocator::new(
                            context.memory_allocator.clone(),
                            SubbufferAllocatorCreateInfo {
                                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                                ..Default::default()
                            },
                        );
                        let subbuffer = subbuffer_allocator.allocate_sized()?;

                        *subbuffer.write()? = fragment::Color { color: s.color };

                        PersistentDescriptorSet::new(
                            &context.descriptor_set_allocator,
                            layout.clone(),
                            [WriteDescriptorSet::buffer(0, subbuffer)],
                            [],
                        )?
                    };

                    builder
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            self.pipeline.layout().clone(),
                            0,
                            view.clone(),
                        )?
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            self.pipeline.layout().clone(),
                            1,
                            texture.clone(),
                        )?
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            self.pipeline.layout().clone(),
                            2,
                            color.clone(),
                        )?
                        .bind_vertex_buffers(0, s.shape.vertices.clone())?
                        .draw(s.shape.vertices.len() as u32, 1, 0, 0)?;
                }
            }
        }

        Ok(())
    }
}
