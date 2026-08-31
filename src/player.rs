pub struct Player {
    pub name: String,
    pub lvl: u16,
    pub xp: u16,
    pub health: u16,
    pub max_health: u16,
    pub max_take_damage: u16
}
impl Player {
    pub fn new(name: String) -> Self {
        Self { name, xp: 0, lvl: 1, health: 100, max_health: 100, max_take_damage: 70 }
    }
    pub fn calculate_score(&self) -> u16 {
        (self.lvl * 100 + self.xp) - 100
    }
}