pub enum RaceModality {
    Driver,
    EmbeddedSystemAcceleration,
    EmbeddedSystemEndurance,
}

fn setup_driver() -> Result<(),()>;
fn setup_embedded_system_acceleration() -> Result<(),()>;
fn setup_embedded_system_endurance() -> Result<(),()>;

pub fn choose_modality(mode: RaceModality) -> Result<(),()>{
    match mode{
        RaceModality::Driver => setup_driver(),
        RaceModality::EmbeddedSystemAcceleration => setup_embedded_system_acceleration(),
        RaceModality::EmbeddedSystemEndurance => setup_embedded_system_endurance(),
    }
}
