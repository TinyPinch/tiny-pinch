use std::{any::{type_name, TypeId}, marker::PhantomData};

use graft::Grafted;

use crate::DUMP;

pub struct Map<T>(PhantomData<T>);

unsafe impl<T> Grafted for Map<T> {
    type Local = T;

    #[inline]
    fn foreign_type_name() -> &'static str {
        type_name::<T>()
    }

    #[inline]
    fn foreign_type_id() -> TypeId {
        let type_name = type_name::<T>();
        DUMP.types().get_type_id(type_name).unwrap_or_else(|| panic!("'{type_name}' does not exist in Tiny Glade"))
    }
}
