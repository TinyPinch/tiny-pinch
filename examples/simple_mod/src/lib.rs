use bevy_app::App;
use bevy_ecs::{component::Component, entity::Entity, event::Events, system::Query};
use canopy::{map::Map, prelude::*, query::{filter::GraftedWith, res::GraftedRes}, DUMP};
use clap::Parser;
use graft::Grafted;
use tracing::info;

pub struct SimpleMod;

#[derive(Parser)]
pub struct Arguments;

impl CanopyMod for SimpleMod {
    type Arguments = Arguments;

    fn initialize(_: &Self::Arguments) -> Self {
        Self
    }

    fn build(&self, _: &Self::Arguments, app: &mut App) -> canopy::Result<()> {
        app.canopy_add_systems("Update", system)?;

        Ok(())
    }
}

#[derive(Component)]
pub struct Critter;

unsafe impl Grafted for Critter {
    type Local = Self;

    fn foreign_type_name() -> &'static str {
        "country_core::resources::critters::Critter"
    }

    fn foreign_type_id() -> std::any::TypeId {
        DUMP.types().get_type_id(Self::foreign_type_name()).unwrap()
    }
}

fn system(critters: Query<Entity, GraftedWith<Critter>>) {
    let count = critters.iter().len();

    info!("{count} critters")
}

canopy_mod!(SimpleMod);
