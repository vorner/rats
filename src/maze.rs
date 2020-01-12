use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub enum Field {
    Wall,
    Path,
}

impl Component for Field {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
