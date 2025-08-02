use bevy::{asset::Handle, ecs::resource::Resource, image::Image, text::Font};

#[derive(Resource)]
pub struct TeamAssets {
    pub rock: Handle<Image>,
    pub paper: Handle<Image>,
    pub scissors: Handle<Image>,
}

#[derive(Resource)]
pub struct TextAssets {
    pub font: Handle<Font>
}
