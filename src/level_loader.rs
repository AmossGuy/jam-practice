use collider::geom::Vec2;

use crate::Object;
use crate::spaceship::Spaceship;

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

    for x in 0..data.width-1 {
        for y in 0..data.height-1 {
            match data.grid[y][x] {
                'q' | 'w' | 'e' | 'd' | 'c' | 'x' | 'z' | 'a' => {
                    world.objects.push(Box::new(Spaceship::new(
                        Vec2::new(x as f64 * 8., y as f64 * -8.),
                        45.,
                    )));
                },
                ' ' | '#' => (),
                _ => panic!(),
            }
        }
    }

    world
}
