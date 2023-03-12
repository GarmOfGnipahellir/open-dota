use bevy::{ecs::world::EntityMut, prelude::*};

use crate::{
    widget::Widget,
    widget_registry::{GetWidgetRegistration, WidgetRegistration},
};
use bevy_markup_ui_derive::Widget;

#[derive(Widget, Default)]
#[bundle]
pub struct Node;

#[derive(Widget, Default)]
#[bundle]
pub struct Button;

#[derive(Widget, Default)]
pub struct Text {
    pub text: String,
}

impl Widget for Text {
    fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w> {
        parent.spawn(TextBundle {
            text: bevy::prelude::Text {
                sections: vec![TextSection::new(self.text.clone(), Default::default())],
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
