pub const SLOTS: usize = 8;
pub const MAX_STAMINA: f64 = 100.0;
pub const REGEN_PER_SEC: f64 = 5.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FruitType {
    MeraMera,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DevilFruitUser {
    pub fruit: FruitType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AbilityId {
    Hiken,
    Higan,
    Hibashira,
}

impl AbilityId {
    pub const ALL: [AbilityId; 3] = [AbilityId::Hiken, AbilityId::Higan, AbilityId::Hibashira];

    pub fn index(self) -> usize {
        self as usize
    }
    pub fn name(self) -> &'static str {
        match self {
            AbilityId::Hiken => "Hiken",
            AbilityId::Higan => "Higan",
            AbilityId::Hibashira => "Enkai: Hibashira",
        }
    }
    pub fn short(self) -> &'static str {
        match self {
            AbilityId::Hiken => "HK",
            AbilityId::Higan => "HG",
            AbilityId::Hibashira => "HB",
        }
    }
    pub fn cost(self) -> f64 {
        match self {
            AbilityId::Hiken => 20.0,
            AbilityId::Higan => 15.0,
            AbilityId::Hibashira => 40.0,
        }
    }
    pub fn cooldown(self) -> f64 {
        match self {
            AbilityId::Hiken => 4.0,
            AbilityId::Higan => 3.0,
            AbilityId::Hibashira => 10.0,
        }
    }
}

pub struct FruitStats {
    pub user: Option<DevilFruitUser>,
    pub stamina: f64,
    pub combat_hotbar_active: bool,
    pub equipped_abilities: [Option<AbilityId>; SLOTS],
    pub cooldowns: [f64; 3], // segundos restantes, por AbilityId
    pub alt_mode: [bool; SLOTS],
    pub menu_open: bool,
    pub selected: usize,
    pub cast: Option<(String, f64)>, // ultima habilidade usada e quando
    pub now: f64,
}

impl FruitStats {
    pub fn new() -> Self {
        FruitStats {
            // Nao ha gancho de "comer item" na API: a fruta ja vem ativa quando o mod esta ligado.
            user: Some(DevilFruitUser { fruit: FruitType::MeraMera }),
            stamina: MAX_STAMINA,
            combat_hotbar_active: false,
            equipped_abilities: [None; SLOTS],
            cooldowns: [0.0; 3],
            alt_mode: [false; SLOTS],
            menu_open: false,
            selected: 0,
            cast: None,
            now: 0.0,
        }
    }

    pub fn advance(&mut self, now: f64, dt: f64) {
        self.now = now;
        self.stamina = (self.stamina + REGEN_PER_SEC * dt).min(MAX_STAMINA);
        for c in self.cooldowns.iter_mut() {
            *c = (*c - dt).max(0.0);
        }
    }
}
