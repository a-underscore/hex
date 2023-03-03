pub mod control;

pub use control::Control;

use glium::Frame;

pub enum Ev<'a, 'b, 'c, 'd> {
    Event(&'a mut Control<'b, 'c>),
    Draw((&'a mut Control<'b, 'c>, &'d mut Frame)),
}
