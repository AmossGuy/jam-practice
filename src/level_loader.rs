use collider::Collider;
use collider::geom::{Shape, Vec2};

use crate::asset_id::SpriteId;

use crate::Object;
use crate::collision::MyHbProfile;
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
    pub collider: Collider<MyHbProfile>,
}

pub fn load_level(id: usize) -> World {
    let data = load_data(id);

    let mut world = World {
        objects: Vec::new(),
        collider: Collider::new(),
    };

    let mut tilemap = Tilemap::new(data.width, data.height);

    let mut hb_id: u64 = 0;

    for x in 0..data.width {
        for y in 0..data.height {
            let c = data.grid[y][x];
            match data.grid[y][x] {
                'q' | 'w' | 'e' | 'd' | 'c' | 'x' | 'z' | 'a' => {
                    world.objects.push(Box::new(Spaceship::new(
                        angles(['q', 'w', 'e', 'd', 'c', 'x', 'z', 'a'], c),
                        hb_id,
                    )));
                    let hitbox = Shape::square(8.)
                        .place(Vec2::new(x as f64 * 8., y as f64 * -8.))
                        .still();
                    world.collider.add_hitbox(MyHbProfile { id: hb_id }, hitbox);
                    hb_id += 1;
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

fn angles(symbols: [char; 8], symbol: char) -> f64 {
    if symbol == symbols[0] { return -45.; }
    else if symbol == symbols[1] { return 0.; }
    else if symbol == symbols[2] { return 45.; }
    else if symbol == symbols[3] { return 90.; }
    else if symbol == symbols[4] { return 135.; }
    else if symbol == symbols[5] { return 180.; }
    else if symbol == symbols[6] { return 225.; }
    else if symbol == symbols[7] { return 270.; }
    else { panic!(); }
}
