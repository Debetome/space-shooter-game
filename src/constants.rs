// Window values
pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const WINDOW_X_BORDER: f32 = WINDOW_WIDTH * 0.5;
pub const WINDOW_Y_BORDER: f32 = WINDOW_HEIGHT * 0.5;

// Sprite textures paths
pub const SHIP_SPRITES: &'static str = "textures\\SpaceShooterAssetPack_Ships.png";
pub const PROJECTILE_SPRITES: &'static str = "textures\\SpaceShooterAssetPack_Projectiles.png";
pub const MISCELLANEOUS_SPRITES: &'static str = "textures\\SpaceShooterAssetPack_Miscellaneous.png";
pub const UI_SPRITES: &'static str = "textures\\SpaceShooterAssetPack_IU.png";

// Ship/player values
pub const SHIP_SPEED: f32 = 530.0;
pub const SHIP_DAMAGE: i32 = 2;
pub const SHIP_HEALTH: i32 = 10;
pub const SHIP_BULLET_SPEED: f32 = 980.0;
pub const SHIP_SHOOTING_DELAY: f32 = 0.12;
pub const SHIP_PUMPER_ANIMATION_TIME: f32 = 0.07;

// Foe values
pub const FOE_SPEED: f32 = 230.0;
pub const FOE_DAMAGE: i32 = 2;
pub const FOE_HEALTH: i32 = 12;
pub const FOE_SPAWN_DELAY: f32 = 1.5;
pub const FOE_UNITS: f32 = 32.0;
pub const FOE_UNIT_WIDTH: f32 = WINDOW_WIDTH / FOE_UNITS;
pub const FOE_SHOOT_DELAY: f32 = 1.2;
pub const FOE_PROJECTILE_SPEED: f32 = 500.0;