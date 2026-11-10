#[derive(Clone, Copy, Debug)]
pub struct Filter;

impl Filter {
    pub const APPROXIMATE_USED_LAST: crate::Field<Self, crate::Text> =
        crate::Field::new("approximateLastUsed");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const EDIT_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("editPermissions");

    pub const FAVOURITE: crate::Field<Self, crate::Boolean> = crate::Field::new("favourite");

    pub const FAVOURITED_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("favouritedCount");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_WRITABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isWritable");
    pub const JQL: crate::Field<Self, crate::Text> = crate::Field::new("jql");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const OWNER: crate::Field<Self, crate::Object<super::User>> = crate::Field::new("owner");
    pub const SEARCH_URL: crate::Field<Self, crate::Text> = crate::Field::new("searchUrl");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const SHARED_USERS: crate::Field<Self, crate::Object<super::UserList>> =
        crate::Field::new("sharedUsers");

    pub const SHARE_PERMISSIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::SharePermission>>,
    > = crate::Field::new("sharePermissions");

    pub const SUBSCRIPTIONS: crate::Field<Self, crate::Object<super::FilterSubscriptionsList>> =
        crate::Field::new("subscriptions");

    pub const VIEW_URL: crate::Field<Self, crate::Text> = crate::Field::new("viewUrl");
}
