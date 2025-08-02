use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Borders {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}
