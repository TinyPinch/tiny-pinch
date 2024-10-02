use bevy_ecs::{schedule::{IntoSystemConfigs, Schedules}, system::Resource, world::{Mut, World}};
use graft::Grafted;

use crate::{map::Map, prelude::*};

pub trait CanopyWorldExt {
    fn canopy_resource<R>(&self) -> &<R as Grafted>::Local 
    where 
        R: Grafted,
        R::Local: Resource;
    fn canopy_resource_mut<'w, R>(&'w mut self) -> Mut<'w, <R as Grafted>::Local> 
    where 
        R: Grafted,
        R::Local: Resource;
    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self>;
}

impl CanopyWorldExt for World {
    fn canopy_resource<R>(&self) -> &<R as Grafted>::Local 
    where 
        R: Grafted,
        R::Local: Resource,
    {
        let component_id = self.components()
            .get_resource_id(R::foreign_type_id())
            .unwrap_or_else(|| panic!("'{}' not found in world", R::foreign_type_name()));

        let ptr = self.get_resource_by_id(component_id).unwrap_or_else(|| panic!("'{}' not in world", R::foreign_type_name()));

        unsafe { ptr.deref() }
    }
    
    fn canopy_resource_mut<'w, R>(&'w mut self) -> Mut<'w, <R as Grafted>::Local> 
    where 
        R: Grafted,
        R::Local: Resource,
    {
        let component_id = self.components()
            .get_resource_id(R::foreign_type_id())
            .unwrap_or_else(|| panic!("'{}' not found in Tiny Glade", R::foreign_type_name()));

        let ptr = self.get_resource_mut_by_id(component_id).unwrap_or_else(|| panic!("'{}' not in world", R::foreign_type_name()));

        unsafe { ptr.with_type() }
    }

    fn canopy_add_systems<M>(&mut self, schedule: &str, systems: impl IntoSystemConfigs<M>) -> crate::Result<&mut Self> {
        let mut schedules = self.canopy_resource_mut::<Map<Schedules>>();

        schedules.canopy_add_systems(schedule, systems)?;

        Ok(self)
    }
}
