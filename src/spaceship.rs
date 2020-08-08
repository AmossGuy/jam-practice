use gate::KeyCode;
use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SpriteId};

use std::collections::HashSet;
use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

fn move_value_towards(value: &mut f64, goal: f64, speed: f64) {
    if goal > *value {
        *value += speed;
        *value = value.min(goal);
    } else if goal < *value {
        *value -= speed;
        *value = value.max(goal);
    }
}

pub struct Spaceship {
    x: f64,
    y: f64,
    angle: f64,
    x_vel: f64,
    y_vel: f64,
    ang_vel: f64,
}

impl Spaceship {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Spaceship {
            x,
            y,
            angle,
            x_vel: 0.,
            y_vel: 0.,
            ang_vel: 0.,
        }
    }

    pub fn advance(&mut self, seconds: f64, pressed_keys: &HashSet<KeyCode>) {
        let left = pressed_keys.contains(&KeyCode::Left);
        let right = pressed_keys.contains(&KeyCode::Right);
        let up = pressed_keys.contains(&KeyCode::Up);
        let down = pressed_keys.contains(&KeyCode::Down);

        let goal_a = match (left, right) {
            (true, false) => 360.,
            (false, true) => -360.,
            _ => 0.,
        };

        move_value_towards(&mut self.ang_vel, goal_a, 700. * seconds);

        self.angle += self.ang_vel * seconds;

        let g_speed_l = match up {
            true => 40.,
            false => 0.,
        };

        self.x_vel += -(self.angle * TAU / 360.).sin() * seconds * g_speed_l;
        self.y_vel += (self.angle * TAU / 360.).cos() * seconds * g_speed_l;

        self.x += seconds * self.x_vel;
        self.y += seconds * self.y_vel;
    }

    pub fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.x, self.y).pre_rotate(self.angle * TAU / 360.);

        renderer_s.draw(&transform, SpriteId::Spaceship);
    }
}
