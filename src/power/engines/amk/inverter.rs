use super::engine::AmkEngine;

const NUMOFENGINE: usize = 4;

pub enum HvStatus{
    Enable,
    Disable,
}

pub struct InverterAmk{
    engine_fl: AmkEngine,
    engine_fr: AmkEngine,
    engine_bl: AmkEngine,
    engine_br: AmkEngine,
}

impl InverterAmk {
    fn new() -> Self {
        Self{ 
            engine_fl: AmkEngine::new(), 
            engine_fr: AmkEngine::new(), 
            engine_bl: AmkEngine::new(), 
            engine_br: AmkEngine::new(), 
        }
    }

    //TODO: define the new logic to check if the HV is active or not
    fn check_hv(&self) -> HvStatus{
        todo!()
    }
}
