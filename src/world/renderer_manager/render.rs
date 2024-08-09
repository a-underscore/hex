use crate::ecs::Control;
use std::sync::Arc;
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator,
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};
use parking_lot::RwLock;

pub type Render<'a> = (
    Arc<RwLock<Control>>,
    &'a mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
        Arc<StandardCommandBufferAllocator>,
    >,
    bool,
);
