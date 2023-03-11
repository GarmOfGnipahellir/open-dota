use bevy::{ecs::world::EntityMut, prelude::*};

use crate::{
    widget::Widget,
    widget_registry::{GetWidgetRegistration, WidgetRegistration},
};

pub struct Node;

impl GetWidgetRegistration for Node {
    fn get_widget() -> Box<dyn Widget> {
        Box::new(Self)
    }

    fn get_widget_registration() -> WidgetRegistration {
        WidgetRegistration::of::<Self>()
    }
}

impl Widget for Node {
    fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w> {
        parent.spawn(NodeBundle::default())
    }
}

pub struct Button;

impl GetWidgetRegistration for Button {
    fn get_widget() -> Box<dyn Widget> {
        Box::new(Self)
    }

    fn get_widget_registration() -> WidgetRegistration {
        WidgetRegistration::of::<Self>()
    }
}

impl Widget for Button {
    fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w> {
        parent.spawn(ButtonBundle::default())
    }
}

pub struct Text;

impl GetWidgetRegistration for Text {
    fn get_widget() -> Box<dyn Widget> {
        Box::new(Self)
    }

    fn get_widget_registration() -> WidgetRegistration {
        WidgetRegistration::of::<Self>()
    }
}

impl Widget for Text {
    fn spawn<'w>(&self, parent: &'w mut WorldChildBuilder) -> EntityMut<'w> {
        parent.spawn(TextBundle::default())
    }
}
