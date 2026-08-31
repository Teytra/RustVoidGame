
pub struct Enemy {
    pub health: u16,
    pub defense: u16,
    pub damage: u16
    
}
impl Enemy {
    pub fn new(player_lvl: u16) -> Self {
        let start = (player_lvl as i16 - 2).max(1);
        let level = fastrand::u16(start as u16..=player_lvl +2);
        Self { health: level * 100, defense: level * 1, damage: level * 50  }
    }
    pub fn get_level(&self) -> u16 {
        self.health / 100
    }
}

