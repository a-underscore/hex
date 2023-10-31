pub mod control;
pub use control::Control;

use vulkano::render_pass::RenderPass;

pub enum Ev<'a, 'b, 'c> {
    Event(&'a mut Control<'b>),
    Draw((&'a mut Control<'b>, &'c mut RenderPass)),
}
