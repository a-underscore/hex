pub mod fragment;
pub mod vertex;

use crate::{
    assets::shape::Vertex2d,
    components::{Camera, Sprite, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use std::sync::Arc;
use vulkano::{
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::ViewportState,
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
};

pub struct Renderer {
    pub pipeline: Arc<GraphicsPipeline>,
}

impl Renderer {
    pub fn new(context: &mut Context) -> anyhow::Result<Self> {
        let pipeline = {
            let vs = vertex::load(context.device.clone())?
                .entry_point("main")
                .unwrap();
            let fs = vertex::load(context.device.clone())?
                .entry_point("main")
                .unwrap();
            let vertex_input_state =
                Vertex2d::per_vertex().definition(&vs.info().input_interface)?;
            let stages = [
                PipelineShaderStageCreateInfo::new(vs),
                PipelineShaderStageCreateInfo::new(fs),
            ];
            let layout = PipelineLayout::new(
                context.device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(context.device.clone())?,
            )?;
            let subpass = Subpass::from(context.render_pass.clone(), 0).unwrap();

            GraphicsPipeline::new(
                context.device.clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState {
                        topology: PrimitiveTopology::TriangleStrip,
                        ..Default::default()
                    }),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState {
                            blend: Some(AttachmentBlend::alpha()),
                            ..Default::default()
                        },
                    )),
                    dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )?
        };

        Ok(Self { pipeline })
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
            if let Some((_c, _ct)) = em.entities().find_map(|e| {
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

                for (s, _t) in sprites {
                    let set = {
                        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();

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

                    builder
                        .set_viewport(0, [context.viewport.clone()].into_iter().collect())?
                        .bind_pipeline_graphics(self.pipeline.clone())?
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            self.pipeline.layout().clone(),
                            0,
                            set.clone(),
                        )?
                        .bind_vertex_buffers(0, s.shape.vertices.clone())?
                        .draw(s.shape.vertices.len() as u32, 1, 0, 0)?;
                }
            }
        }

        Ok(())
    }
}
