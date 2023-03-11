use bevy::{ecs::world::EntityMut, prelude::*};

use crate::{
    widget::Widget,
    widget_registry::{GetWidgetRegistration, WidgetRegistration},
};
use bevy_markup_ui_derive::Widget;

#[derive(Widget)]
#[bundle]
pub struct Node;

#[derive(Widget)]
#[bundle]
pub struct Button;

#[derive(Widget)]
#[bundle]
pub struct Text;
