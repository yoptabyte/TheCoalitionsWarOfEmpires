use bevy::{
    prelude::*,
};

use super::common::*;

const CRIMSON: Color = Color::rgb(0.86, 0.08, 0.24);

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

pub fn settings_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnSettingsMenuScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: 80.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::SettingsDisplay,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Display",
                    button_text_style.clone(),
                ));
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::SettingsSound,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Sound",
                    button_text_style.clone(),
                ));
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::BackToMainMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Back",
                    button_text_style,
                ));
            });
        });
    });
}

pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..default()
    };

    let display_quality = *display_quality;
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnDisplaySettingsMenuScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Display Quality",
                    button_text_style.clone(),
                ));

                for quality_setting in [
                    DisplayQuality::Low,
                    DisplayQuality::Medium,
                    DisplayQuality::High,
                ] {
                    let mut entity_commands = parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                ..button_style.clone()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        quality_setting,
                    ));

                    entity_commands.with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{quality_setting:?}"),
                            button_text_style.clone(),
                        ));
                    });

                    if display_quality == quality_setting {
                        entity_commands.insert(SelectedOption);
                    }
                }
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::BackToSettings,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Back",
                    button_text_style.clone(),
                ));
            });
        });
    });
}

pub fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    
    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..default()
    };

    let volume = *volume;
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnSoundSettingsMenuScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Volume",
                    button_text_style.clone(),
                ));

                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                    let mut entity_commands = parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(65.0),
                                ..button_style.clone()
                            },
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        Volume(volume_setting),
                    ));

                    if volume == Volume(volume_setting) {
                        entity_commands.insert(SelectedOption);
                    }
                }
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::BackToSettings,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Back",
                    button_text_style,
                ));
            });
        });
    });
}

pub fn settings_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(OnEnter(MenuState::SettingsDisplay), display_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay))
        )
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound))
        );
} 