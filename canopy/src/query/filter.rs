use std::marker::PhantomData;

use bevy_ecs::{component::Component, query::{ArchetypeFilter, QueryFilter, With, Without, WorldQuery}};
use graft::Grafted;

pub struct GraftedWith<T>(PhantomData<T>);

impl<T: Grafted> ArchetypeFilter for GraftedWith<T> 
where
    T::Local: Component    
{

}

impl<T: Grafted> QueryFilter for GraftedWith<T> 
where
    T::Local: Component    
{
    const IS_ARCHETYPAL: bool = <With<T::Local> as QueryFilter>::IS_ARCHETYPAL;

    unsafe fn filter_fetch(
        fetch: &mut Self::Fetch<'_>,
        entity: bevy_ecs::entity::Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> bool {
        <With<T::Local> as QueryFilter>::filter_fetch(fetch, entity, table_row)
    }
}

unsafe impl<T: Grafted> WorldQuery for GraftedWith<T> 
where
    T::Local: Component    
{
    type Item<'a> = <With<T::Local> as WorldQuery>::Item<'a>;
    type Fetch<'a> = <With<T::Local> as WorldQuery>::Fetch<'a>;
    type State = <With<T::Local> as WorldQuery>::State;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        <With<T::Local> as WorldQuery>::shrink(item)
    }

    unsafe fn init_fetch<'w>(
        world: bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy_ecs::component::Tick,
        this_run: bevy_ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        <With<T::Local> as WorldQuery>::init_fetch(world, state, last_run, this_run)
    }

    const IS_DENSE: bool = <With<T::Local> as WorldQuery>::IS_DENSE;

    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <With<T::Local> as WorldQuery>::set_archetype(fetch, state, archetype, table);
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w bevy_ecs::storage::Table) {
        <With<T::Local> as WorldQuery>::set_table(fetch, state, table);
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy_ecs::entity::Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        <With<T::Local> as WorldQuery>::fetch(fetch, entity, table_row)
    }

    fn update_component_access(state: &Self::State, access: &mut bevy_ecs::query::FilteredAccess<bevy_ecs::component::ComponentId>) {
        <With<T::Local> as WorldQuery>::update_component_access(state, access);
    }

    fn init_state(world: &mut bevy_ecs::world::World) -> Self::State {
        let components = world.components();

        components.get_id(T::foreign_type_id()).unwrap_or_else(|| panic!("'{}' does not exist in world", T::foreign_type_name()))
    }

    fn get_state(components: &bevy_ecs::component::Components) -> Option<Self::State> {
        components.get_id(T::foreign_type_id())
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <With<T::Local> as WorldQuery>::matches_component_set(state, set_contains_id)
    }
}

pub struct GraftedWithout<T>(PhantomData<T>);

impl<T: Grafted> ArchetypeFilter for GraftedWithout<T> 
where
    T::Local: Component    
{
    
}

impl<T: Grafted> QueryFilter for GraftedWithout<T> 
where
    T::Local: Component    
{
    const IS_ARCHETYPAL: bool = <Without<T::Local> as QueryFilter>::IS_ARCHETYPAL;

    unsafe fn filter_fetch(
        fetch: &mut Self::Fetch<'_>,
        entity: bevy_ecs::entity::Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> bool {
        <Without<T::Local> as QueryFilter>::filter_fetch(fetch, entity, table_row)
    }
}

unsafe impl<T: Grafted> WorldQuery for GraftedWithout<T> 
where
    T::Local: Component    
{
    type Item<'a> = <Without<T::Local> as WorldQuery>::Item<'a>;
    type Fetch<'a> = <Without<T::Local> as WorldQuery>::Fetch<'a>;
    type State = <Without<T::Local> as WorldQuery>::State;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        <Without<T::Local> as WorldQuery>::shrink(item)
    }

    unsafe fn init_fetch<'w>(
        world: bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy_ecs::component::Tick,
        this_run: bevy_ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        <Without<T::Local> as WorldQuery>::init_fetch(world, state, last_run, this_run)
    }

    const IS_DENSE: bool = <Without<T::Local> as WorldQuery>::IS_DENSE;

    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w bevy_ecs::archetype::Archetype,
        table: &'w bevy_ecs::storage::Table,
    ) {
        <Without<T::Local> as WorldQuery>::set_archetype(fetch, state, archetype, table);
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w bevy_ecs::storage::Table) {
        <Without<T::Local> as WorldQuery>::set_table(fetch, state, table);
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy_ecs::entity::Entity,
        table_row: bevy_ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        <Without<T::Local> as WorldQuery>::fetch(fetch, entity, table_row)
    }

    fn update_component_access(state: &Self::State, access: &mut bevy_ecs::query::FilteredAccess<bevy_ecs::component::ComponentId>) {
        <Without<T::Local> as WorldQuery>::update_component_access(state, access);
    }

    fn init_state(world: &mut bevy_ecs::world::World) -> Self::State {
        let components = world.components();

        components.get_id(T::foreign_type_id()).unwrap_or_else(|| panic!("'{}' does not exist in world", T::foreign_type_name()))
    }

    fn get_state(components: &bevy_ecs::component::Components) -> Option<Self::State> {
        components.get_id(T::foreign_type_id())
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(bevy_ecs::component::ComponentId) -> bool,
    ) -> bool {
        <Without<T::Local> as WorldQuery>::matches_component_set(state, set_contains_id)
    }
}
