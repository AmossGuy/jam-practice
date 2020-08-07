#[macro_use]
extern crate gate;

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::Renderer;

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId, SpriteId, MusicId, SoundId};

gate_header!();

struct Game {
}

impl Game {
    fn new() -> Self {
        Game {}
    }
}

impl App<AssetId> for Game {
    fn advance(&mut self, _seconds: f64, _ctx: &mut AppContext<AssetId>) {
    }

    fn key_down(&mut self, _key: KeyCode, _ctx: &mut AppContext<AssetId>) {
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, _ctx: &AppContext<AssetId>) {
        renderer.clear((0, 46, 85));
    }
}

fn main() {
    let info = AppInfo::with_max_dims(160., 90.)
                       .min_dims(120., 86.)
                       .tile_width(8)
                       .title("Practice");
    gate::run(info, Game::new());
}
