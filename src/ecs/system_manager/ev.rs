use glium::{glutin::event::Event, Frame};

pub enum Ev<'a, 'b, 'c> {
    Event(&'a Event<'b, ()>),
    Draw((&'a Event<'b, ()>, &'c mut Frame)),
}
