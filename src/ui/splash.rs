use bevy::prelude::*;

use crate::menu::common::{despawn_screen, GameState};

pub fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    commands.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        OnSplashScreen,
    ))
    .with_children(|parent| {
        parent.spawn(
            ImageBundle {
                image: UiImage::new(icon),
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                ..default()
            }
        );
    });
    
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
} 