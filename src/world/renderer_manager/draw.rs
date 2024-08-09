use crate::Control;
use parking_lot::RwLock;
use std::sync::Arc;
use vulkano::command_buffer::{
    allocator::StandardCommandBufferAllocator,
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};

pub type Draw<'a> = (
    Arc<RwLock<Control>>,
    &'a mut AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
        Arc<StandardCommandBufferAllocator>,
    >,
    bool,
);
