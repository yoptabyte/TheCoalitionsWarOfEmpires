use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::menu::main_menu::Faction;
use super::{MilitaryUnit, PlayerFaction};
use crate::game::components::{CanShoot, Health, ShapeType, Selectable, HoveredOutline, Tank as TankMarker};

/// Enum to represent different tank types for the Entente faction
#[derive(Component, Clone, Copy, Debug)]
pub enum EntenteTankType {
    TsarTank,      // Higher health, slower movement
    Mark1,         // Balanced stats
    RenaultFT,     // Faster movement, lower health
}

/// Enum to represent different tank types for the Central Powers faction
#[derive(Component, Clone, Copy, Debug)]
pub enum CentralPowersTankType {
    AustroDaimlerPanzerwagen,  // Balanced stats
    A7V,                       // Higher health, slower movement
    OttomanTank,               // Faster movement, lower health (placeholder name)
}

/// Tank-specific attributes
#[derive(Component)]
pub struct TankAttributes {
    pub tank_type: TankType,
}

/// Enum to store the specific type of tank
#[derive(Clone, Copy, Debug)]
pub enum TankType {
    Entente(EntenteTankType),
    CentralPowers(CentralPowersTankType),
}

impl TankType {
    pub fn get_name(&self) -> String {
        match self {
            TankType::Entente(entente_type) => {
                match entente_type {
                    EntenteTankType::TsarTank => "Tsar Tank".to_string(),
                    EntenteTankType::Mark1 => "Mark I".to_string(),
                    EntenteTankType::RenaultFT => "Renault FT".to_string(),
                }
            },
            TankType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersTankType::AustroDaimlerPanzerwagen => "Austro-Daimler Panzerwagen".to_string(),
                    CentralPowersTankType::A7V => "A7V".to_string(),
                    CentralPowersTankType::OttomanTank => "Ottoman Tank".to_string(),
                }
            },
        }
    }

    pub fn get_stats(&self) -> MilitaryUnit {
        match self {
            // Entente tank stats
            TankType::Entente(entente_type) => {
                match entente_type {
                    EntenteTankType::TsarTank => MilitaryUnit {
                        speed: 1.0,
                        health: 300.0,
                        max_health: 300.0,
                        attack_damage: 40.0,
                        attack_speed: 0.5,
                        cost: 35,
                    },
                    EntenteTankType::Mark1 => MilitaryUnit {
                        speed: 1.5,
                        health: 250.0,
                        max_health: 250.0,
                        attack_damage: 35.0,
                        attack_speed: 0.6,
                        cost: 30,
                    },
                    EntenteTankType::RenaultFT => MilitaryUnit {
                        speed: 2.0,
                        health: 200.0,
                        max_health: 200.0,
                        attack_damage: 30.0,
                        attack_speed: 0.7,
                        cost: 25,
                    },
                }
            },
            // Central Powers tank stats
            TankType::CentralPowers(central_type) => {
                match central_type {
                    CentralPowersTankType::AustroDaimlerPanzerwagen => MilitaryUnit {
                        speed: 1.7,
                        health: 220.0,
                        max_health: 220.0,
                        attack_damage: 32.0,
                        attack_speed: 0.65,
                        cost: 28,
                    },
                    CentralPowersTankType::A7V => MilitaryUnit {
                        speed: 1.2,
                        health: 280.0,
                        max_health: 280.0,
                        attack_damage: 38.0,
                        attack_speed: 0.55,
                        cost: 32,
                    },
                    CentralPowersTankType::OttomanTank => MilitaryUnit {
                        speed: 1.9,
                        health: 190.0,
                        max_health: 190.0,
                        attack_damage: 28.0,
                        attack_speed: 0.75,
                        cost: 24,
                    },
                }
            },
        }
    }
}

/// System to spawn tank based on selected faction
pub fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_faction: &Res<PlayerFaction>,
    tank_type_index: usize,
    position: Vec3,
) -> Entity {
    let tank_type = match player_faction.0 {
        Faction::Entente => {
            match tank_type_index {
                0 => TankType::Entente(EntenteTankType::TsarTank),
                1 => TankType::Entente(EntenteTankType::Mark1),
                _ => TankType::Entente(EntenteTankType::RenaultFT),
            }
        },
        Faction::CentralPowers => {
            match tank_type_index {
                0 => TankType::CentralPowers(CentralPowersTankType::AustroDaimlerPanzerwagen),
                1 => TankType::CentralPowers(CentralPowersTankType::A7V),
                _ => TankType::CentralPowers(CentralPowersTankType::OttomanTank),
            }
        },
    };

    // Get model path based on tank type
    let model_path = match tank_type {
        TankType::Entente(entente_type) => {
            match entente_type {
                EntenteTankType::TsarTank => "models/entente/tanks/tsar_tank.glb#Scene0",
                EntenteTankType::Mark1 => "models/entente/tanks/mark1.glb#Scene0",
                EntenteTankType::RenaultFT => "models/entente/tanks/renault_ft17.glb#Scene0",
            }
        },
        TankType::CentralPowers(central_type) => {
            match central_type {
                CentralPowersTankType::AustroDaimlerPanzerwagen => "models/central_powers/tanks/panzerwagen.glb#Scene0",
                CentralPowersTankType::A7V => "models/central_powers/tanks/a7v.glb#Scene0",
                CentralPowersTankType::OttomanTank => "models/central_powers/tanks/steam_wheel_tank.glb#Scene0",
            }
        },
    };

    // Get stats for this tank type
    let stats = tank_type.get_stats();

    // Spawn the tank entity
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(model_path),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(0.4)),
            ..default()
        },
        TankMarker,
        TankAttributes {
            tank_type,
        },
        stats,
        Health {
            current: stats.health,
            max: stats.max_health,
        },
        CanShoot {
            cooldown: 1.0 / stats.attack_speed,
            last_shot: 0.0,
            range: 15.0,
            damage: stats.attack_damage,
        },
        ShapeType::Cube, // Using existing Cube shape type for tanks
        Selectable,
        HoveredOutline,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
        Collider::cuboid(0.8, 0.6, 1.2),
        PickableBundle::default(),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    )).id()
}

