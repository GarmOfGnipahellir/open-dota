pub mod bxml;
pub mod loader;
pub mod widget;
pub mod widget_registry;
pub mod widgets;

use bevy::prelude::*;
use bevy_ecss::prelude::*;

pub struct MarkupUiPlugin;

impl Plugin for MarkupUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EcssPlugin::default())
            .init_resource::<widget_registry::AppWidgetRegistry>()
            .init_asset_loader::<bxml::loader::BxmlLoader>();
    }
}
