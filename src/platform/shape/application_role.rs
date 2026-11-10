#[derive(Clone, Copy, Debug)]
pub struct ApplicationRole;

impl ApplicationRole {
    pub const DEFAULT_GROUPS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("defaultGroups");

    pub const DEFAULT_GROUPS_DETAILS: crate::Field<
        Self,
        crate::List<crate::Object<super::GroupName>>,
    > = crate::Field::new("defaultGroupsDetails");

    pub const DEFINED: crate::Field<Self, crate::Boolean> = crate::Field::new("defined");
    pub const GROUPS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("groups");

    pub const GROUP_DETAILS: crate::Field<Self, crate::List<crate::Object<super::GroupName>>> =
        crate::Field::new("groupDetails");

    pub const HAS_UNLIMITED_SEATS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("hasUnlimitedSeats");

    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const NUMBER_OF_SEATS: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberOfSeats");

    pub const PLATFORM: crate::Field<Self, crate::Boolean> = crate::Field::new("platform");

    pub const REMAINING_SEATS: crate::Field<Self, crate::Integer> =
        crate::Field::new("remainingSeats");

    pub const SELECTED_BY_DEFAULT: crate::Field<Self, crate::Boolean> =
        crate::Field::new("selectedByDefault");

    pub const USER_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("userCount");

    pub const USER_DESCRIPTION_COUNT: crate::Field<Self, crate::Text> =
        crate::Field::new("userCountDescription");
}
