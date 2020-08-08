#[macro_use]
extern crate gate;

gate_header!();

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::Renderer;

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId};

use std::collections::HashSet;

mod spaceship;
use spaceship::Spaceship;

mod vector2;
use vector2::Vector2;

struct Game {
    pressed_keys: HashSet<KeyCode>,
    objects: Vec<Spaceship>,
}

impl Game {
    fn new() -> Self {
        Game {
            pressed_keys: HashSet::new(),
            objects: vec![Spaceship::new(Vector2::new(10., 10.), -45.)],
        }
    }
}

impl App<AssetId> for Game {
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<AssetId>) {
        for object in &mut self.objects {
            object.advance(seconds, &self.pressed_keys, ctx);
        }
    }

    fn key_down(&mut self, key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        self.pressed_keys.insert(key);
    }

    fn key_up(&mut self, key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        self.pressed_keys.remove(&key);
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, _ctx: &AppContext<AssetId>) {
        let mut flash = 0.;
        for object in &self.objects {
            flash += object.get_flash();
        }

        let r = (0. * (1.-flash) + 255. * flash).round() as u8;
        let g = (46. * (1.-flash) + 255. * flash).round() as u8;
        let b = (85. * (1.-flash) + 255. * flash).round() as u8;

        renderer.clear((r, g, b));

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
