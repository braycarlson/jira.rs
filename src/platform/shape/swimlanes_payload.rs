#[derive(Clone, Copy, Debug)]
pub struct SwimlanesPayload;

impl SwimlanesPayload {
    pub const CUSTOM_SWIMLANES: crate::Field<
        Self,
        crate::List<crate::Object<super::SwimlanePayload>>,
    > = crate::Field::new("customSwimlanes");

    pub const DEFAULT_CUSTOM_SWIMLANE_NAME: crate::Field<Self, crate::Text> =
        crate::Field::new("defaultCustomSwimlaneName");

    pub const SWIMLANE_STRATEGY: crate::Field<Self, crate::Text> =
        crate::Field::new("swimlaneStrategy");
}
