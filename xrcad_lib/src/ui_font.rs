use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct UiFont(pub Handle<Font>);

impl UiFont {
    pub fn font(&self) -> &Handle<Font> {
        &self.0
    }
}

pub fn setup_ui_font(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font: Handle<Font> = asset_server.load("fonts/FiraCode-Light.ttf");
    commands.insert_resource(UiFont(font));
}
