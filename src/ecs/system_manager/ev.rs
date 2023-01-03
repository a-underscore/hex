use cgmath::Vector4;
use glium::{glutin::event::Event, Frame};

pub enum Ev<'a, 'b, 'c, 'd> {
    Event(&'a Event<'b, ()>),
    Draw((&'a Event<'b, ()>, &'c mut Frame, &'d mut Vector4<f32>)),
}
