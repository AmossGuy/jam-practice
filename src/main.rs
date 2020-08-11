#[macro_use]
extern crate gate;

gate_header!();

use collider::Collider;
use collider::geom::Vec2;

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::Renderer;

mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }
use crate::asset_id::{AssetId};

use std::collections::HashSet;

mod spaceship;
mod tilemap;

mod collision;
use collision::MyHbProfile;

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
        let target_time = self.world.collider.time() + seconds;

        for object in &mut self.world.objects {
            object.advance(seconds, &self.pressed_keys, ctx, &mut self.world.collider);
        }

        while self.world.collider.time() < target_time {
            let t = self.world.collider.next_time().min(target_time);
            self.world.collider.set_time(t);

            if let Some((_event, _profile_1, _profile_2)) = self.world.collider.next() {
            }
        }
    }

    fn key_down(&mut self, key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        self.pressed_keys.insert(key);
    }

    fn key_up(&mut self, key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        self.pressed_keys.remove(&key);
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        let mut flash = 0.;
        for object in &self.world.objects {
            flash += object.get_flash(&self.world.collider);
            if let Some(cam) = object.get_camera(&self.world.collider) {
                let (x, y) = ctx.native_px_align(cam.x, cam.y);
                self.camera = Vec2::new(x, y);
            }
        }

        let r = (0. * (1.-flash) + 255. * flash).round() as u8;
        let g = (46. * (1.-flash) + 255. * flash).round() as u8;
        let b = (85. * (1.-flash) + 255. * flash).round() as u8;

        renderer.clear((r, g, b));

        for object in &self.world.objects {
            object.render(&self.world.collider, renderer, self.camera);
        }
    }
}

pub trait Object {
    fn advance(
        &mut self,
        _seconds: f64,
        _pressed_keys: &HashSet<KeyCode>,
        _ctx: &mut AppContext<AssetId>,
        _collider: &mut Collider<MyHbProfile>,
    ) {
        // do nothing
    }
    fn render(&self, _collider: &Collider<MyHbProfile>, _renderer: &mut Renderer<AssetId>, _camera: Vec2) {
        // do nothing
    }

    fn get_flash(&self, _collider: &Collider<MyHbProfile>) -> f64 {
        0.
    }

    fn get_camera(&self, _collider: &Collider<MyHbProfile>) -> Option<Vec2> {
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
