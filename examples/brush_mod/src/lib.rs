use std::{any::TypeId, mem::swap};

use bevy_ecs::system::ResMut;
use bevy_input::{keyboard::KeyCode, ButtonInput};
use canopy::{canopy_mod, map::Map, prelude::{bevy_app::App, bevy_ecs::system::Resource, CanopyAppExt}, query::res::{GraftedRes, GraftedResMut}, CanopyMod, DUMP};
use clap::Parser;
use graft::Grafted;

pub struct BrushMod;

#[derive(Parser)]
pub struct Arguments {
    #[arg(default_value = "6.")]
    pub brush_size: f32,
}

impl CanopyMod for BrushMod {
    type Arguments = Arguments;

    fn initialize(_: &Self::Arguments) -> Self {
        Self
    }

    fn build(&self, arguments: &Self::Arguments, app: &mut App) -> canopy::Result<()> {
        app
            .insert_resource(ModBrushSizes::new(arguments.brush_size))
            .canopy_add_systems("Update", update_brush_size)?;

        Ok(())
    }
}

// This is the foreign brush sizes. This is grafted to the Tiny Glade internal BrushSizes resource
#[derive(Debug, Resource)]
pub struct TinyGladeBrushSizes(pub BrushSizes);

// This is the local brush sizes. We use this to store the previous values before we change them so we can swap back and forth.
#[derive(Debug, Resource)]
pub struct ModBrushSizes(pub BrushSizes);

impl ModBrushSizes {
    fn new(size: f32) -> Self {
        Self(BrushSizes { path: size, water: size, garden: size })
    }
}

#[derive(Debug)]
pub struct BrushSizes {
    pub path: f32,
    pub water: f32,
    pub garden: f32,
}

unsafe impl Grafted for TinyGladeBrushSizes {
    type Local = Self;

    fn foreign_type_name() -> &'static str {
        "country_core::resources::brush_sizes::BrushSizes"
    }

    fn foreign_type_id() -> TypeId {
        DUMP.types().get_type_id(Self::foreign_type_name()).unwrap()
    }
}

fn update_brush_size(
    GraftedRes(keys): GraftedRes<Map<ButtonInput<KeyCode>>>,
    GraftedResMut(mut brush_sizes): GraftedResMut<TinyGladeBrushSizes>,
    mut storage: ResMut<ModBrushSizes>,
) {
    // Here we can't just use `just_pressed` so we have to iterate and manually check. I am not positive why the Hash impl is different...
    for key in keys.get_just_pressed() {
        if *key == KeyCode::Backquote {
            swap(&mut brush_sizes.0, &mut storage.0);
        }
    }
}

canopy_mod!(BrushMod);
