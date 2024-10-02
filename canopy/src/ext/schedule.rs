use bevy_ecs::schedule::{IntoSystemConfigs, Schedule, Schedules};
use tracing::error;

use crate::CanopyError;

pub trait CanopySchedulesExt {
    fn canopy_schedule_by_name(&self, schedule: &str) -> Option<&Schedule>;
    fn canopy_schedule_mut_by_name(&mut self, schedule: &str) -> Option<&mut Schedule>;
    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self>;
}

impl CanopySchedulesExt for Schedules {
    fn canopy_schedule_by_name(&self, schedule: &str) -> Option<&Schedule> {
        self
            .iter()
            .find_map(|(l, s)| if format!("{l:?}") == schedule { Some(s) } else { None })
    }

    fn canopy_schedule_mut_by_name(&mut self, schedule: &str) -> Option<&mut Schedule> {
        self
            .iter_mut()
            .find_map(|(l, s)| if format!("{l:?}") == schedule { Some(s) } else { None })
    }

    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self> {
        let Some(schedule) = self.canopy_schedule_mut_by_name(schedule) else {
            error!("Could not find schedule: {schedule:?}");
            return Err(CanopyError::TypeNotFound)
        };

        schedule.add_systems(systems);

        Ok(self)
    }
}
