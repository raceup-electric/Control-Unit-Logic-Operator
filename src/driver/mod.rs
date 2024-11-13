pub mod inputs;
pub mod implausibility;

pub struct DriverInfo{
    pub data: inputs::DriverInput,
    pub imps: implausibility::DriverImplausibility,
}
