use bevy::sprite::collide_aabb::collide;
use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::states::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ship)
            .add_systems(
                FixedUpdate, 
                (
                    ship_movement,
                    ship_shooting,
                    update_bullets,
                    check_if_hit_by_bullet,                                      
                    check_if_hit_by_foe,
                    update_pumper,
                    ship_swerving,
                    pumper_animation                 
                ).run_if(in_state(GameState::Playing))
            );
    }
}

fn setup_ship(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
) {
    let ship_texture_handle = asset_server.load(SHIP_SPRITES);
    let pumper_texture_handle = asset_server.load(MISCELLANEOUS_SPRITES);

    let ship_texture_atlas = 
        TextureAtlas::from_grid(ship_texture_handle, Vec2::new(7.9, 7.5), 3, 1, Some(Vec2::new(0.2, 0.2)), None);
    let pumper_texture_atlas =
        TextureAtlas::from_grid(pumper_texture_handle, Vec2::new(8.0, 7.5), 10, 4, None, None);

    let ship_texture_atlas_handle = texture_atlases.add(ship_texture_atlas);
    let pumper_texture_atlas_handle = texture_atlases.add(pumper_texture_atlas);

    // Spawns space ship
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: ship_texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },        
        ShootingDelay(Timer::from_seconds(SHIP_SHOOTING_DELAY, TimerMode::Repeating)),
        Health(SHIP_HEALTH),
        Speed(SHIP_SPEED),
        SpaceShip
    ));

    // Spawn fire pumper
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: pumper_texture_atlas_handle,
            sprite: TextureAtlasSprite::new(15),
            transform: Transform {
                translation: Vec3::new(0.0, -36.0, 0.0),
                scale: Vec3::new(6.0, 6.0, 0.0),
                ..default()
            },
            ..default()
        },
        AnimationIndices::new(15, 18),
        AnimationTimer(Timer::from_seconds(SHIP_PUMPER_ANIMATION_TIME, TimerMode::Repeating)),
        Pumper
    ));
}

fn ship_movement(
    mut ship_query: Query<(&mut Transform, &Speed), With<SpaceShip>>,    
    keyboard_input: Res<Input<KeyCode>>, 
    time: Res<Time>
) {
    if ship_query.is_empty() { return; }

    let (mut ship_transform, speed) = ship_query.single_mut();    

    let current_x = ship_transform.translation.x;
    let current_y = ship_transform.translation.y;

    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard_input.pressed(KeyCode::Left) && !(current_x <= -WINDOW_X_BORDER) {
        horizontal -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) && !(current_x >= WINDOW_X_BORDER) {
        horizontal += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) && !(current_y >= WINDOW_Y_BORDER) {
        vertical += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) && !(current_y <= -WINDOW_Y_BORDER) {
        vertical -= 1.0;
    }

    ship_transform.translation.x += horizontal * speed.0 * time.delta_seconds();
    ship_transform.translation.y += vertical * speed.0 * time.delta_seconds();    
}

fn update_pumper(
    mut pumper_query: Query<&mut Transform, (With<Pumper>, Without<Speed>)>,
    ship_query: Query<&Transform, (With<SpaceShip>, Without<Pumper>)>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if pumper_query.is_empty() { return }

    let mut pumper_transform = pumper_query.single_mut();
    let ship_transform = ship_query.single();

    let mut horizontal_tip = 0.0;
    let mut vertical_tip = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        horizontal_tip -= 13.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        horizontal_tip += 13.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        vertical_tip += 10.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        vertical_tip -= 10.0;
    }

    pumper_transform.translation.x = ship_transform.translation.x + horizontal_tip;
    pumper_transform.translation.y = (ship_transform.translation.y - 36.0) + vertical_tip;
}

