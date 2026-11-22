use crate::components::*;

pub const W: f32 = 8.0 * 24.0;
const BG: u32 = 0x7800_0000;
const BG_COMBAT: u32 = 0x7840_2000;
const DIM: u32 = 0xB000_0000;
const WHITE: u32 = 0xFFFF_FFFF;
const GOLD: u32 = 0xFFFF_AA00;
const GRAY: u32 = 0xFF99_9999;
const RED: u32 = 0xFFFF_5555;

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    /// x, y, w, h e argb.
    Fill([f32; 4], u32),
    /// texto, x, y, argb, sombra.
    Label(String, [f32; 2], u32, bool),
}

fn centered(out: &mut Vec<Shape>, text: &str, [x, y, w, _h]: [f32; 4], argb: u32, width: &dyn Fn(&str) -> f32) {
    out.push(Shape::Label(text.to_owned(), [(x + (w - width(text)) / 2.0).round(), y.round()], argb, true));
}

/// O painel (relativo ao canto de cima a esquerda) e a altura dele.
pub fn shapes(s: &FruitStats, width: &dyn Fn(&str) -> f32) -> (Vec<Shape>, f32) {
    let mut out = Vec::new();
    let mut y = 0.0_f32;

    if s.menu_open {
        out.push(Shape::Label("== Abilities ==  (L fecha)".into(), [0.0, y], GOLD, true));
        y += 10.0;
        for (i, a) in AbilityId::ALL.iter().enumerate() {
            let mark = if i == s.selected { ">" } else { " " };
            let text = format!("{mark} {}  custo {:.0}  recarga {:.0}s", a.name(), a.cost(), a.cooldown());
            out.push(Shape::Label(text, [0.0, y], WHITE, true));
            y += 10.0;
        }
        out.push(Shape::Label("A/D escolhe | 1-8 equipa no slot".into(), [0.0, y], GRAY, true));
        y += 12.0;
    }

    if let Some((text, at)) = &s.cast {
        if s.now - at < 1.5 {
            out.push(Shape::Label(text.clone(), [0.0, y], GOLD, true));
            y += 10.0;
        }
    }

    // Estamina.
    let frac = (s.stamina / MAX_STAMINA).clamp(0.0, 1.0) as f32;
    out.push(Shape::Fill([0.0, y, W, 8.0], BG));
    out.push(Shape::Fill([0.0, y, W * frac, 8.0], if s.stamina < 20.0 { RED } else { 0xFFFF_8800 }));
    let tag = if s.combat_hotbar_active { "COMBATE  " } else { "" };
    out.push(Shape::Label(format!("{tag}Estamina {:.0}/{:.0}", s.stamina, MAX_STAMINA), [2.0, y], WHITE, true));
    y += 11.0;

    // Hotbar de habilidades.
    if s.combat_hotbar_active || s.menu_open {
        let bg = if s.combat_hotbar_active { BG_COMBAT } else { BG };
        for slot in 0..SLOTS {
            let x = slot as f32 * 24.0;
            out.push(Shape::Fill([x, y, 22.0, 22.0], bg));
            out.push(Shape::Label(format!("{}", slot + 1), [x + 2.0, y + 1.0], GRAY, false));
            if let Some(a) = s.equipped_abilities[slot] {
                let alt = if s.alt_mode[slot] { "*" } else { "" };
                centered(&mut out, &format!("{}{alt}", a.short()), [x, y + 7.0, 22.0, 9.0], WHITE, width);
                let cd = s.cooldowns[a.index()];
                if cd > 0.0 {
                    let f = (cd / a.cooldown()).clamp(0.0, 1.0) as f32;
                    out.push(Shape::Fill([x, y, 22.0, 22.0 * f], DIM));
                    centered(&mut out, &format!("{cd:.1}"), [x, y + 13.0, 22.0, 9.0], GOLD, width);
                }
            }
        }
        y += 22.0;
    }
    (out, y)
}
