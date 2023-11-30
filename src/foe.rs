use bevy::sprite::collide_aabb::collide;
use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::states::*;
use crate::utils::*;

pub struct FoePlugin;

impl Plugin for FoePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_foes)  
            .add_systems(
                FixedUpdate, 
                (
                    spawn_foes,
                    update_foes,                    
                    check_if_foe_hit,
                    foes_shooting,
                    update_foe_projectiles
                ).run_if(in_state(GameState::Playing))
            );
    }
}

fn setup_foes(mut commands: Commands) {
    // Insert enemy spawning delay, so they don't spawn too quickly
    commands.insert_resource(EnemySpawnDelay(Timer::from_seconds(FOE_SPAWN_DELAY, TimerMode::Repeating)));
}

fn spawn_foes(
    mut commands: Commands, 
    mut foe_timer: ResMut<EnemySpawnDelay>,
    time: Res<Time>
) {
    if foe_timer.0.tick(time.delta()).just_finished() {
        let random_unit = rand::thread_rng().gen_range(-15..=15) as f32;
        let foe_x = FOE_UNIT_WIDTH * random_unit;
        let foe_y = (WINDOW_HEIGHT * 0.5) + 50.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(foe_x, foe_y, 0.0),
                    scale: Vec3::new(30.0, 30.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                ..default()                
            },            
            ShootingDelay(Timer::from_seconds(FOE_SHOOT_DELAY, TimerMode::Repeating)),
            Health(FOE_HEALTH),
            Speed(FOE_SPEED),
            Foe
        ));
    }
}

fn update_foes(
    mut commands: Commands,
    mut foes: Query<(Entity, &mut Transform, &Speed), With<Foe>>,
    time: Res<Time>
) {
    for (foe_entity, mut transform, speed) in &mut foes {
        transform.translation.y += -speed.0 * time.delta_seconds();
        if transform.translation.y < ((-WINDOW_HEIGHT * 0.5) - 25.0) {
            commands.entity(foe_entity).despawn();
        }
    }
}

fn foes_shooting(
    mut commands: Commands,
    mut foes: Query<(&Transform, &mut ShootingDelay), With<Foe>>,
    player: Query<&Transform, With<SpaceShip>>,
    time: Res<Time>
) {
    if foes.is_empty() || player.is_empty() { return }

    let player = player.single();
    let player_x = player.translation.x;
    let player_y = player.translation.y;

    for (f_transform, mut shoot_delay) in &mut foes {
        let foe_x = f_transform.translation.x;
        let foe_y = f_transform.translation.y;

        if shoot_delay.0.tick(time.delta()).just_finished() {            
            let (dir_x, dir_y) = calc_bullet_direction(
                foe_x, 
                foe_y, 
                player_x, 
                player_y                
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(foe_x, foe_y, 0.0),
                        scale: Vec3::new(20.0, 20.0, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    ..default()
                },
                GDirection::new(dir_x, dir_y),                
                Speed(FOE_PROJECTILE_SPEED),
                Damage(FOE_DAMAGE),
                FoeProjectile,                
            ));
        }
    }
}

fn update_foe_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &GDirection, &Speed), With<FoeProjectile>>,
    time: Res<Time>
) {
    for (entity, mut transform, gdirection, speed) in &mut projectiles {            
        if transform.translation.y < (-WINDOW_HEIGHT * 0.5) 
        || transform.translation.x < (-WINDOW_WIDTH * 0.5)
        || transform.translation.x > (WINDOW_WIDTH * 0.5) {
            commands.entity(entity).despawn();
        }

        transform.translation.x += gdirection.x * speed.0 * time.delta_seconds();
        transform.translation.y += gdirection.y * speed.0 * time.delta_seconds();  
    }
}

fn check_if_foe_hit(
    mut commands: Commands,
    mut foes: Query<(Entity, &Transform, &mut Health), With<Foe>>,
    bullets: Query<(Entity, &Transform, &Damage), With<Bullet>>
) {
    for (foe_entity, foe_transform, mut foe_health) in &mut foes {
        for (bullet_entity, bullet_transform, bullet_damage) in &bullets {
            let bullet_translation = bullet_transform.translation;
            let bullet_scale = bullet_transform.scale.truncate();
            let foe_translation = foe_transform.translation;
            let foe_scale = foe_transform.scale.truncate();

            let collision = collide(
                bullet_translation,
                bullet_scale,
                foe_translation,
                foe_scale
            );

            if let Some(_) = collision {
                commands.entity(bullet_entity).despawn();
                foe_health.0 -= bullet_damage.0;
            }
        }

        if foe_health.0 <= 0 {
            commands.entity(foe_entity).despawn();                
        }
    }
}