use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeRegistryArc,
    utils::BoxedFuture,
};
use bevy_ecss::{Class, StyleSheet};
use scraper::{ElementRef, Html};

use crate::widget_registry::{AppWidgetRegistry, WidgetRegistryArc};

pub struct HtmlLoader {
    widget_registry: WidgetRegistryArc,
    type_registry: TypeRegistryArc,
}

impl FromWorld for HtmlLoader {
    fn from_world(world: &mut World) -> Self {
        let widget_registry = world.resource::<AppWidgetRegistry>();
        let type_registry = world.resource::<AppTypeRegistry>();
        Self {
            widget_registry: widget_registry.0.clone(),
            type_registry: type_registry.0.clone(),
        }
    }
}

impl AssetLoader for HtmlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let html_str = std::str::from_utf8(bytes)?;
            let html = Html::parse_fragment(html_str);
            let style_path = AssetPath::new("button.css".into(), None);

            let mut world = World::new();
            let mut root = world.spawn(NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
            root.insert(StyleSheet::new(load_context.get_handle(style_path.clone())));

            let widget_registry = self.widget_registry.read();
            let elem = ElementRef::wrap(html.root_element().first_child().unwrap()).unwrap();
            let name = elem.value().name();
            root.with_children(|parent| {
                let mut entity = widget_registry
                    .get_with_name(name)
                    .unwrap()
                    .widget()
                    .spawn(parent);
                if let Some(id) = elem.value().id() {
                    entity.insert(Name::new(id.to_string()));
                }
                for class in elem.value().classes() {
                    entity.insert(Class::new(class.to_string()));
                }
            });

            let type_registry = AppTypeRegistry(self.type_registry.clone());
            let scene = DynamicScene::from_world(&world, &type_registry);
            load_context.set_default_asset(LoadedAsset::new(scene).with_dependency(style_path));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["html"]
    }
}

// fn spawn_node(parent: EntityMut, elem: ElementRef) {
//     for elem.traverse()
//     for child in elem.children() {
//         child.
//     }
// }
