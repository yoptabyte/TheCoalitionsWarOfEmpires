use bevy::prelude::*;
use crate::game_plugin::OnGameScreen;



// Resource to track notification states
#[derive(Resource, Default)]
pub struct NotificationState {
    pub purchase_menu_opened: bool,
    pub infantry_explained: bool,
    pub tanks_notified: bool,
    pub aircraft_notified: bool,
    pub farm_notified: bool,
    pub mine_notified: bool,
    pub steel_factory_notified: bool,
    pub petrochemical_plant_notified: bool,
}

// Component for blinking animations
#[derive(Component)]
pub struct BlinkingButton {
    pub timer: Timer,
    pub visible: bool,
}

impl Default for BlinkingButton {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            visible: true,
        }
    }
}

// Component for notification popups
#[derive(Component)]
pub struct NotificationPopup;

// Component for notification text
#[derive(Component)]
pub struct NotificationText;

// Component to mark infantry buttons as highlighted
#[derive(Component)]
pub struct HighlightedInfantryButton;

// Component for unit tooltips
#[derive(Component)]
pub struct UnitTooltip;


// Component for individual infantry unit buttons for hover detection
#[derive(Component)]
pub struct InfantryUnitButton {
    pub unit_type: usize, // 0, 1, 2 for the three infantry types
    pub faction: crate::menu::main_menu::Faction,
}

// Component for tank unit buttons
#[derive(Component)]
pub struct TankUnitButton {
    pub unit_type: usize, // 0, 1, 2 for the three tank types
    pub faction: crate::menu::main_menu::Faction,
}

// Component for aircraft unit buttons
#[derive(Component)]
pub struct AircraftUnitButton {
    pub unit_type: usize, // 0, 1, 2 for the three aircraft types
    pub faction: crate::menu::main_menu::Faction,
}

// Component for building buttons
#[derive(Component)]
pub struct BuildingButton {
    pub building_type: BuildingType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BuildingType {
    Mine,
    SteelFactory,
    PetrochemicalPlant,
}

// System to handle blinking animation
pub fn handle_blinking(
    mut query: Query<(&mut BlinkingButton, &mut Visibility)>,
    time: Res<Time>,
) {
    for (mut blinking, mut visibility) in query.iter_mut() {
        blinking.timer.tick(time.delta());
        
        if blinking.timer.just_finished() {
            blinking.visible = !blinking.visible;
            *visibility = if blinking.visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

// System to handle infantry button highlighting
pub fn handle_infantry_highlighting(
    mut query: Query<(&mut BackgroundColor, &HighlightedInfantryButton)>,
    time: Res<Time>,
) {
    for (mut bg_color, _) in query.iter_mut() {
        let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.3 + 0.7;
        *bg_color = Color::rgb(0.3 + pulse * 0.4, 0.3 + pulse * 0.2, 0.7 + pulse * 0.3).into();
    }
}

// System to spawn purchase menu notification
pub fn spawn_purchase_notification(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notification_state: Res<NotificationState>,
) {
    if !notification_state.purchase_menu_opened {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(65.0),
                    top: Val::Px(55.0),
                    width: Val::Px(200.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::rgba(0.2, 0.2, 0.3, 0.9).into(),
                ..default()
            },
            NotificationPopup,
            OnGameScreen,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Click to open purchase menu",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
                NotificationText,
            ));
        });
    }
}

// System to spawn infantry explanation notification
pub fn spawn_infantry_notification(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    notification_state: Res<NotificationState>,
) {
    if notification_state.purchase_menu_opened && !notification_state.infantry_explained {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(410.0),
                    top: Val::Px(200.0),
                    width: Val::Px(350.0),
                    height: Val::Px(120.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgba(0.2, 0.3, 0.2, 0.9).into(),
                ..default()
            },
            NotificationPopup,
            OnGameScreen,
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Infantry: Weakest and cheapest military units.\nGood for basic defense and scouting.\n\nTo purchase: Click on infantry button, then click\non the game field to spawn the unit.",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
                NotificationText,
            ));
        });
    }
}

// System to remove notifications when appropriate
pub fn cleanup_notifications(
    mut commands: Commands,
    notification_query: Query<Entity, With<NotificationPopup>>,
    notification_state: Res<NotificationState>,
) {
    for entity in &notification_query {
        // Remove purchase menu notification when menu is opened
        if notification_state.purchase_menu_opened {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// System to handle infantry interaction for marking as explained
pub fn handle_infantry_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<InfantryUnitButton>)>,
    mut notification_state: ResMut<NotificationState>,
    mut highlighted_query: Query<(Entity, &HighlightedInfantryButton)>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            if !notification_state.infantry_explained {
                notification_state.infantry_explained = true;
                
                // Remove highlighting from all infantry buttons
                for (entity, _) in highlighted_query.iter_mut() {
                    commands.entity(entity).remove::<HighlightedInfantryButton>();
                }
            }
        }
    }
}


