use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    ecs::world::EntityMut,
    prelude::*,
    reflect::TypeRegistryArc,
    utils::BoxedFuture,
};
use bevy_ecss::StyleSheet;
use roxmltree::{Document, Node};
use thiserror::Error;

use crate::widget_registry::{AppWidgetRegistry, WidgetRegistry, WidgetRegistryArc};

#[derive(Error, Debug)]
pub enum BxmlError {
    #[error("found Style element without src attribute")]
    StyleWithoutSrc,
}

pub struct BxmlLoader {
    widget_registry: WidgetRegistryArc,
    type_registry: TypeRegistryArc,
}

impl FromWorld for BxmlLoader {
    fn from_world(world: &mut World) -> Self {
        let widget_registry = world.resource::<AppWidgetRegistry>();
        let type_registry = world.resource::<AppTypeRegistry>();
        Self {
            widget_registry: widget_registry.0.clone(),
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
            load_bxml(
                bytes,
                load_context,
                &self.widget_registry,
                &self.type_registry,
            )?;
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
    widget_registry: &'a WidgetRegistryArc,
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

    let widget_registry = widget_registry.read();
    let mut dependencies = Vec::new();
    for child in doc.root_element().children() {
        load_node(
            &child,
            &mut root,
            &mut dependencies,
            load_context,
            &widget_registry,
        )?;
    }

    let type_registry = AppTypeRegistry(type_registry.clone());
    let scene = DynamicScene::from_world(&world, &type_registry);
    load_context.set_default_asset(LoadedAsset::new(scene).with_dependencies(dependencies));

    Ok(())
}

fn load_node(
    node: &Node,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
    widget_registry: &WidgetRegistry,
) -> Result<()> {
    if node.is_element() {
        load_element(node, parent, dependencies, load_context, widget_registry)?;
    }

    Ok(())
}

fn load_element(
    node: &Node,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
    widget_registry: &WidgetRegistry,
) -> Result<()> {
    let name = node.tag_name().name();
    match name {
        "Style" => load_style(node, parent, dependencies, load_context)?,
        _ => load_widget(node, parent, dependencies, load_context, widget_registry)?,
    }
    Ok(())
}

fn load_style(
    node: &Node,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
) -> Result<()> {
    let src = node.attribute("src").ok_or(BxmlError::StyleWithoutSrc)?;
    let asset_path = AssetPath::new(src.into(), None);
    let handle = load_context.get_handle(asset_path.clone());
    parent.insert(StyleSheet::new(handle));
    dependencies.push(asset_path);
    Ok(())
}

fn load_widget(
    node: &Node,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
    widget_registry: &WidgetRegistry,
) -> Result<()> {
    let tag_name = node.tag_name().name();
    let text_content = node.text();

    let mut res = Ok(());
    parent.with_children(|parent| {
        let Some(widget) = widget_registry.get_with_name(tag_name).map(|registration| registration.widget()) else {
            warn!("unkown widget of name: {}", tag_name);
            return;
        };

        let mut entity = widget.spawn(parent);

        if let Some((content, mut text)) = text_content.zip(entity.get_mut::<Text>()) {
            text.sections = vec![TextSection::new(content, Default::default())];
        }

        if let Some(name) = node.attribute("name") {
            entity.insert(Name::new(name.to_string()));
        }

        // TODO: add classes

        for child in node.children() {
            if let Err(err) = load_node(
                &child,
                &mut entity,
                dependencies,
                load_context,
                widget_registry,
            ) {
                res = Err(err);
                return;
            }
        }
    });
    res
}
