/*
 * Author: Alberto Damo
 * Date: 14/11/2024
 */

pub enum RaceModality {
    Driver,
    EmbeddedSystemAcceleration,
    EmbeddedSystemEndurance,
}

fn setup_driver() -> Result<(),()>
{
    todo!()
}

fn setup_embedded_system_acceleration() -> Result<(),()>
{
    todo!()
}

fn setup_embedded_system_endurance() -> Result<(),()>
{
    todo!()
}

pub fn choose_modality(mode: RaceModality) -> Result<(),()>{
    match mode{
        RaceModality::Driver => setup_driver(),
        RaceModality::EmbeddedSystemAcceleration => setup_embedded_system_acceleration(),
        RaceModality::EmbeddedSystemEndurance => setup_embedded_system_endurance(),
    }
}
