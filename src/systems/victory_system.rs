use bevy::prelude::*;
use crate::game::{EnemyTower, Health, Tower};
use crate::menu::common::GameState;
use crate::game::components::*;
use crate::ui::money_ui::*;

#[derive(Resource, Default)]
pub struct VictoryState {
    pub game_ended: bool,
    pub victory_timer: Option<Timer>,
    pub defeat_timer: Option<Timer>,
}

#[derive(Component)]
pub struct VictoryScreen;

#[derive(Component)]
pub struct DefeatScreen;

#[derive(Component)]
pub struct VictoryAudio;

#[derive(Component)]
pub struct DefeatAudio;

/// Check win/lose conditions based on tower destruction
pub fn check_victory_conditions(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut victory_state: ResMut<VictoryState>,
    player_towers: Query<&Health, (With<Tower>, Without<EnemyTower>)>,
    enemy_towers: Query<&Health, (With<Tower>, With<EnemyTower>)>,
) {
    if victory_state.game_ended {
        return;
    }

    // Count alive towers
    let player_towers_alive = player_towers.iter().filter(|h| h.current > 0.0).count();
    let enemy_towers_alive = enemy_towers.iter().filter(|h| h.current > 0.0).count();

    // Check victory condition (all enemy towers destroyed)
    if enemy_towers_alive == 0 {
        victory_state.game_ended = true;
        victory_state.victory_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
        
        // Show victory screen
        show_victory_screen(&mut commands, &asset_server);
        
        println!("🎉 VICTORY! All enemy towers destroyed!");
    }
    // Check defeat condition (all player towers destroyed)
    else if player_towers_alive == 0 {
        victory_state.game_ended = true;
        victory_state.defeat_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
        
        // Show defeat screen
        show_defeat_screen(&mut commands, &asset_server);
        
        println!("💀 DEFEAT! All your towers have been destroyed!");
    }
}

fn show_victory_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Victory image
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            image: UiImage::new(asset_server.load("pic/victory.png")),
            z_index: ZIndex::Global(1000),
            ..default()
        },
        VictoryScreen,
    ));

    // Victory audio
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/victory.ogg"),
            settings: PlaybackSettings::ONCE,
        },
        VictoryAudio,
    ));
}

fn show_defeat_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Defeat image
    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            image: UiImage::new(asset_server.load("pic/defeat.png")),
            z_index: ZIndex::Global(1000),
            ..default()
        },
        DefeatScreen,
    ));

    // Defeat audio
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/defeat.ogg"),
            settings: PlaybackSettings::ONCE,
        },
        DefeatAudio,
    ));
}

/// Handle victory/defeat timers and return to main menu
pub fn handle_victory_timers(
    mut commands: Commands,
    mut victory_state: ResMut<VictoryState>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    victory_screens: Query<Entity, With<VictoryScreen>>,
    defeat_screens: Query<Entity, With<DefeatScreen>>,
    victory_audio: Query<Entity, With<VictoryAudio>>,
    defeat_audio: Query<Entity, With<DefeatAudio>>,
    // Query all game entities to clean up
    game_entities: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    // Handle victory timer
    if let Some(ref mut timer) = victory_state.victory_timer {
        timer.tick(time.delta());
        if timer.finished() {
            // Clean up victory screen and audio
            for entity in victory_screens.iter() {
                commands.entity(entity).despawn_recursive();
            }
            for entity in victory_audio.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Clean up all game entities before returning to menu
            cleanup_game_entities(&mut commands, &game_entities);
            
            // Return to main menu
            game_state.set(GameState::Menu);
            victory_state.victory_timer = None;
            victory_state.game_ended = false;
            
            println!("🏠 Returning to main menu after victory");
        }
    }

    // Handle defeat timer
    if let Some(ref mut timer) = victory_state.defeat_timer {
        timer.tick(time.delta());
        if timer.finished() {
            // Clean up defeat screen and audio
            for entity in defeat_screens.iter() {
                commands.entity(entity).despawn_recursive();
            }
            for entity in defeat_audio.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Clean up all game entities before returning to menu
            cleanup_game_entities(&mut commands, &game_entities);
            
            // Return to main menu
            game_state.set(GameState::Menu);
            victory_state.defeat_timer = None;
            victory_state.game_ended = false;
            
            println!("🏠 Returning to main menu after defeat");
        }
    }
}

/// Clean up game entities when returning to menu
fn cleanup_game_entities(
    commands: &mut Commands,
    game_entities: &Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    let mut cleaned_count = 0;
    // Despawn most game entities, but be careful not to despawn cameras or windows
    for entity in game_entities.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
            cleaned_count += 1;
        }
    }
    println!("🧹 Cleaned up {} game entities", cleaned_count);
}
