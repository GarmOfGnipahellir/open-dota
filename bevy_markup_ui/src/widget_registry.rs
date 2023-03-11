use std::{any::TypeId, sync::Arc};

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{widget::Widget, widgets::Node};

pub struct WidgetRegistry {
    registrations: HashMap<TypeId, WidgetRegistration>,
    name_to_id: HashMap<String, TypeId>,
    ambiguous_names: HashSet<String>,
}

#[derive(Default, Clone)]
pub struct WidgetRegistryArc {
    pub internal: Arc<RwLock<WidgetRegistry>>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AppWidgetRegistry(pub WidgetRegistryArc);

pub struct WidgetRegistration {
    name: String,
    type_id: TypeId,
    widget: Box<dyn Widget>,
}

pub trait GetWidgetRegistration {
    fn get_widget() -> Box<dyn Widget>;
    fn get_widget_registration() -> WidgetRegistration;
}

impl Default for WidgetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetRegistry {
    pub fn empty() -> Self {
        Self {
            registrations: Default::default(),
            name_to_id: Default::default(),
            ambiguous_names: Default::default(),
        }
    }

    pub fn new() -> Self {
        let mut registry = Self::empty();
        registry.register::<Node>();
        registry
    }

    pub fn register<W>(&mut self)
    where
        W: GetWidgetRegistration + 'static,
    {
        let registration = W::get_widget_registration();
        self.name_to_id
            .insert(registration.name.to_lowercase(), registration.type_id);
        self.registrations
            .insert(registration.type_id, registration);
    }

    pub fn get(&self, type_id: TypeId) -> Option<&WidgetRegistration> {
        self.registrations.get(&type_id)
    }

    pub fn get_with_name(&self, name: &str) -> Option<&WidgetRegistration> {
        self.name_to_id
            .get(&name.to_lowercase())
            .and_then(|type_id| self.registrations.get(type_id))
    }
}

impl WidgetRegistryArc {
    pub fn read(&self) -> RwLockReadGuard<'_, WidgetRegistry> {
        self.internal.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, WidgetRegistry> {
        self.internal.write()
    }
}

impl WidgetRegistration {
    pub fn of<W: GetWidgetRegistration + 'static>() -> Self {
        let type_name = std::any::type_name::<W>();
        Self {
            name: bevy::utils::get_short_name(type_name),
            type_id: TypeId::of::<W>(),
            widget: W::get_widget(),
        }
    }

    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }
}
