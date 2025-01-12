pub mod fragment;
pub mod sprite_drawable;
pub mod vertex;

pub use sprite_drawable::SpriteDrawable;

use crate::{
    assets::{shape::Vertex2, Shape, Texture},
    components::Trans,
    Context, Drawable, Id,
};
use nalgebra::Vector4;
use parking_lot::RwLock;
use std::sync::Arc;
use vulkano::{
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{CompareOp, DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::ViewportState,
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
    shader::EntryPoint,
};

pub type SpriteEntity = (Id, Arc<RwLock<Sprite>>, Arc<RwLock<Trans>>);
pub type SpritePipeline = (Arc<GraphicsPipeline>, EntryPoint, EntryPoint);

#[derive(Clone)]
pub struct Sprite {
    pub shape: Shape,
    pub texture: Texture,
    pub color: Vector4<f32>,
    pub layer: u32,
    pub drawable: Arc<dyn Drawable<SpriteEntity>>,
    pub pipeline: SpritePipeline,
}

impl Sprite {
    pub fn new(
        context: &Context,
        shape: Shape,
        texture: Texture,
        color: Vector4<f32>,
        layer: u32,
    ) -> anyhow::Result<Arc<RwLock<Self>>> {
        let vertex = vertex::load(context.device.clone())?
            .entry_point("main")
            .unwrap();
        let fragment = fragment::load(context.device.clone())?
            .entry_point("main")
            .unwrap();

        Ok(Arc::new(RwLock::new(Self {
            shape,
            texture,
            color,
            layer,
            pipeline: (
                Self::pipeline(context, vertex.clone(), fragment.clone())?,
                vertex,
                fragment,
            ),
            drawable: SpriteDrawable::new(),
        })))
    }

    pub fn recreate_pipeline(&mut self, context: &Context) -> anyhow::Result<()> {
        let (ref mut pipeline, ref vertex, ref fragment) = self.pipeline;

        *pipeline = Self::pipeline(context, vertex.clone(), fragment.clone())?;

        Ok(())
    }

    pub fn pipeline(
        context: &Context,
        vertex: EntryPoint,
        fragment: EntryPoint,
    ) -> anyhow::Result<Arc<GraphicsPipeline>> {
        let vertex_input_state =
            Vertex2::per_vertex().definition(&vertex.info().input_interface)?;
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
                    viewports: [context.viewport.clone()].into_iter().collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState {
                        write_enable: true,
                        compare_op: CompareOp::LessOrEqual,
                    }),
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
