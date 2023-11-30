use bevy::ecs::prelude::States;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]    
    Playing,
    GameOver
}