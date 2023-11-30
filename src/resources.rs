use bevy::ecs::system::Resource;
use bevy::sprite::TextureAtlas;
use bevy::time::Timer;

#[derive(Resource)]
pub struct SpriteAtlas(pub TextureAtlas);

#[derive(Resource)]
pub struct EnemySpawnDelay(pub Timer);