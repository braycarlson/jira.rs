#[derive(Clone, Copy, Debug)]
pub struct FieldUpdateOperation;

impl FieldUpdateOperation {
    pub const ADD: crate::Field<Self, crate::Any> = crate::Field::new("add");
    pub const COPY: crate::Field<Self, crate::Any> = crate::Field::new("copy");
    pub const EDIT: crate::Field<Self, crate::Any> = crate::Field::new("edit");
    pub const REMOVE: crate::Field<Self, crate::Any> = crate::Field::new("remove");
    pub const SET: crate::Field<Self, crate::Any> = crate::Field::new("set");
}
