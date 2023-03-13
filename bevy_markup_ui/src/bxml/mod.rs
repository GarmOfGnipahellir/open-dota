use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeRegistryArc,
    utils::BoxedFuture,
};
use roxmltree::Document;

pub struct BxmlLoader {
    type_registry: TypeRegistryArc,
}

impl FromWorld for BxmlLoader {
    fn from_world(world: &mut World) -> Self {
        // let widget_registry = world.resource::<AppWidgetRegistry>();
        let type_registry = world.resource::<AppTypeRegistry>();
        Self {
            // widget_registry: widget_registry.0.clone(),
            type_registry: type_registry.0.clone(),
        }
    }
}

impl AssetLoader for BxmlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            load_bxml(bytes, load_context, &self.type_registry)?;
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["bxml"]
    }
}

fn load_bxml<'a>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
    type_registry: &'a TypeRegistryArc,
) -> Result<()> {
    let doc = Document::parse(std::str::from_utf8(bytes)?)?;
    dbg!(&doc);

    let mut world = World::new();
    let mut root = world.spawn(NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut dependencies = Vec::<AssetPath>::new();

    let type_registry = AppTypeRegistry(type_registry.clone());
    let scene = DynamicScene::from_world(&world, &type_registry);
    load_context.set_default_asset(LoadedAsset::new(scene).with_dependencies(dependencies));

    Ok(())
}

fn load_element() {}
