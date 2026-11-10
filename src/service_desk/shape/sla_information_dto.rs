#[derive(Clone, Copy, Debug)]
pub struct SlaInformationDTO;

impl SlaInformationDTO {
    pub const COMPLETED_CYCLES: crate::Field<
        Self,
        crate::List<crate::Object<super::SlaInformationCompletedCycleDTO>>,
    > = crate::Field::new("completedCycles");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ONGOING_CYCLE: crate::Field<
        Self,
        crate::Object<super::SlaInformationOngoingCycleDTO>,
    > = crate::Field::new("ongoingCycle");

    pub const SLA_DISPLAY_FORMAT: crate::Field<Self, crate::Text> =
        crate::Field::new("slaDisplayFormat");
}
