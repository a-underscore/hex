pub mod control;
pub use control::Control;

use vulkano::render_pass::RenderPass;

pub enum Ev<'a, 'b> {
    Event(&'a mut Control),
    Draw((&'a mut Control, &'b mut RenderPass)),
}
