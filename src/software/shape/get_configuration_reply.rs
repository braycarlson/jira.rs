#[derive(Clone, Copy, Debug)]
pub struct GetConfigurationReply;

impl GetConfigurationReply {
    pub const COLUMN_CONFIG: crate::Field<
        Self,
        crate::Object<super::GetConfigurationReplyColumnConfig>,
    > = crate::Field::new("columnConfig");

    pub const ESTIMATION: crate::Field<
        Self,
        crate::Object<super::GetConfigurationReplyEstimation>,
    > = crate::Field::new("estimation");

    pub const FILTER: crate::Field<Self, crate::Object<super::GetConfigurationReplyFilter>> =
        crate::Field::new("filter");

    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const LOCATION: crate::Field<Self, crate::Object<super::GetConfigurationReplyLocation>> =
        crate::Field::new("location");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const RANKING: crate::Field<Self, crate::Object<super::GetConfigurationReplyRanking>> =
        crate::Field::new("ranking");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const SUB_QUERY: crate::Field<Self, crate::Object<super::GetConfigurationReplySubQuery>> =
        crate::Field::new("subQuery");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
