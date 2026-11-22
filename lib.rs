//! Mera Mera no Mi: menu de habilidades, hotbar de combate, estamina e cooldowns.
//!
//! Teclas (o mod le os mapeamentos do jogo, em Opcoes > Controles):
//!   F (key.swapOffhand)  alterna a hotbar de combate
//!   L (key.advancements) abre/fecha o menu Abilities
//!   A / D                escolhem a habilidade no menu
//!   1..8 (hotbar)        no menu equipam no slot; em combate disparam; com Shift alternam o modo
mod abilities;
mod components;
mod ui;

use std::sync::Mutex;

use mod_api::{
    ClientHost, Corner, Hooks, Registrar, RegistrationContext, RenderFramePreEvent,
    RenderGuiPostEvent,
};

use components::*;

pub const ID: &str = "mera_mera_logic";

/// Mapeamentos lidos a cada quadro: combate, menu, esquerda, direita e os 8 slots.
const KEYS: [&str; 12] = [
    "key.swapOffhand",
    "key.advancements",
    "key.left",
    "key.right",
    "key.hotbar.1",
    "key.hotbar.2",
    "key.hotbar.3",
    "key.hotbar.4",
    "key.hotbar.5",
    "key.hotbar.6",
    "key.hotbar.7",
    "key.hotbar.8",
];

struct State {
    stats: FruitStats,
    was: [bool; 12],
    last: Option<f64>,
}

struct Mera {
    // Nunca segurado durante uma chamada ao host.
    state: Mutex<State>,
}

impl Default for Mera {
    fn default() -> Self {
        Mera {
            state: Mutex::new(State { stats: FruitStats::new(), was: [false; 12], last: None }),
        }
    }
}

impl Mera {
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn down(host: &dyn ClientHost, name: &str) -> bool {
    matches!(host.mapping(name), Ok(Some(m)) if m.down)
}

fn on_edges(s: &mut FruitStats, edge: [bool; 12], shift: bool) {
    if s.user.is_none() {
        return;
    }
    if edge[0] {
        s.combat_hotbar_active = !s.combat_hotbar_active;
    }
    if edge[1] {
        s.menu_open = !s.menu_open;
    }
    if s.menu_open {
        let n = AbilityId::ALL.len();
        if edge[2] {
            s.selected = (s.selected + n - 1) % n;
        }
        if edge[3] {
            s.selected = (s.selected + 1) % n;
        }
    }
    for slot in 0..SLOTS {
        if !edge[4 + slot] {
            continue;
        }
        if s.menu_open {
            abilities::equip(s, AbilityId::ALL[s.selected], slot);
        } else if s.combat_hotbar_active {
            if shift {
                s.alt_mode[slot] = !s.alt_mode[slot];
            } else {
                abilities::try_use(s, slot);
            }
        }
    }
}

#[mod_api::horizon_mod]
impl Hooks for Mera {
    fn register(&self, _r: &mut dyn Registrar, _: &RegistrationContext) {}

    /// Todo quadro: le as teclas, regenera estamina e abaixa os cooldowns.
    fn render_frame_pre(&self, host: &mut dyn ClientHost, _: &mut RenderFramePreEvent) {
        if !host.on(ID) {
            return;
        }
        let host = &*host;
        let now = host.millis() as f64 / 1000.0;
        let playing = host.playing();
        let keys: [bool; 12] = std::array::from_fn(|i| playing && down(host, KEYS[i]));
        let shift = playing && down(host, "key.sneak");
        let mut st = self.state();
        let dt = st.last.map_or(0.0, |l| (now - l).clamp(0.0, 1.0));
        st.last = Some(now);
        let was = st.was;
        st.was = keys;
        let edge: [bool; 12] = std::array::from_fn(|i| keys[i] && !was[i]);
        st.stats.advance(now, dt);
        on_edges(&mut st.stats, edge, shift);
    }

    /// O painel no canto de baixo a esquerda.
    fn render_gui_post(&self, host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>) {
        if !host.on(ID) || !host.playing() {
            return;
        }
        let draw = &mut *event.draw;
        let (shapes, h) = {
            let st = self.state();
            ui::shapes(&st.stats, &|t| draw.text_width(t))
        };
        let (x, y) = event.corners.place(Corner::BottomLeft, ui::W, h);
        for shape in shapes {
            match shape {
                ui::Shape::Fill([dx, dy, w, h], argb) => draw.fill([x + dx, y + dy, w, h], argb),
                ui::Shape::Label(text, [dx, dy], argb, shadow) => {
                    draw.text(&text, [x + dx, y + dy], argb, shadow)
                }
            }
        }
    }
}
