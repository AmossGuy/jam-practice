use collider::geom::Vec2;

use gate::{AppContext, KeyCode};
use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SoundId, SpriteId};

use std::collections::HashSet;
use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

use rand::random;

use crate::Object;

fn move_value_towards(value: &mut f64, goal: f64, speed: f64) {
    if goal > *value {
        *value += speed;
        *value = value.min(goal);
    } else if goal < *value {
        *value -= speed;
        *value = value.max(goal);
    }
}

const TIMER_LENGTH: f64 = 1. / 16.;

pub struct Spaceship {
    pos: Vec2,
    angle: f64,
    lin_vel: Vec2,
    ang_vel: f64,
    charge: Option<f64>,
    effect_flags: [bool; 3],
    effect_timer: f64,
    shot_timer: f64,
}

impl Spaceship {
    pub fn new(pos: Vec2, angle: f64) -> Self {
        Spaceship {
            pos,
            angle,
            lin_vel: Vec2::new(0., 0.),
            ang_vel: 0.,
            charge: None,
            effect_flags: [false; 3],
            effect_timer: random::<f64>() * TIMER_LENGTH,
            shot_timer: f64::INFINITY,
        }
    }
}

impl Object for Spaceship {
    fn advance(
        &mut self,
        seconds: f64,
        pressed_keys: &HashSet<KeyCode>,
        ctx: &mut AppContext<AssetId>,
    ) {
        let left = pressed_keys.contains(&KeyCode::Left);
        let right = pressed_keys.contains(&KeyCode::Right);
        let up = pressed_keys.contains(&KeyCode::Up);
        let down = pressed_keys.contains(&KeyCode::Down);
        let space = pressed_keys.contains(&KeyCode::Space);

        self.shot_timer += seconds;

        if space && self.charge == None {
            self.charge = Some(seconds);

            ctx.audio.play_sound(SoundId::Charge);
        } else if let Some(c) = &self.charge {
            self.charge = Some(c + seconds);
        }

        if self.charge >= Some(1.) {
            self.shot_timer = 0.;
            self.charge = None;

            ctx.audio.play_sound(SoundId::BigShot);
        }

        self.effect_timer += seconds;
        if self.effect_timer >= TIMER_LENGTH {
            self.effect_timer -= TIMER_LENGTH;
            self.effect_flags = [
                random::<bool>(),
                random::<bool>(),
                random::<bool>()
            ];
        }

        let goal_a = match (left, right) {
            (true, false) => -360.,
            (false, true) => 360.,
            _ => 0.,
        };

        move_value_towards(&mut self.ang_vel, goal_a, 700. * seconds);

        self.angle += self.ang_vel * seconds;

        if up {
            self.lin_vel += Vec2::new(40., 0.).rotate((-self.angle + 90.) * TAU / 360.) * seconds;
        } else {
            if self.lin_vel.len() != 0. {
                 let deaccel = match down {
                     true => 80.,
                     false => 20.,
                 };

                let mut new_mag = self.lin_vel.len() - deaccel * seconds;
                new_mag = new_mag.max(0.);
                self.lin_vel = self.lin_vel.normalize().unwrap() * new_mag;
            }
        }

        self.pos += self.lin_vel * seconds;
    }

    fn render(&self, renderer: &mut Renderer<AssetId>) {
        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(self.pos.x, self.pos.y)
                               .pre_rotate(-self.angle * TAU / 360.);

        renderer_s.draw(&transform, SpriteId::Spaceship);

        if self.charge.is_some() {
            let mut eff_aff = Affine::translate(self.pos.x, self.pos.y);
            let v = Vec2::new(6., 0.).rotate((-self.angle + 90.) * TAU / 360.);
            eff_aff = eff_aff.post_translate(v.x, v.y);

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

    fn get_flash(&self) -> f64 {
        (1. - self.shot_timer / 2.).max(0.)
    }
}
