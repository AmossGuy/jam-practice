use collider::geom::Vec2;

use crate::asset_id::SpriteId;

use crate::Object;
use crate::spaceship::Spaceship;
use crate::tilemap::Tilemap;

const LEVELS: [&'static str; 1] = [
    include_str!("levels/level1.txt"),
];

struct LevelData {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

fn load_data(id: usize) -> LevelData {
    let grid: Vec<Vec<char>> = LEVELS[id].lines()
                                         .map(|s| s.chars().collect())
                                         .filter(|s: &Vec<char>| !s.is_empty())
                                         .collect();
    LevelData {
        width: grid[0].len(),
        height: grid.len(),
        grid,
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
}

pub fn load_level(id: usize) -> World {
    let data = load_data(id);

    let mut world = World {
        objects: Vec::new(),
    };

    let mut tilemap = Tilemap::new(data.width, data.height);

    for x in 0..data.width {
        for y in 0..data.height {
            match data.grid[y][x] {
                'q' | 'w' | 'e' | 'd' | 'c' | 'x' | 'z' | 'a' => {
                    world.objects.push(Box::new(Spaceship::new(
                        Vec2::new(x as f64 * 8., y as f64 * -8.),
                        45.,
                    )));
                },
                '#' => tilemap.set_tile(x, y, Some(SpriteId::TileR0C0)),
                ' ' => (),
                _ => panic!(),
            }
        }
    }

    world.objects.push(Box::new(tilemap));

    world
}
