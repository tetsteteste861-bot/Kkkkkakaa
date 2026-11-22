use crate::components::*;

/// Tenta usar a habilidade do slot: confere estamina e cooldown, cobra e inicia a recarga.
/// Devolve true se disparou. O efeito no mundo (projetil, fogo, knockback) fica por conta da
/// API do jogo; aqui o disparo aparece no HUD.
pub fn try_use(stats: &mut FruitStats, slot: usize) -> bool {
    if stats.user.is_none() {
        return false;
    }
    let Some(ability) = stats.equipped_abilities[slot] else {
        return false;
    };
    let i = ability.index();
    if stats.cooldowns[i] > 0.0 || stats.stamina < ability.cost() {
        return false;
    }
    stats.stamina -= ability.cost();
    stats.cooldowns[i] = ability.cooldown();
    let alt = if stats.alt_mode[slot] { " (alt)" } else { "" };
    stats.cast = Some((format!("{}{}!", ability.name(), alt), stats.now));
    true
}

/// Uma habilidade ocupa um slot so.
pub fn equip(stats: &mut FruitStats, ability: AbilityId, slot: usize) {
    for s in stats.equipped_abilities.iter_mut() {
        if *s == Some(ability) {
            *s = None;
        }
    }
    stats.equipped_abilities[slot] = Some(ability);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn costs_stamina_and_starts_cooldown() {
        let mut s = FruitStats::new();
        equip(&mut s, AbilityId::Hiken, 0);
        assert!(try_use(&mut s, 0));
        assert_eq!(s.stamina, 80.0);
        assert!(!try_use(&mut s, 0), "em recarga");
        s.advance(5.0, 4.0);
        assert!(try_use(&mut s, 0));
    }
}
