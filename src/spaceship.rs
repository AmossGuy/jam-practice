use collider::{Collider, HbVel};
use collider::geom::Vec2;

use gate::{AppContext, KeyCode};
use gate::renderer::{Affine, Renderer};

use crate::asset_id::{AssetId, SoundId, SpriteId};

use std::collections::HashSet;
use std::f64::consts::PI;
const TAU: f64 = 2. * PI;

use rand::random;

use crate::Object;
use crate::SCREEN_SIZE;
use crate::collision::MyHbProfile;

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
    angle: f64,
    ang_vel: f64,
    charge: Option<f64>,
    effect_flags: [bool; 3],
    effect_timer: f64,
    shot_timer: f64,
    hb_id: u64,
}

impl Spaceship {
    pub fn new(angle: f64, hb_id: u64) -> Self {
        Spaceship {
            angle,
            ang_vel: 0.,
            charge: None,
            effect_flags: [false; 3],
            effect_timer: random::<f64>() * TIMER_LENGTH,
            shot_timer: f64::INFINITY,
            hb_id,
        }
    }
}

impl Object for Spaceship {
    fn advance(
        &mut self,
        seconds: f64,
        pressed_keys: &HashSet<KeyCode>,
        ctx: &mut AppContext<AssetId>,
        collider: &mut Collider<MyHbProfile>,
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

        let prev_vel = collider.get_hitbox(self.hb_id).vel.value;

        if up {
            let extra = Vec2::new(40., 0.).rotate((-self.angle + 90.) * TAU / 360.) * seconds;
            collider.set_hitbox_vel(self.hb_id, HbVel::moving(prev_vel + extra));
        } else {
            if prev_vel.len() != 0. {
                 let deaccel = match down {
                     true => 80.,
                     false => 20.,
                 };

                let mut new_mag = prev_vel.len() - deaccel * seconds;
                new_mag = new_mag.max(0.);
                let aaaaaa = prev_vel.normalize().unwrap() * new_mag;
                collider.set_hitbox_vel(self.hb_id, HbVel::moving(aaaaaa));
            }
        }
    }
    fn render(&self, collider: &Collider<MyHbProfile>, renderer: &mut Renderer<AssetId>, camera: Vec2) {
        let pos = collider.get_hitbox(self.hb_id).value.pos;

        let mut renderer_s = renderer.sprite_mode();

        let transform = Affine::translate(pos.x, pos.y)
                               .pre_rotate(-self.angle * TAU / 360.)
                               .post_translate(-camera.x, -camera.y);

        renderer_s.draw(&transform, SpriteId::Spaceship);

        if self.charge.is_some() {
            let mut eff_aff = Affine::translate(pos.x, pos.y);
            let v = Vec2::new(6., 0.).rotate((-self.angle + 90.) * TAU / 360.);
            eff_aff = eff_aff.post_translate(v.x, v.y);
            eff_aff = eff_aff.post_translate(-camera.x, -camera.y);

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

    fn get_flash(&self, _collider: &Collider<MyHbProfile>) -> f64 {
        (1. - self.shot_timer / 2.).max(0.)
    }

    fn get_camera(&self, collider: &Collider<MyHbProfile>) -> Option<Vec2> {
        let pos = collider.get_hitbox(self.hb_id).value.pos;
        Some(pos - Vec2::new(SCREEN_SIZE.x / 2., SCREEN_SIZE.y / 2.))
    }
}
