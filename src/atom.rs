use nannou::prelude::*;

use crate::Settings;

#[derive(Copy, Clone, Debug)]
pub struct Atom {
    pub pos: Vec2,
    pub vel: Vec2,
    pub t: usize,
}

impl Atom {
    #![allow(dead_code)]
    pub fn new(pos: Vec2, vel: Vec2, t: usize) -> Self {
        Self { pos, vel, t }
    }

    pub fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            t: 0_usize,
        }
    }

    pub fn get_force(&self, p: &Atom, s: &Settings) -> Vec2 {
        let rel = &s.rel;

        let delta = p.pos - self.pos;
        let mut d = delta.length();
        if d > s.r_max {
            Vec2::ZERO
        } else {
            d /= s.r_max;
            if d < s.r_min {
                delta * ((d / s.r_min) - 1.0)
            } else {
                let avg = (s.r_min + s.r_max) / 2.0;
                if d > avg {
                    d = d - avg + s.r_min;
                }
                let g = rel.get(self.t, p.t);
                delta * s.r_max * (g * (d - s.r_min) / (avg - s.r_min))
            }
        }
    }

    pub fn apply_forces(&mut self, f: Vec2, s: &Settings) {
        self.vel = (self.vel + f) * s.friction;
    }
    
    pub fn update(&mut self) {
        self.pos += self.vel;
    }
    
    pub fn draw(&self, d: &Draw, z: f32, s: f32) {
        let col = Self::get_col(self.t);
        d.ellipse()
            .color(col)
            .x_y(self.pos.x * z, self.pos.y * z)
            .w_h(s, s);
    }

    fn get_col(t: usize) -> Rgb<u8> {
        match t {
            0 => SALMON,
            1 => SEAGREEN,
            2 => PLUM,
            3 => INDIANRED,
            4 => KHAKI,
            5 => LIGHTCORAL,
            6 => LIGHTPINK,
            _ => SEASHELL,
        }
    }
}