// Component to track current tooltip
#[derive(Component)]
pub enum CurrentTooltip {
    Infantry { unit_type: usize, faction: crate::menu::main_menu::Faction },
    Tank { unit_type: usize, faction: crate::menu::main_menu::Faction },
    Aircraft { unit_type: usize, faction: crate::menu::main_menu::Faction },
    Building { building_type: BuildingType },
}

// System to manage tooltips on hover - runs every frame
pub fn manage_unit_tooltips(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    infantry_query: Query<(&Interaction, &InfantryUnitButton), With<Button>>,
    tank_query: Query<(&Interaction, &TankUnitButton), With<Button>>,
    aircraft_query: Query<(&Interaction, &AircraftUnitButton), With<Button>>,
    building_query: Query<(&Interaction, &BuildingButton), With<Button>>,
    existing_tooltips: Query<(Entity, &CurrentTooltip), With<UnitTooltip>>,
) {
    let mut current_hover: Option<CurrentTooltip> = None;
    
    // Check infantry buttons
    for (interaction, infantry_button) in &infantry_query {
        if *interaction == Interaction::Hovered {
            current_hover = Some(CurrentTooltip::Infantry {
                unit_type: infantry_button.unit_type,
                faction: infantry_button.faction,
            });
            break;
        }
    }
    
    // Check tank buttons if no infantry hovered
    if current_hover.is_none() {
        for (interaction, tank_button) in &tank_query {
            if *interaction == Interaction::Hovered {
                current_hover = Some(CurrentTooltip::Tank {
                    unit_type: tank_button.unit_type,
                    faction: tank_button.faction,
                });
                break;
            }
        }
    }
    
    // Check aircraft buttons if no tank hovered
    if current_hover.is_none() {
        for (interaction, aircraft_button) in &aircraft_query {
            if *interaction == Interaction::Hovered {
                current_hover = Some(CurrentTooltip::Aircraft {
                    unit_type: aircraft_button.unit_type,
                    faction: aircraft_button.faction,
                });
                break;
            }
        }
    }
    
    // Check building buttons if no aircraft hovered
    if current_hover.is_none() {
        for (interaction, building_button) in &building_query {
            if *interaction == Interaction::Hovered {
                current_hover = Some(CurrentTooltip::Building {
                    building_type: building_button.building_type,
                });
                break;
            }
        }
    }
    
    match current_hover {
        Some(hovered) => {
            // Check if we need to create/update tooltip
            let needs_new_tooltip = if let Ok((_, current_tooltip)) = existing_tooltips.get_single() {
                // Compare current tooltip with hovered tooltip
                !tooltips_match(current_tooltip, &hovered)
            } else {
                // No tooltip exists
                true
            };
            
            if needs_new_tooltip {
                // Remove existing tooltips
                for (entity, _) in &existing_tooltips {
                    commands.entity(entity).despawn_recursive();
                }
                
                // Get tooltip content based on type
                let (unit_name, stats) = match &hovered {
                    CurrentTooltip::Infantry { unit_type, faction } => get_infantry_stats(*faction, *unit_type),
                    CurrentTooltip::Tank { unit_type, faction } => get_tank_stats(*faction, *unit_type),
                    CurrentTooltip::Aircraft { unit_type, faction } => get_aircraft_stats(*faction, *unit_type),
                    CurrentTooltip::Building { building_type } => get_building_stats(*building_type),
                };
                
                // Create new tooltip with appropriate width
                let tooltip_width = match &hovered {
                    CurrentTooltip::Infantry { .. } => 250.0,
                    CurrentTooltip::Tank { .. } => 350.0,
                    CurrentTooltip::Aircraft { .. } => 350.0,
                    CurrentTooltip::Building { .. } => 320.0,
                };
                
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(420.0),
                            top: Val::Px(300.0),
                            width: Val::Px(tooltip_width),
                            padding: UiRect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::rgba(0.1, 0.1, 0.2, 0.95).into(),
                        z_index: ZIndex::Global(1000),
                        ..default()
                    },
                    UnitTooltip,
                    hovered,
                    OnGameScreen,
                )).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("{}\n{}", unit_name, stats),
                        TextStyle {
                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            }
        }
        None => {
            // No button hovered, remove all tooltips
            for (entity, _) in &existing_tooltips {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Helper function to compare tooltips
fn tooltips_match(current: &CurrentTooltip, new: &CurrentTooltip) -> bool {
    match (current, new) {
        (
            CurrentTooltip::Infantry { unit_type: u1, faction: f1 },
            CurrentTooltip::Infantry { unit_type: u2, faction: f2 }
        ) => u1 == u2 && f1 == f2,
        (
            CurrentTooltip::Tank { unit_type: u1, faction: f1 },
            CurrentTooltip::Tank { unit_type: u2, faction: f2 }
        ) => u1 == u2 && f1 == f2,
        (
            CurrentTooltip::Aircraft { unit_type: u1, faction: f1 },
            CurrentTooltip::Aircraft { unit_type: u2, faction: f2 }
        ) => u1 == u2 && f1 == f2,
        (
            CurrentTooltip::Building { building_type: b1 },
            CurrentTooltip::Building { building_type: b2 }
        ) => b1 == b2,
        _ => false,
    }
}

// Helper function to get infantry unit cost
pub fn get_infantry_cost(faction: crate::menu::main_menu::Faction, unit_type: usize) -> f32 {
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => 15.0, // Russian Infantry
                1 => 18.0, // British Infantry
                2 => 16.0, // French Infantry
                _ => 15.0,
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => 20.0, // German Infantry
                1 => 14.0, // Turkish Infantry
                2 => 17.0, // Austro-Hungarian Infantry
                _ => 15.0,
            }
        },
    }
}

