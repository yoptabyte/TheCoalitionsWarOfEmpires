use bevy::prelude::*;
use crate::menu::main_menu::Faction;

pub mod infantry;
pub mod tanks;
pub mod aircraft;

/// Resource to store the player's selected faction
#[derive(Resource, Clone, Copy)]
pub struct PlayerFaction(pub Faction);

impl Default for PlayerFaction {
    fn default() -> Self {
        Self(Faction::Entente)
    }
}

/// Common traits for all military units
#[derive(Component, Clone, Copy)]
pub struct MilitaryUnit {
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,  // attacks per second
    pub cost: u32,
}

/// Plugin to register all unit-related systems and resources
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerFaction>()
            .add_systems(Update, (
                infantry::update_infantry_system,
                tanks::update_tanks_system,
                aircraft::update_aircraft_system,
            ));
    }
}

/// System to save the selected faction when player makes a choice
pub fn save_faction_selection(
    mut player_faction: ResMut<PlayerFaction>,
    interaction_query: Query<(&Interaction, &Faction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, faction) in &interaction_query {
        if *interaction == Interaction::Pressed {
            player_faction.0 = *faction;
        }
    }
}