fn ship_shooting(
    mut commands: Commands, 
    mut ship_transform: Query<(&Transform, &mut ShootingDelay), With<SpaceShip>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if ship_transform.is_empty() { return }

    let (ship_transform, mut shooting_delay) = ship_transform.single_mut();
    let ship_x = ship_transform.translation.x;
    let ship_y = ship_transform.translation.y;

    if keyboard_input.pressed(KeyCode::Z) {
        if shooting_delay.0.tick(time.delta()).just_finished() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(ship_x - 10.0, ship_y + 15.0, 0.0),
                        scale: Vec3::new(10.0, 20.0, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::ORANGE,
                        ..default()
                    },
                    ..default()
                },            
                Speed(SHIP_BULLET_SPEED),
                Damage(SHIP_DAMAGE),
                Bullet
            ));

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(ship_x + 10.0, ship_y + 15.0, 0.0),
                        scale: Vec3::new(10.0, 20.0, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::ORANGE,
                        ..default()
                    },
                    ..default()
                },            
                Speed(SHIP_BULLET_SPEED),
                Damage(SHIP_DAMAGE),
                Bullet
            ));
        }
    }
}

fn update_bullets(
    mut bullets_query: Query<(Entity, &mut Transform, &Speed), With<Bullet>>, 
    mut commands: Commands,
    time: Res<Time>
) {
    for (bullet_entity, mut transform, speed) in &mut bullets_query {
        transform.translation.y += speed.0 * time.delta_seconds();
        if transform.translation.y > (WINDOW_HEIGHT * 0.5) {
            commands.entity(bullet_entity).despawn();            
        }        
    }
}

fn check_if_hit_by_foe(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    ship_query: Query<(Entity, &Transform), With<SpaceShip>>,
    pumper_query: Query<Entity, With<Pumper>>,
    foes_query: Query<&Transform, With<Foe>>
) {
    if ship_query.is_empty() || foes_query.is_empty() { return; }

    let (ship_entity, ship_transform) = ship_query.single();
    let pumper_entity = pumper_query.single();

    let ship_translation = ship_transform.translation;
    let ship_scale = ship_transform.scale.truncate();

    for f_transform in &foes_query {
        let f_translation = f_transform.translation;
        let f_scale = f_transform.scale.truncate();

        let collision = collide(
            ship_translation,
            ship_scale,
            f_translation,
            f_scale
        );

        if let Some(_) = collision {
            commands.entity(ship_entity).despawn();
            commands.entity(pumper_entity).despawn();
            next_state.set(GameState::GameOver);
        }
    }
}

fn check_if_hit_by_bullet(
    mut commands: Commands, 
    mut next_state: ResMut<NextState<GameState>>,
    mut ship_query: Query<(Entity, &Transform, &mut Health), With<SpaceShip>>,
    foe_bullets_query: Query<(Entity, &Transform, &Damage), With<FoeProjectile>>,
    pumper_query: Query<Entity, With<Pumper>>
) {
    if ship_query.is_empty() { return }

    let (ship_entity, ship_transform, mut health) = ship_query.single_mut();
    let pumper_entity = pumper_query.single();

    let ship_translation = ship_transform.translation;
    let ship_scale = ship_transform.scale.truncate();

    for (bullet_entity, bullet_transform, bullet_damage) in &foe_bullets_query {
        let b_translation = bullet_transform.translation;
        let b_scale = bullet_transform.scale.truncate();

        let collision = collide(
            ship_translation,
            ship_scale,
            b_translation,
            b_scale
        );

        if let Some(_) = collision {
            commands.entity(bullet_entity).despawn();
            health.0 -= bullet_damage.0;
        }
    }

    if health.0 <= 0 {
        commands.entity(ship_entity).despawn();
        commands.entity(pumper_entity).despawn();
        next_state.set(GameState::GameOver);
    }
}

fn ship_swerving(
    mut ship_query: Query<&mut TextureAtlasSprite, With<SpaceShip>>,    
    keyboard_input: Res<Input<KeyCode>>
) {
    if ship_query.is_empty() { return }

    let mut texture_atlas = ship_query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {        
        texture_atlas.index = 0;
    } else if keyboard_input.pressed(KeyCode::Right) {        
        texture_atlas.index = 2;
    } else {
        texture_atlas.index = 1;
    }
}

fn pumper_animation(
    mut pumper_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &AnimationIndices), With<Pumper>>,
    time: Res<Time>
) {
    if pumper_query.is_empty() { return }

    let (mut texture_atlas, mut timer, indices) = pumper_query.single_mut();
    if timer.0.tick(time.delta()).just_finished() {
        texture_atlas.index = if texture_atlas.index != indices.last {
            texture_atlas.index + 1        
        } else {
            indices.first
        };
    }
}