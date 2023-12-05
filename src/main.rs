use bevy::window::{WindowMode, WindowTheme};
use bevy::prelude::*;
use bevy_parallax::ParallaxCameraComponent;

use space_shooter::background::*;
use space_shooter::components::*;
use space_shooter::constants::*;
use space_shooter::states::*;

use space_shooter::player::*;
use space_shooter::foe::*;

fn setup(mut commands: Commands) {
    // Spawns 2d camera
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn(Camera2dBundle::default()).insert(ParallaxCameraComponent::default());    
}

fn setup_gameover(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        GameBanner
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font_size: 80.0,
                    color: Color::PURPLE,
                    ..default()
                }
            )
        );
    });
}

fn check_to_reset(mut next_state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing)
    }
}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

struct SpaceShooterPlugin;

impl Plugin for SpaceShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((BackgroundPlugin, PlayerPlugin, FoePlugin))
            .add_systems(Startup, setup)                        
            .add_systems(OnEnter(GameState::GameOver), setup_gameover)
            .add_systems(Update, check_to_reset.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), teardown);
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space shooter".to_string(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    mode: WindowMode::Windowed,
                    focused: true,
                    resizable: false,
                    visible: true,
                    window_theme: Some(WindowTheme::Dark),                    
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()), 
            SpaceShooterPlugin
        ))
        .run()
}
