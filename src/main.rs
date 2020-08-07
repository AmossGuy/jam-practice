#[macro_use]
extern crate gate;

gate_header!();

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::{Affine, Renderer};

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId, SpriteId, MusicId, SoundId};

mod spaceship;
use spaceship::Spaceship;

use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

struct Game {
    objects: Vec<Spaceship>,
}

impl Game {
    fn new() -> Self {
        Game {
            objects: vec![Spaceship::new(10., 10., -45. * TAU / 360.)],
        }
    }
}

impl App<AssetId> for Game {
    fn advance(&mut self, _seconds: f64, _ctx: &mut AppContext<AssetId>) {
    }

    fn key_down(&mut self, _key: KeyCode, _ctx: &mut AppContext<AssetId>) {
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, _ctx: &AppContext<AssetId>) {
        renderer.clear((0, 46, 85));

        for object in &self.objects {
            object.render(renderer);
        }
    }
}

fn main() {
    let info = AppInfo::with_max_dims(200., 200.)
                       .min_dims(200., 200.)
                       .tile_width(8)
                       .title("Practice");
    gate::run(info, Game::new());
}
