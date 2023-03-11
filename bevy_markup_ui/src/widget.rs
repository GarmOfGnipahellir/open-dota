use bevy::{ecs::world::EntityMut, prelude::WorldChildBuilder};

pub trait Widget: Send + Sync {
    fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w>;
}
