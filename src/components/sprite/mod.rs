pub mod drawable;
pub mod fragment;
pub mod sprite_drawable;
pub mod vertex;

pub use drawable::Drawable;
pub use sprite_drawable::SpriteDrawable;

use crate::{
    assets::{shape::Vertex2, Shape, Texture},
    component_manager::Component,
    Context,
};
use nalgebra::Vector4;
use std::sync::Arc;
use vulkano::{
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
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

#[derive(Clone)]
pub struct Sprite {
    pub shape: Shape,
    pub texture: Texture,
    pub color: Vector4<f32>,
    pub layer: i32,
    pub drawable: Arc<dyn Drawable>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub shaders: (EntryPoint, EntryPoint),
    pub active: bool,
}

impl Sprite {
    pub fn new(
        shape: Shape,
        texture: Texture,
        color: Vector4<f32>,
        layer: i32,
        pipeline: Arc<GraphicsPipeline>,
        shaders: (EntryPoint, EntryPoint),
        active: bool,
    ) -> Self {
        Self {
            shape,
            texture,
            color,
            layer,
            pipeline,
            shaders,
            drawable: SpriteDrawable::new(),
            active,
        }
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

    pub fn calculate_z(end: i32, layer: i32) -> f32 {
        let end = end as f32;
        let layer = layer as f32;

        -((end - end / 2.0) - layer / 2.0)
    }
}

impl Component for Sprite {}
