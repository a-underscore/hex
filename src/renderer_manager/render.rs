use crate::ecs::Control;
use std::sync::{Arc, RwLock};
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator,
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};

pub type Render<'a> = (
    Arc<RwLock<Control>>,
    &'a mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
        Arc<StandardCommandBufferAllocator>,
    >,
    bool,
);
