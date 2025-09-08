use bevy::prelude::*;
use crate::menu::common::{GameState, MenuState};

#[derive(Component)]
pub struct ConfirmDialog;

#[derive(Component)]
pub struct ConfirmDialogButton;

#[derive(Component)]
pub enum ConfirmDialogAction {
    Yes,
    No,
}

pub fn spawn_confirm_dialog(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                z_index: ZIndex::Global(1001), // Выше всего, включая AI turn screen и pause menu
                ..default()
            },
            ConfirmDialog,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Are you sure?",
                        TextStyle {
                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                margin: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Yes button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                        ..default()
                                    },
                                    ConfirmDialogButton,
                                    ConfirmDialogAction::Yes,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Yes",
                                        TextStyle {
                                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });

                            // No button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style,
                                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                        ..default()
                                    },
                                    ConfirmDialogButton,
                                    ConfirmDialogAction::No,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "No",
                                        TextStyle {
                                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        });
                });
        });
}

pub fn confirm_dialog_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ConfirmDialogAction),
        (Changed<Interaction>, With<ConfirmDialogButton>),
    >,
) {
    for (interaction, mut background_color, _) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => Color::rgb(0.35, 0.75, 0.35).into(),
            Interaction::Hovered => Color::rgb(0.25, 0.25, 0.25).into(),
            Interaction::None => Color::rgb(0.15, 0.15, 0.15).into(),
        }
    }
}

pub fn handle_confirm_dialog_actions(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ConfirmDialogAction), (Changed<Interaction>, With<ConfirmDialogButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    dialog_query: Query<Entity, With<ConfirmDialog>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ConfirmDialogAction::Yes => {
                    println!("DEBUG: Confirm dialog YES pressed - transitioning to main menu");
                    // First despawn the dialog
                    for entity in dialog_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    // Return to main menu - только GameState, MenuState установится автоматически
                    println!("DEBUG: Setting GameState::Menu (MenuState will be set automatically)");
                    game_state.set(GameState::Menu);
                    // НЕ устанавливаем menu_state.set(MenuState::Main) здесь!
                }
                ConfirmDialogAction::No => {
                    // Just close the dialog
                    for entity in dialog_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
} 