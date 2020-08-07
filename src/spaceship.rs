use gate::KeyCode;
use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SpriteId};

use std::collections::HashSet;
use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

pub struct Spaceship {
    x: f64,
    y: f64,
    angle: f64,
}

impl Spaceship {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Spaceship {
            x,
            y,
            angle,
        }
    }

    pub fn advance(&mut self, seconds: f64, pressed_keys: &HashSet<KeyCode>) {
        let left = pressed_keys.contains(&KeyCode::Left);
        let right = pressed_keys.contains(&KeyCode::Right);

        let turn = match (left, right) {
            (true, false) => 1.,
            (false, true) => -1.,
            _ => 0.,
        };

        self.angle += (turn * 360. * seconds) * TAU / 360.;
    }

    pub fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.x, self.y).pre_rotate(self.angle);

        renderer_s.draw(&transform, SpriteId::Spaceship);
    }
}