// Helper function to get tank unit costs and requirements
pub fn get_tank_cost(faction: crate::menu::main_menu::Faction, unit_type: usize) -> (f32, f32, f32, f32, f32) {
    // Returns (money, wood, iron, steel, oil)
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => (50.0, 5.0, 8.0, 6.0, 12.0), // Tsar Tank
                1 => (45.0, 4.0, 7.0, 5.0, 10.0), // Mark I
                2 => (40.0, 3.0, 6.0, 4.0, 8.0),  // Renault FT
                _ => (45.0, 4.0, 7.0, 5.0, 10.0),
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => (48.0, 5.0, 7.0, 6.0, 11.0), // Austro-Daimler
                1 => (55.0, 6.0, 9.0, 7.0, 13.0), // A7V
                2 => (42.0, 4.0, 6.0, 5.0, 9.0),  // Ottoman Tank
                _ => (48.0, 5.0, 7.0, 6.0, 11.0),
            }
        },
    }
}

// Helper function to get aircraft unit costs and requirements
pub fn get_aircraft_cost(faction: crate::menu::main_menu::Faction, unit_type: usize) -> (f32, f32, f32, f32, f32) {
    // Returns (money, wood, iron, steel, oil)
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => (35.0, 8.0, 3.0, 4.0, 15.0), // Sopwith Camel
                1 => (40.0, 9.0, 4.0, 5.0, 18.0), // SPAD S.XIII
                2 => (60.0, 12.0, 6.0, 8.0, 25.0), // Sikorsky
                _ => (35.0, 8.0, 3.0, 4.0, 15.0),
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => (38.0, 8.0, 4.0, 5.0, 16.0), // Fokker Dr.I
                1 => (42.0, 9.0, 4.0, 6.0, 19.0), // Albatros D.III
                2 => (55.0, 11.0, 5.0, 7.0, 22.0), // Gotha G.V
                _ => (38.0, 8.0, 4.0, 5.0, 16.0),
            }
        },
    }
}

// Helper function to get building costs
pub fn get_building_cost(building_type: BuildingType) -> (f32, f32, f32, f32, f32) {
    // Returns (money, wood, iron, steel, oil)
    match building_type {
        BuildingType::Mine => (30.0, 8.0, 0.0, 0.0, 0.0),
        BuildingType::SteelFactory => (40.0, 12.0, 15.0, 0.0, 0.0),
        BuildingType::PetrochemicalPlant => (50.0, 15.0, 10.0, 8.0, 0.0),
    }
}

