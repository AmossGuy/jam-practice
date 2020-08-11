use collider::Collider;
use collider::geom::Vec2;

use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SpriteId};

use crate::Object;
use crate::collision::MyHbProfile;

pub struct Tilemap {
    width: usize,
    height: usize,
    grid: Vec<Vec<Option<SpriteId>>>,
}

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Self {
        Tilemap {
            width,
            height,
            grid: vec![vec![None; width]; height],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Option<SpriteId>) {
        self.grid[y][x] = tile;
    }
}

impl Object for Tilemap {
    fn render(&self, _collider: &Collider<MyHbProfile>, renderer: &mut Renderer<AssetId>, camera: Vec2) {
        let mut renderer_s = renderer.sprite_mode();

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(tile) = self.grid[y][x] {
                    let transform = Affine::translate(x as f64 * 8., y as f64 * -8.)
                        .pre_translate(-camera.x, -camera.y);
                    renderer_s.draw(&transform, tile);
                }
            }
        }
    }
}
