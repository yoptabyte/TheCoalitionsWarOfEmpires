use bevy::prelude::*;
use crate::menu::main_menu::Faction;
use super::{MilitaryUnit, PlayerFaction};
use crate::game::components::{CanShoot, Health, ShapeType, Selectable, HoveredOutline, Aircraft as AircraftMarker};

/// Enum to represent different aircraft types for the Entente faction
#[derive(Component, Clone, Copy, Debug)]
pub enum EntenteAircraftType {
    Sopwith,       // British aircraft - faster, lower damage
    Spad,          // French aircraft - balanced stats
    Sikorsky,      // Russian aircraft - slower, higher damage
}

/// Enum to represent different aircraft types for the Central Powers faction
#[derive(Component, Clone, Copy, Debug)]
pub enum CentralPowersAircraftType {
    Fokker,        // German aircraft - higher damage, balanced speed
    Albatros,      // German/Austro-Hungarian aircraft - balanced stats
    Gotha,         // German bomber - slower, highest damage
}

/// Aircraft-specific attributes
#[derive(Component)]
pub struct AircraftAttributes {
    pub aircraft_type: AircraftType,
    pub altitude: f32,
}

/// Enum to store the specific type of aircraft
#[derive(Clone, Copy, Debug)]
pub enum AircraftType {
    Entente(EntenteAircraftType),
    CentralPowers(CentralPowersAircraftType),
}

impl AircraftType {
    pub fn get_name(&self) -> String {
        match self {
            AircraftType::Entente(entente_type) => {
                match entente_type {
                    EntenteAircraftType::Sopwith => "Sopwith Camel".to_string(),
                    EntenteAircraftType::Spad => "SPAD S.XIII".to_string(),
                    EntenteAircraftType::Sikorsky => "Sikorsky Ilya Muromets".to_string(),
                }
            },
            AircraftType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersAircraftType::Fokker => "Fokker Dr.I".to_string(),
                    CentralPowersAircraftType::Albatros => "Albatros D.III".to_string(),
                    CentralPowersAircraftType::Gotha => "Gotha G.V".to_string(),
                }
            },
        }
    }

    pub fn get_stats(&self) -> MilitaryUnit {
        match self {
            // Entente aircraft stats
            AircraftType::Entente(entente_type) => {
                match entente_type {
                    EntenteAircraftType::Sopwith => MilitaryUnit {
                        speed: 3.5,
                        health: 80.0,
                        max_health: 80.0,
                        attack_damage: 20.0,
                        attack_speed: 1.2,
                        cost: 20,
                    },
                    EntenteAircraftType::Spad => MilitaryUnit {
                        speed: 3.0,
                        health: 90.0,
                        max_health: 90.0,
                        attack_damage: 25.0,
                        attack_speed: 1.0,
                        cost: 22,
                    },
                    EntenteAircraftType::Sikorsky => MilitaryUnit {
                        speed: 2.0,
                        health: 120.0,
                        max_health: 120.0,
                        attack_damage: 35.0,
                        attack_speed: 0.7,
                        cost: 30,
                    },
                }
            },
            // Central Powers aircraft stats
            AircraftType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersAircraftType::Fokker => MilitaryUnit {
                        speed: 3.2,
                        health: 85.0,
                        max_health: 85.0,
                        attack_damage: 28.0,
                        attack_speed: 1.1,
                        cost: 24,
                    },
                    CentralPowersAircraftType::Albatros => MilitaryUnit {
                        speed: 3.0,
                        health: 90.0,
                        max_health: 90.0,
                        attack_damage: 24.0,
                        attack_speed: 1.0,
                        cost: 22,
                    },
                    CentralPowersAircraftType::Gotha => MilitaryUnit {
                        speed: 2.2,
                        health: 110.0,
                        max_health: 110.0,
                        attack_damage: 40.0,
                        attack_speed: 0.6,
                        cost: 32,
                    },
                }
            },
        }
    }
}

/// System to spawn aircraft based on selected faction
pub fn spawn_aircraft(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_faction: &Res<PlayerFaction>,
    aircraft_type_index: usize,
    position: Vec3,
) -> Entity {
    let aircraft_type = match player_faction.0 {
        Faction::Entente => {
            match aircraft_type_index {
                0 => AircraftType::Entente(EntenteAircraftType::Sopwith),
                1 => AircraftType::Entente(EntenteAircraftType::Spad),
                _ => AircraftType::Entente(EntenteAircraftType::Sikorsky),
            }
        },
        Faction::CentralPowers => {
            match aircraft_type_index {
                0 => AircraftType::CentralPowers(CentralPowersAircraftType::Fokker),
                1 => AircraftType::CentralPowers(CentralPowersAircraftType::Albatros),
                _ => AircraftType::CentralPowers(CentralPowersAircraftType::Gotha),
            }
        },
    };

    // Get model path based on aircraft type
    let model_path = match aircraft_type {
        AircraftType::Entente(entente_type) => {
            match entente_type {
                EntenteAircraftType::Sopwith => "models/aircraft/sopwith_camel.glb#Scene0",
                EntenteAircraftType::Spad => "models/aircraft/spad_xiii.glb#Scene0",
                EntenteAircraftType::Sikorsky => "models/aircraft/sikorsky_ilya_muromets.glb#Scene0",
            }
        },
        AircraftType::CentralPowers(central_type) => {
            match central_type {
                CentralPowersAircraftType::Fokker => "models/aircraft/fokker_dr1.glb#Scene0",
                CentralPowersAircraftType::Albatros => "models/aircraft/albatros_d3.glb#Scene0",
                CentralPowersAircraftType::Gotha => "models/aircraft/gotha_g5.glb#Scene0",
            }
        },
    };

    // Get stats for this aircraft type
    let stats = aircraft_type.get_stats();
    
    // Set altitude based on aircraft type
    let altitude = match aircraft_type {
        AircraftType::Entente(EntenteAircraftType::Sikorsky) => 15.0,
        AircraftType::CentralPowers(CentralPowersAircraftType::Gotha) => 15.0,
        _ => 12.0,
    };

    // Spawn the aircraft entity
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(model_path),
            transform: Transform::from_translation(position + Vec3::new(0.0, altitude, 0.0))
                .with_scale(Vec3::splat(0.6)),
            ..default()
        },
        AircraftMarker {
            height: altitude,
            speed: stats.speed,
        },
        AircraftAttributes {
            aircraft_type,
            altitude,
        },
        stats,
        Health {
            current: stats.health,
            max: stats.max_health,
        },
        CanShoot {
            cooldown: 1.0 / stats.attack_speed,
            last_shot: 0.0,
            range: 20.0,
            damage: stats.attack_damage,
        },
        ShapeType::Airplane,
        Selectable,
        HoveredOutline,
    )).id()
}

