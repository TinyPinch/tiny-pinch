use std::any::TypeId;

pub unsafe trait Grafted {
    type Local;
    
    fn foreign_type_name() -> &'static str;
    fn foreign_type_id() -> TypeId;
}
