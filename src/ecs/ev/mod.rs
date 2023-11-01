pub mod control;
pub use control::Control;

use std::sync::Arc;
use vulkano::command_buffer::{
    allocator::{CommandBufferAllocator, StandardCommandBufferAllocator},
    auto::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};

pub enum Ev<'a, 'b, 'c> {
    Event(&'a mut Control<'b>),
    Draw(
        (
            &'a mut Control<'b>,
            &'c mut AutoCommandBufferBuilder<
                PrimaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>,
                Arc<StandardCommandBufferAllocator>,
            >,
        ),
    ),
}
