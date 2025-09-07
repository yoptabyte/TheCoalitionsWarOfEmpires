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

/// Resource to store the AI's faction (always opposite to player)
#[derive(Resource, Clone, Copy)]
pub struct AIFaction(pub Faction);

impl Default for AIFaction {
    fn default() -> Self {
        Self(Faction::CentralPowers) // Default opposite to player
    }
}

impl AIFaction {
    pub fn set_opposite_to_player(&mut self, player_faction: Faction) {
        self.0 = match player_faction {
            Faction::Entente => Faction::CentralPowers,
            Faction::CentralPowers => Faction::Entente,
        };
    }
    
    pub fn get_opposite(&self) -> Faction {
        match self.0 {
            Faction::Entente => Faction::CentralPowers,
            Faction::CentralPowers => Faction::Entente,
        }
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
        app.init_resource::<PlayerFaction>()
           .init_resource::<AIFaction>()
           .add_systems(Update, update_ai_faction_on_player_change);
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

/// System to automatically set AI faction to opposite of player faction
pub fn update_ai_faction_on_player_change(
    player_faction: Res<PlayerFaction>,
    mut ai_faction: ResMut<AIFaction>,
) {
    if player_faction.is_changed() {
        ai_faction.set_opposite_to_player(player_faction.0);
        info!("Player chose {:?}, AI will be {:?}", player_faction.0, ai_faction.0);
    }
}
