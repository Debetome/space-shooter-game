use bevy::prelude::*;
use bevy_parallax::{
    LayerSpeed, LayerData, ParallaxPlugin, ParallaxMoveEvent, CreateParallaxEvent
};

use crate::constants::*;
use crate::states::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParallaxPlugin)
            .add_systems(OnEnter(GameState::Playing), setup_background)
            .add_systems(Update, move_background.run_if(in_state(GameState::Playing)));
    }
}

fn setup_background(    
    mut create_parallax: EventWriter<CreateParallaxEvent>,
    camera_query: Query<Entity, With<Camera>>
) {    
    let camera_entity = camera_query.single();    

    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Vertical(0.85),
                path: BACKGROUND_LAYER_1.to_string(),
                tile_size: Vec2::new(BACKGROUND_WIDTH, BACKGROUND_HEIGHT),
                cols: 4,
                rows: 4,
                scale: 2.5,
                z: 1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Vertical(0.95),
                path: BACKGROUND_LAYER_2.to_string(),
                tile_size: Vec2::new(BACKGROUND_WIDTH, BACKGROUND_HEIGHT),
                cols: 4,
                rows: 4,
                scale: 2.5,
                z: 0.0,
                ..default()
            }
        ],
        camera: camera_entity
    });
}

fn move_background(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>
) {
    let camera_entity = camera_query.single();

    move_event_writer.send(ParallaxMoveEvent { 
        camera_move_speed: Vec2::new(0.0, 2.0), 
        camera: camera_entity
    });
}