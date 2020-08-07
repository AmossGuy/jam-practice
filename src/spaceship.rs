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
    lin_vel: f64,
    angle: f64,
    ang_vel: f64,
}

impl Spaceship {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Spaceship {
            x,
            y,
            lin_vel: 0.,
            angle,
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

        let goal_l = match up {
            true => 100.,
            false => 0.,
        };

        let g_speed_l = match down && !up {
            true => 160.,
            false => 40.,
        };

        move_value_towards(&mut self.lin_vel, goal_l, g_speed_l * seconds);

        self.x += -(self.angle * TAU / 360.).sin() * seconds * self.lin_vel;
        self.y += (self.angle * TAU / 360.).cos() * seconds * self.lin_vel;
    }

    pub fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.x, self.y).pre_rotate(self.angle * TAU / 360.);

        renderer_s.draw(&transform, SpriteId::Spaceship);
    }
}
