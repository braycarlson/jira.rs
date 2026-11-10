#[derive(Clone, Copy, Debug)]
pub struct NewUserDetails;

impl NewUserDetails {
    pub const APPLICATION_KEYS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("applicationKeys");

    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EMAIL_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("emailAddress");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PASSWORD: crate::Field<Self, crate::Text> = crate::Field::new("password");

    pub const PRODUCTS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("products");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
}
