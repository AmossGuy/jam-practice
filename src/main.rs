#[macro_use]
extern crate gate;

gate_header!();

use collider::geom::Vec2;

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::Renderer;

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId};

use std::collections::HashSet;

mod spaceship;

mod level_loader;
use level_loader::{load_level, World};

const SCREEN_SIZE: Vec2 = Vec2 {x: 200., y: 200.};

struct Game {
    pressed_keys: HashSet<KeyCode>,
    camera: Vec2,
    world: World,
}

impl Game {
    fn new() -> Self {
        Game {
            pressed_keys: HashSet::new(),
            camera: Vec2::new(0., 0.),
            world: load_level(0),
        }
    }
}

impl App<AssetId> for Game {
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<AssetId>) {
        for object in &mut self.world.objects {
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
        for object in &self.world.objects {
            flash += object.get_flash();
            if let Some(cam) = object.get_camera() {
                self.camera = cam;
            }
        }

        let r = (0. * (1.-flash) + 255. * flash).round() as u8;
        let g = (46. * (1.-flash) + 255. * flash).round() as u8;
        let b = (85. * (1.-flash) + 255. * flash).round() as u8;

        renderer.clear((r, g, b));

        for object in &self.world.objects {
            object.render(renderer, self.camera);
        }
    }
}

pub trait Object {
    fn advance(
        &mut self,
        seconds: f64,
        pressed_keys: &HashSet<KeyCode>,
        ctx: &mut AppContext<AssetId>,
    );

    fn render(&self, renderer: &mut Renderer<AssetId>, camera: Vec2);

    fn get_flash(&self) -> f64 {
        0.
    }

    fn get_camera(&self) -> Option<Vec2> {
        None
    }
}

fn main() {
    let info = AppInfo::with_max_dims(SCREEN_SIZE.x, SCREEN_SIZE.y)
                       .min_dims(SCREEN_SIZE.x, SCREEN_SIZE.y)
                       .tile_width(8)
                       .title("Practice");
    gate::run(info, Game::new());
}
