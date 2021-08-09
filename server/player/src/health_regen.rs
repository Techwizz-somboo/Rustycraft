//! Handles Health regen for player

use feather_server_types::{Dead, Game, Health, Player};
use fecs::World;

pub fn update_player_health(game: &mut Game, world: &mut World, player: &mut Player) {
    //How fast the player gets his/her health back
    let regen_rate = 1;
}
