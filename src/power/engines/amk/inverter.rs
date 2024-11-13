use super::engine::AmkEngine;

const NUMOFENGINE: usize = 4;

pub enum HvStatus{
    Enable,
    Disable,
}

pub struct InverterAmk{
    engine_fl: AmkEngine,
    engine_fr: AmkEngine,
    engine_rl: AmkEngine,
    engine_rr: AmkEngine,
}

impl InverterAmk {
    fn new() -> Self {
        Self{ 
            engine_fl: AmkEngine::new(), 
            engine_fr: AmkEngine::new(), 
            engine_rl: AmkEngine::new(), 
            engine_rr: AmkEngine::new(), 
        }
    }

    //TODO: define the new logic to check if the HV is active or not
    fn check_hv(&self) -> HvStatus{
        todo!()
    }
}
