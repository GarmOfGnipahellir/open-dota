pub mod loader;
pub mod parser;

use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};

#[derive(TypeUuid)]
#[uuid = "0f88e5f2-2903-4bd8-bcae-5772463733c1"]
pub struct Bxml {
    pub templates: HashMap<String, Handle<Scene>>,
    pub scene: Handle<Scene>,
}
