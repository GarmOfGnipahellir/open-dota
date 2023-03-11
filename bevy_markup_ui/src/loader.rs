use anyhow::Result;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset},
    ecs::world::EntityMut,
    prelude::*,
    reflect::TypeRegistryArc,
    utils::BoxedFuture,
};
use bevy_ecss::{Class, StyleSheet};
use ego_tree::NodeRef;
use scraper::{node::Element, ElementRef, Html, Node};
use thiserror::Error;

use crate::widget_registry::{AppWidgetRegistry, WidgetRegistry, WidgetRegistryArc};

#[derive(Error, Debug)]
pub enum HtmlError {
    #[error("invalid utf-8 text: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

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
            load_html(
                bytes,
                load_context,
                &self.widget_registry,
                &self.type_registry,
            )?;
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["html"]
    }
}

fn load_html<'a>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
    widget_registry: &'a WidgetRegistryArc,
    type_registry: &'a TypeRegistryArc,
) -> Result<(), HtmlError> {
    let html_str = std::str::from_utf8(bytes)?;
    let html = Html::parse_fragment(html_str);

    for err in &html.errors {
        error!("{err}");
    }

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
    for child in html.root_element().children() {
        load_node(
            child,
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
    node: NodeRef<Node>,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
    widget_registry: &WidgetRegistry,
) -> Result<(), HtmlError> {
    let Some(element_ref) = ElementRef::wrap(node) else {
        return Ok(());
    };
    let element = element_ref.value();
    let name = element.name();

    if name == "link" {
        return load_link(element, parent, dependencies, load_context);
    }

    let mut res = Ok(());

    parent.with_children(|parent| {
        let Some(widget) = widget_registry.get_with_name(name).map(|registration| registration.widget()) else {
            warn!("unkown widget of name: {}", name);
            return;
        };
        let mut entity = widget.spawn(parent);
        if let Some(id) = element.id() {
            entity.insert(Name::new(id.to_string()));
        }
        for class in element.classes() {
            entity.insert(Class::new(class.to_string()));
        }

        for child in element_ref.children() {
            if let Err(err) = load_node(
                child,
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

fn load_link(
    element: &Element,
    parent: &mut EntityMut,
    dependencies: &mut Vec<AssetPath>,
    load_context: &mut LoadContext,
) -> Result<(), HtmlError> {
    let Some(rel) = element.attr("rel") else {
        warn!("link without rel: {element:?}");
        return Ok(());
    };

    if rel == "stylesheet" {
        let Some(href) = element.attr("href") else {
            warn!("stylsheet link without href: {element:?}");
            return Ok(());
        };

        let asset_path = AssetPath::new(href.into(), None);
        let handle = load_context.get_handle(asset_path.clone());
        let style_sheet = StyleSheet::new(handle);
        parent.insert(style_sheet);
        dependencies.push(asset_path);
    } else {
        warn!("unsupported link rel: {rel}");
    }

    Ok(())
}
