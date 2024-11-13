use super::engine::AmkEngine;

const NUMOFENGINE: usize = 4;

pub enum HvStatus{
    Enable,
    Disable,
}

pub struct InverterAmk{
    engines: [AmkEngine;NUMOFENGINE]
}

impl InverterAmk {
    fn new() -> Self {
        let engines = [
            AmkEngine::new(),
            AmkEngine::new(),
            AmkEngine::new(),
            AmkEngine::new(),
        ];
        Self{ engines }
    }
}
