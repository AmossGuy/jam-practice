use gate::KeyCode;
use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SpriteId};

use std::collections::HashSet;
use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

use rand::random;

use crate::vector2::Vector2;

fn move_value_towards(value: &mut f64, goal: f64, speed: f64) {
    if goal > *value {
        *value += speed;
        *value = value.min(goal);
    } else if goal < *value {
        *value -= speed;
        *value = value.max(goal);
    }
}

const timer_length: f64 = 1. / 16.;

pub struct Spaceship {
    pos: Vector2,
    angle: f64,
    lin_vel: Vector2,
    ang_vel: f64,
    charge: Option<f64>,
    effect_flags: [bool; 3],
    effect_timer: f64,
}

impl Spaceship {
    pub fn new(pos: Vector2, angle: f64) -> Self {
        Spaceship {
            pos,
            angle,
            lin_vel: Vector2::new(0., 0.),
            ang_vel: 0.,
            charge: None,
            effect_flags: [false; 3],
            effect_timer: random::<f64>() * timer_length,
        }
    }

    pub fn advance(&mut self, seconds: f64, pressed_keys: &HashSet<KeyCode>) {
        let left = pressed_keys.contains(&KeyCode::Left);
        let right = pressed_keys.contains(&KeyCode::Right);
        let up = pressed_keys.contains(&KeyCode::Up);
        let down = pressed_keys.contains(&KeyCode::Down);
        let space = pressed_keys.contains(&KeyCode::Space);

        if space && self.charge == None {
            self.charge = Some(seconds);
        } else if let Some(c) = &self.charge {
            self.charge = Some(c + seconds);
        }

        if self.charge >= Some(1.) {
            self.charge = None;
        }

        self.effect_timer += seconds;
        if self.effect_timer >= timer_length {
            self.effect_timer -= timer_length;
            self.effect_flags = [
                random::<bool>(),
                random::<bool>(),
                random::<bool>()
            ];
        }

        let goal_a = match (left, right) {
            (true, false) => 360.,
            (false, true) => -360.,
            _ => 0.,
        };

        move_value_towards(&mut self.ang_vel, goal_a, 700. * seconds);

        self.angle += self.ang_vel * seconds;

        if up {
            self.lin_vel += Vector2::from_dir_mag(self.angle, 40.) * seconds;
        } else {
            if self.lin_vel.magnitude() != 0. {
                 let deaccel = match down {
                     true => 80.,
                     false => 20.,
                 };

                let mut new_mag = self.lin_vel.magnitude() - deaccel * seconds;
                new_mag = new_mag.max(0.);
                self.lin_vel = self.lin_vel.unit() * new_mag;
            }
        }

        self.pos += self.lin_vel * seconds;
    }

    pub fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.pos.x, self.pos.y)
                               .pre_rotate(self.angle * TAU / 360.);

        renderer_s.draw(&transform, SpriteId::Spaceship);

        if self.charge.is_some() {
            let mut eff_aff = Affine::translate(self.pos.x, self.pos.y);
            let v = Vector2::from_dir_mag(self.angle, 6.);
            eff_aff = eff_aff.post_translate(v.x, v.y); // .pre_rotate(self.angle * TAU / 360.);

            if self.effect_flags[0] {
                eff_aff = eff_aff.pre_rotate(90. * TAU / 360.);
            }
            if self.effect_flags[1] {
                eff_aff = eff_aff.pre_scale_axes(-1., 1.);
            }
            if self.effect_flags[1] {
                eff_aff = eff_aff.pre_scale_axes(1., -1.);
            }

            renderer_s.draw(&eff_aff, SpriteId::Charge);
        }
    }
}
