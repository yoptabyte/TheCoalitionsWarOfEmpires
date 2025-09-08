use bevy::prelude::*;
use crate::systems::turn_system::{TurnState, PlayerTurn};
use crate::menu::common::GameState;
use crate::game_plugin::OnGameScreen;

#[derive(Component)]
pub struct CurrentPlayerText;

#[derive(Component)]
pub struct TurnTimerText;

#[derive(Component)]
pub struct TurnNumberText;

#[derive(Component)]
pub struct AITurnVeil;

pub struct TurnUiPlugin;

impl Plugin for TurnUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TurnState>()
            .add_systems(OnEnter(GameState::Game), setup_turn_ui)
            .add_systems(
                Update,
                (update_turn_ui, manage_ai_veil).run_if(in_state(GameState::Game)),
            );
    }
}

fn setup_turn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Контейнер для turn-based UI (центр экрана, верхняя часть)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    position_type: PositionType::Absolute,
                    top: Val::Px(80.0),
                    left: Val::Px(0.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center, // Центрируем содержимое
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(), // Убираем фон контейнера
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // Внутренний контейнер с фоном
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0), // Расстояние между элементами
                    padding: UiRect::all(Val::Px(15.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            })
            .with_children(|inner_parent| {
            // Номер хода
            inner_parent.spawn((
                TextBundle::from_section(
                    "Turn: 1",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 32.0, // Увеличен размер
                        color: Color::WHITE,
                    },
                ),
                TurnNumberText,
            ));

            // Разделитель
            inner_parent.spawn(TextBundle::from_section(
                "|",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 28.0,
                    color: Color::rgb(0.5, 0.5, 0.5),
                },
            ));

            // Текущий игрок
            inner_parent.spawn((
                TextBundle::from_section(
                    "Player Turn",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 28.0, // Увеличен размер
                        color: Color::rgb(0.2, 1.0, 0.2),
                    },
                ),
                CurrentPlayerText,
            ));

            // Разделитель
            inner_parent.spawn(TextBundle::from_section(
                "|",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 28.0,
                    color: Color::rgb(0.5, 0.5, 0.5),
                },
            ));

            // Таймер хода
            inner_parent.spawn((
                TextBundle::from_section(
                    "Time: 30s",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 26.0, // Увеличен размер
                        color: Color::rgb(1.0, 0.8, 0.2),
                    },
                ),
                TurnTimerText,
            ));
            }); // Закрываем inner_parent
        });
}

fn update_turn_ui(
    turn_state: Res<TurnState>,
    mut turn_number_query: Query<&mut Text, (With<TurnNumberText>, Without<CurrentPlayerText>, Without<TurnTimerText>)>,
    mut current_player_query: Query<&mut Text, (With<CurrentPlayerText>, Without<TurnNumberText>, Without<TurnTimerText>)>,
    mut turn_timer_query: Query<&mut Text, (With<TurnTimerText>, Without<TurnNumberText>, Without<CurrentPlayerText>)>,
    player_faction: Res<crate::game::units::PlayerFaction>,
) {
    // Обновляем номер хода
    if let Ok(mut text) = turn_number_query.get_single_mut() {
        text.sections[0].value = format!("Turn: {}", turn_state.turn_number);
    }

    // Обновляем текущего игрока
    if let Ok(mut text) = current_player_query.get_single_mut() {
        match turn_state.current_player {
            PlayerTurn::Human => {
                text.sections[0].value = "Player Turn".to_string();
                text.sections[0].style.color = Color::rgb(0.2, 1.0, 0.2);
            }
            PlayerTurn::AI => {
                // Показываем название противоположной фракции
                let enemy_faction_name = match player_faction.0 {
                    crate::menu::main_menu::Faction::Entente => "Central Powers Turn",
                    crate::menu::main_menu::Faction::CentralPowers => "Entente Turn",
                };
                text.sections[0].value = enemy_faction_name.to_string();
                text.sections[0].style.color = Color::rgb(1.0, 0.3, 0.3);
            }
        }
    }

    // Обновляем таймер
    if let Ok(mut text) = turn_timer_query.get_single_mut() {
        let seconds = turn_state.time_left.ceil() as i32;
        text.sections[0].value = format!("Time: {}s", seconds);
        
        // Меняем цвет когда время заканчивается
        if turn_state.time_left <= 5.0 {
            text.sections[0].style.color = Color::rgb(1.0, 0.2, 0.2);
        } else {
            text.sections[0].style.color = Color::rgb(1.0, 0.8, 0.2);
        }
    }
}

fn manage_ai_veil(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    existing_veil: Query<Entity, With<AITurnVeil>>,
    asset_server: Res<AssetServer>,
    player_faction: Res<crate::game::units::PlayerFaction>,
) {
    match turn_state.current_player {
        PlayerTurn::AI => {
            // Создаем вуаль если её еще нет
            if existing_veil.is_empty() {
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                        z_index: ZIndex::Global(999), // Максимальный z-index чтобы быть поверх всего
                        ..default()
                    },
                    AITurnVeil,
                    OnGameScreen,
                ))
                .with_children(|parent| {
                    // Большой текст с названием фракции противника
                    let enemy_faction_name = match player_faction.0 {
                        crate::menu::main_menu::Faction::Entente => "CENTRAL POWERS TURN",
                        crate::menu::main_menu::Faction::CentralPowers => "ENTENTE TURN",
                    };
                    parent.spawn(TextBundle::from_section(
                        enemy_faction_name,
                        TextStyle {
                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                            font_size: 72.0,
                            color: Color::rgb(1.0, 0.3, 0.3),
                        },
                    ));
                    
                    // Подпись
                    parent.spawn(TextBundle::from_section(
                        "Enemy is moving and attacking...",
                        TextStyle {
                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                            font_size: 24.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    ));
                });
            }
        }
        PlayerTurn::Human => {
            // Убираем вуаль если она есть
            for entity in existing_veil.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}