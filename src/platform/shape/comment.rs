#[derive(Clone, Copy, Debug)]
pub struct Comment;

impl Comment {
    pub const AUTHOR: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("author");

    pub const BODY: crate::Field<Self, crate::Any> = crate::Field::new("body");
    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const JSD_AUTHOR_CAN_SEE_REQUEST: crate::Field<Self, crate::Boolean> =
        crate::Field::new("jsdAuthorCanSeeRequest");

    pub const JSD_PUBLIC: crate::Field<Self, crate::Boolean> = crate::Field::new("jsdPublic");

    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Object<super::EntityProperty>>> =
        crate::Field::new("properties");

    pub const RENDERED_BODY: crate::Field<Self, crate::Text> = crate::Field::new("renderedBody");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const UPDATED: crate::Field<Self, crate::Text> = crate::Field::new("updated");

    pub const UPDATE_AUTHOR: crate::Field<Self, crate::Object<super::UserDetails>> =
        crate::Field::new("updateAuthor");

    pub const VISIBILITY: crate::Field<Self, crate::Object<super::Visibility>> =
        crate::Field::new("visibility");
}
