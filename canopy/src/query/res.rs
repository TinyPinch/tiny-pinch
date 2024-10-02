use std::marker::PhantomData;

use bevy_ecs::system::{Res, ResMut, Resource, SystemParam};
use graft::Grafted;

pub struct GraftedRes<'a, T>(PhantomData<&'a T>);

unsafe impl<'a, T: Grafted> SystemParam for GraftedRes<'a, T>
where
    T::Local: Resource,
{
    type State = <Res<'a, T::Local> as SystemParam>::State;

    type Item<'world, 'state> = <Res<'a, T::Local> as SystemParam>::Item<'world, 'state>;

    fn init_state(world: &mut bevy_ecs::world::World, _: &mut bevy_ecs::system::SystemMeta) -> Self::State {
        let components = world.components();

        components.get_id(T::foreign_type_id()).unwrap_or_else(|| panic!("'{}' does not exist in world", T::foreign_type_name()))
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy_ecs::system::SystemMeta,
        world: bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy_ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        <Res<'a, T::Local> as SystemParam>::get_param(state, system_meta, world, change_tick)
    }
}


pub struct GraftedResMut<'a, T>(PhantomData<&'a mut T>);

unsafe impl<'a, T: Grafted> SystemParam for GraftedResMut<'a, T>
where
    T::Local: Resource,
{
    type State = <ResMut<'a, T::Local> as SystemParam>::State;

    type Item<'world, 'state> = <ResMut<'a, T::Local> as SystemParam>::Item<'world, 'state>;

    fn init_state(world: &mut bevy_ecs::world::World, _: &mut bevy_ecs::system::SystemMeta) -> Self::State {
        let components = world.components();

        components.get_id(T::foreign_type_id()).unwrap_or_else(|| panic!("'{}' does not exist in world", T::foreign_type_name()))
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy_ecs::system::SystemMeta,
        world: bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy_ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        <ResMut<'a, T::Local> as SystemParam>::get_param(state, system_meta, world, change_tick)
    }
}
