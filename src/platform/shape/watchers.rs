#[derive(Clone, Copy, Debug)]
pub struct Watchers;

impl Watchers {
    pub const IS_WATCHING: crate::Field<Self, crate::Boolean> = crate::Field::new("isWatching");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const WATCHERS: crate::Field<Self, crate::List<crate::Object<super::UserDetails>>> =
        crate::Field::new("watchers");

    pub const WATCH_COUNT: crate::Field<Self, crate::Integer> = crate::Field::new("watchCount");
}
