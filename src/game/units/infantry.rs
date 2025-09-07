use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::menu::main_menu::Faction;
use super::{MilitaryUnit, PlayerFaction};
use crate::game::components::{CanShoot, Health, ShapeType, Selectable, HoveredOutline};

/// Enum to represent different infantry types for the Entente faction
#[derive(Component, Clone, Copy, Debug)]
pub enum EntenteInfantryType {
    Russian,   // Higher health, slower movement
    British,   // Balanced stats
    French,    // Faster movement, lower health
}

/// Enum to represent different infantry types for the Central Powers faction
#[derive(Component, Clone, Copy, Debug)]
pub enum CentralPowersInfantryType {
    German,        // Higher damage, balanced speed
    Turkish,       // Faster movement, lower health
    AustroHungarian, // Higher health, slower movement
}

/// Component to mark an entity as infantry
#[derive(Component)]
pub struct Infantry;

/// Infantry-specific attributes
#[derive(Component)]
pub struct InfantryAttributes {
    pub infantry_type: InfantryType,
}

/// Enum to store the specific type of infantry
#[derive(Clone, Copy, Debug)]
pub enum InfantryType {
    Entente(EntenteInfantryType),
    CentralPowers(CentralPowersInfantryType),
}

impl InfantryType {
    pub fn get_name(&self) -> String {
        match self {
            InfantryType::Entente(entente_type) => {
                match entente_type {
                    EntenteInfantryType::Russian => "Russian Infantry".to_string(),
                    EntenteInfantryType::British => "British Infantry".to_string(),
                    EntenteInfantryType::French => "French Infantry".to_string(),
                }
            },
            InfantryType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersInfantryType::German => "German Infantry".to_string(),
                    CentralPowersInfantryType::Turkish => "Turkish Infantry".to_string(),
                    CentralPowersInfantryType::AustroHungarian => "Austro-Hungarian Infantry".to_string(),
                }
            },
        }
    }

    pub fn get_stats(&self) -> MilitaryUnit {
        match self {
            // Entente infantry stats
            InfantryType::Entente(entente_type) => {
                match entente_type {
                    EntenteInfantryType::Russian => MilitaryUnit {
                        speed: 1.5,
                        health: 120.0,
                        max_health: 120.0,
                        attack_damage: 15.0,
                        attack_speed: 0.8,
                        cost: 10,
                    },
                    EntenteInfantryType::British => MilitaryUnit {
                        speed: 2.0,
                        health: 100.0,
                        max_health: 100.0,
                        attack_damage: 12.0,
                        attack_speed: 1.0,
                        cost: 12,
                    },
                    EntenteInfantryType::French => MilitaryUnit {
                        speed: 2.5,
                        health: 80.0,
                        max_health: 80.0,
                        attack_damage: 10.0,
                        attack_speed: 1.2,
                        cost: 8,
                    },
                }
            },
            // Central Powers infantry stats
            InfantryType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersInfantryType::German => MilitaryUnit {
                        speed: 2.0,
                        health: 100.0,
                        max_health: 100.0,
                        attack_damage: 18.0,
                        attack_speed: 0.9,
                        cost: 12,
                    },
                    CentralPowersInfantryType::Turkish => MilitaryUnit {
                        speed: 2.8,
                        health: 70.0,
                        max_health: 70.0,
                        attack_damage: 12.0,
                        attack_speed: 1.3,
                        cost: 8,
                    },
                    CentralPowersInfantryType::AustroHungarian => MilitaryUnit {
                        speed: 1.7,
                        health: 110.0,
                        max_health: 110.0,
                        attack_damage: 14.0,
                        attack_speed: 0.8,
                        cost: 10,
                    },
                }
            },
        }
    }
}

/// System to spawn infantry based on selected faction
pub fn spawn_infantry(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_faction: &Res<PlayerFaction>,
    infantry_type_index: usize,
    position: Vec3,
) -> Entity {
    let infantry_type = match player_faction.0 {
        Faction::Entente => {
            match infantry_type_index {
                0 => InfantryType::Entente(EntenteInfantryType::Russian),
                1 => InfantryType::Entente(EntenteInfantryType::British),
                _ => InfantryType::Entente(EntenteInfantryType::French),
            }
        },
        Faction::CentralPowers => {
            match infantry_type_index {
                0 => InfantryType::CentralPowers(CentralPowersInfantryType::German),
                1 => InfantryType::CentralPowers(CentralPowersInfantryType::Turkish),
                _ => InfantryType::CentralPowers(CentralPowersInfantryType::AustroHungarian),
            }
        },
    };

    // Get model path based on infantry type
    let model_path = match infantry_type {
        InfantryType::Entente(entente_type) => {
            match entente_type {
                EntenteInfantryType::Russian => "models/infantry/russian_infantry.glb#Scene0",
                EntenteInfantryType::British => "models/infantry/british_infantry.glb#Scene0",
                EntenteInfantryType::French => "models/infantry/french_infantry.glb#Scene0",
            }
        },
        InfantryType::CentralPowers(central_type) => {
            match central_type {
                CentralPowersInfantryType::German => "models/infantry/german_infantry.glb#Scene0",
                CentralPowersInfantryType::Turkish => "models/infantry/turkish_infantry.glb#Scene0",
                CentralPowersInfantryType::AustroHungarian => "models/infantry/austrohungarian_infantry.glb#Scene0",
            }
        },
    };

    // Get stats for this infantry type
    let stats = infantry_type.get_stats();

    // Spawn the infantry entity
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(model_path),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(0.5)),
            ..default()
        },
        Infantry,
        InfantryAttributes {
            infantry_type,
        },
        stats,
        Health {
            current: stats.health,
            max: stats.max_health,
        },
        CanShoot {
            cooldown: 1.0 / stats.attack_speed,
            last_shot: 0.0,
            range: 10.0,
            damage: stats.attack_damage,
        },
        ShapeType::Infantry,
        Selectable,
        HoveredOutline,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
        Collider::ball(0.5),
        PickableBundle::default(),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    )).id()
}

