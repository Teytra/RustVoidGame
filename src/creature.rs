pub struct Creature {
    pub name: String,
    pub lvl: f32,
    pub xp: f32,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub defense: f32
}
impl Creature {
    pub fn create_enemy(name: String,lvl: f32,xp: f32,) -> Self {
        let start_health: f32 = 50.0;
        let start_damage: f32 = 20.0;
        let start_defense: f32 = 2.5;
        Self { name, lvl, xp,max_health: 5.0 ,health: start_health + (lvl * 50.0), damage: start_damage + (lvl * 50.0), defense: start_defense + (lvl * 2.5)}
    }
    pub fn create_player (name: String,lvl: f32,xp: f32,health: f32, max_health: f32, damage: f32, defense: f32) -> Self {

        Self {name, xp, lvl ,health , max_health, damage, defense}
    }
    pub fn calculate_score(&self) -> f32 {
        (self.lvl * 100.0 + self.xp) - 100.0
    }
}