// Helper function to get infantry unit stats
fn get_infantry_stats(faction: crate::menu::main_menu::Faction, unit_type: usize) -> (String, String) {
    let cost = get_infantry_cost(faction, unit_type);
    
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => ("Russian Infantry".to_string(), format!("Cost: ${}\nHP: 100\nAttack: 25\nSpeed: Medium", cost)),
                1 => ("British Infantry".to_string(), format!("Cost: ${}\nHP: 110\nAttack: 28\nSpeed: Medium", cost)),
                2 => ("French Infantry".to_string(), format!("Cost: ${}\nHP: 105\nAttack: 26\nSpeed: Medium", cost)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => ("German Infantry".to_string(), format!("Cost: ${}\nHP: 115\nAttack: 30\nSpeed: Medium", cost)),
                1 => ("Turkish Infantry".to_string(), format!("Cost: ${}\nHP: 95\nAttack: 24\nSpeed: Fast", cost)),
                2 => ("Austro-Hungarian Infantry".to_string(), format!("Cost: ${}\nHP: 108\nAttack: 27\nSpeed: Medium", cost)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
    }
}

// Helper function to get tank unit stats
fn get_tank_stats(faction: crate::menu::main_menu::Faction, unit_type: usize) -> (String, String) {
    let (money, wood, iron, steel, oil) = get_tank_cost(faction, unit_type);
    
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => ("Tsar Tank".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 300\nAttack: 80\nSpeed: Slow\nHeavy armor, devastating firepower", money, wood, iron, steel, oil)),
                1 => ("Mark I".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 250\nAttack: 70\nSpeed: Slow\nFirst battle tank, reliable", money, wood, iron, steel, oil)),
                2 => ("Renault FT".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 200\nAttack: 60\nSpeed: Medium\nLight and maneuverable", money, wood, iron, steel, oil)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => ("Austro-Daimler".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 280\nAttack: 75\nSpeed: Slow\nAustrian engineering", money, wood, iron, steel, oil)),
                1 => ("A7V".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 320\nAttack: 85\nSpeed: Very Slow\nGerman super-heavy tank", money, wood, iron, steel, oil)),
                2 => ("Ottoman Tank".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 220\nAttack: 65\nSpeed: Medium\nAdapted for desert warfare", money, wood, iron, steel, oil)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
    }
}

// Helper function to get aircraft unit stats
fn get_aircraft_stats(faction: crate::menu::main_menu::Faction, unit_type: usize) -> (String, String) {
    let (money, wood, iron, steel, oil) = get_aircraft_cost(faction, unit_type);
    
    match faction {
        crate::menu::main_menu::Faction::Entente => {
            match unit_type {
                0 => ("Sopwith Camel".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 80\nAttack: 45\nSpeed: Fast\nAgile fighter aircraft", money, wood, iron, steel, oil)),
                1 => ("SPAD S.XIII".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 90\nAttack: 50\nSpeed: Fast\nFrench ace's choice", money, wood, iron, steel, oil)),
                2 => ("Sikorsky".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 150\nAttack: 70\nSpeed: Medium\nHeavy bomber aircraft", money, wood, iron, steel, oil)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
        crate::menu::main_menu::Faction::CentralPowers => {
            match unit_type {
                0 => ("Fokker Dr.I".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 85\nAttack: 48\nSpeed: Fast\nTriplane fighter", money, wood, iron, steel, oil)),
                1 => ("Albatros D.III".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 95\nAttack: 52\nSpeed: Fast\nGerman air superiority", money, wood, iron, steel, oil)),
                2 => ("Gotha G.V".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{} ⛽{}\nHP: 140\nAttack: 65\nSpeed: Medium\nStrategic bomber", money, wood, iron, steel, oil)),
                _ => ("Unknown".to_string(), "".to_string()),
            }
        },
    }
}

// Helper function to get building stats
fn get_building_stats(building_type: BuildingType) -> (String, String) {
    let (money, wood, iron, steel, _oil) = get_building_cost(building_type);
    
    match building_type {
        BuildingType::Mine => ("Mine".to_string(), format!("Cost: ${} 🪵{}\nProduces: Iron\nRate: +1 iron/sec\nLimited: One per player\nCan be rebuilt if destroyed", money, wood)),
        BuildingType::SteelFactory => ("Steel Factory".to_string(), format!("Cost: ${} 🪵{} ⛏️{}\nProduces: Steel\nRate: +1 steel/sec\nLimited: One per player\nCan be rebuilt if destroyed", money, wood, iron)),
        BuildingType::PetrochemicalPlant => ("Petrochemical Plant".to_string(), format!("Cost: ${} 🪵{} ⛏️{} 🔩{}\nProduces: Oil\nRate: +1 oil/sec\nLimited: One per player\nCan be rebuilt if destroyed", money, wood, iron, steel)),
    }
}

pub struct NotificationSystemPlugin;

impl Plugin for NotificationSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NotificationState>()
            .add_systems(Update, (
                handle_blinking,
                handle_infantry_highlighting,
                spawn_purchase_notification,
                spawn_infantry_notification,
                cleanup_notifications,
                handle_infantry_interaction,
                manage_unit_tooltips,
            ).run_if(in_state(crate::menu::common::GameState::Game)));
    }
}