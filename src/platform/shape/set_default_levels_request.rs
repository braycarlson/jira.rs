#[derive(Clone, Copy, Debug)]
pub struct SetDefaultLevelsRequest;

impl SetDefaultLevelsRequest {
    pub const DEFAULT_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::DefaultLevelValue>>,
    > = crate::Field::new("defaultValues");
}
