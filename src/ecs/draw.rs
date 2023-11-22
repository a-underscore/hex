use super::Control;
use std::sync::Arc;
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator,
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};

pub struct Draw<'a, 'b>(
    pub &'a mut Control,
    pub  &'b mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
        Arc<StandardCommandBufferAllocator>,
    >,
);
