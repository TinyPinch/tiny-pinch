use bevy_app::App;
use bevy_ecs::schedule::IntoSystemConfigs;

use crate::prelude::*;

pub trait CanopyAppExt {
    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self>;
}

impl CanopyAppExt for App {
    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self> {
        let world = self.world_mut();

        world.canopy_add_systems(schedule, systems)?;

        Ok(self)
    }
}
