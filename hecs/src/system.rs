pub trait System {
    pub fn update(&self, world: Rc<World>);
}
