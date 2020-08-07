use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SpriteId};

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

    pub fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.x, self.y).pre_rotate(self.angle);

        renderer_s.draw(&transform, SpriteId::Spaceship);
    }
}
