use std::{any::{type_name, TypeId}, mem::transmute};

use const_fnv1a_hash::fnv1a_hash_str_64;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Mapping between real types and Tiny Glade type information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Types(HashMap<u64, (u64, u64)>);

impl Types {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn get_type_id(&self, name: &str) -> Option<TypeId> {
        let hash = fnv1a_hash_str_64(name);

        let type_id_internals = *self.0.get(&hash)?;

        Some(unsafe { transmute(type_id_internals) })
    }

    /// Gets the type id of this type **as far as Tiny Glade is concerned** this is not the same as `TypeId::of::<T>()`.
    pub fn type_id_of<T>(&self) -> Option<TypeId> {
        self.get_type_id(type_name::<T>())
    }

    pub fn insert(&mut self, name: &str, type_id: TypeId) {
        let type_id_internals = unsafe { transmute(type_id) };
        
        self.0.insert(fnv1a_hash_str_64(name), type_id_internals);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
