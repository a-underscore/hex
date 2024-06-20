use crate::Control;
use std::sync::{Arc, RwLock};
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
