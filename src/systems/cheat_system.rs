use bevy::prelude::*;
use crate::ui::money_ui::{Money, Wood, Iron, Steel, Oil};
use crate::systems::turn_system::TurnState;
use crate::systems::victory_system::{VictoryState, TwitterConfig};
use crate::game::{EnemyTower, Health, Tower};
use crate::menu::common::GameState;

/// Handle cheat key inputs
pub fn handle_cheat_keys(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    mut steel: ResMut<Steel>,
    mut oil: ResMut<Oil>,
    mut turn_state: ResMut<TurnState>,
    mut victory_state: ResMut<VictoryState>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_towers: Query<&mut Health, (With<Tower>, Without<EnemyTower>)>,
    mut enemy_towers: Query<&mut Health, (With<Tower>, With<EnemyTower>)>,
    twitter_config: Res<TwitterConfig>,
) {
    // Cheat: D - Force defeat
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        println!("🔧 CHEAT: Force defeat activated!");
        
        // Destroy all player towers
        for mut health in player_towers.iter_mut() {
            health.current = 0.0;
        }
        
        victory_state.game_ended = true;
        victory_state.defeat_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
        
        // Show defeat screen
        show_defeat_screen_cheat(&mut commands, &asset_server);
    }

    // Cheat: V - Force victory
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        println!("🔧 CHEAT: Force victory activated!");
        
        // Destroy all enemy towers
        for mut health in enemy_towers.iter_mut() {
            health.current = 0.0;
        }
        
        victory_state.game_ended = true;
        victory_state.victory_timer = Some(Timer::from_seconds(3.0, TimerMode::Once));
        
        // Show victory screen
        show_victory_screen_cheat(&mut commands, &asset_server);
    }

    // Cheat: M - Add 1000 resources
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        println!("🔧 CHEAT: +1000 resources activated!");
        
        money.0 += 1000.0;
        wood.0 += 1000.0;
        iron.0 += 1000.0;
        steel.0 += 1000.0;
        oil.0 += 1000.0;
        
        println!("💰 Added 1000 to all resources!");
    }

    // Cheat: T - Add 10 seconds to player turn
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        println!("🔧 CHEAT: +10 seconds to turn activated!");
        
        turn_state.time_left += 10.0;
        
        println!("⏰ Added 10 seconds to current turn! Time left: {:.1}s", turn_state.time_left);
    }

    // Cheat: P - Test Twitter post
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        println!("🔧 CHEAT: Testing Twitter post!");
        
        if twitter_config.enabled {
            if let Some(ref client) = twitter_config.client {
                match client.test_twitter_integration_blocking() {
                    Ok(_) => println!("✅ Test tweet posted successfully!"),
                    Err(e) => eprintln!("❌ Failed to post test tweet: {}", e),
                }
            } else {
                println!("❌ Twitter client not configured");
            }
        } else {
            println!("❌ Twitter integration disabled");
        }
    }
}

fn show_victory_screen_cheat(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    use crate::systems::victory_system::{VictoryScreen, VictoryAudio};
    
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

fn show_defeat_screen_cheat(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    use crate::systems::victory_system::{DefeatScreen, DefeatAudio};
    
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
