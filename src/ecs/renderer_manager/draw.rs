use crate::ecs::Control;
use std::sync::{Arc, RwLock};
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator,
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};

pub struct Draw<'a>(
    pub Arc<RwLock<Control>>,
    pub  &'a mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
        Arc<StandardCommandBufferAllocator>,
    >,
);
