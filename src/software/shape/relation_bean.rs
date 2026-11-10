#[derive(Clone, Copy, Debug)]
pub struct RelationBean;

impl RelationBean {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
