use super::*;

impl State {
    pub fn spawn_guy(&mut self, name: String, random: bool) {
        let level = self.db.find_level(&name);
        let health = self.assets.config.initial_health
            + level * self.assets.config.health_increase_per_level;
        let id = self.next_id;
        self.next_id += 1;
        self.guys.insert(Guy {
            id,
            skin: self.find_skin(&name, !random),
            name,
            position: std::iter::from_fn(|| {
                Some(
                    self.camera.center
                        + vec2(
                            global_rng().gen_range(
                                0.0..self.camera.fov / 2.0
                                    * (self.framebuffer_size.x as f32
                                        / self.framebuffer_size.y as f32)
                                        .max(1.0),
                            ),
                            0.0,
                        )
                        .rotate(global_rng().gen_range(0.0..2.0 * f32::PI)),
                )
            })
            .take(50)
            .filter(|&pos| {
                for guy in &self.guys {
                    if (guy.position - pos).len() < State::MIN_DISTANCE {
                        return false;
                    }
                }
                true
            })
            .min_by_key(|&pos| r32((pos - self.circle.center).len()))
            .unwrap_or(self.camera.center),
            velocity: Vec2::ZERO,
            health,
            max_health: health,
            spawn: 0.0,
        });

        let mut sound_effect = self
            .assets
            .spawn_sfx
            .choose(&mut global_rng())
            .unwrap()
            .effect();
        sound_effect.set_volume(self.assets.config.volume);
        sound_effect.play();
    }
}
