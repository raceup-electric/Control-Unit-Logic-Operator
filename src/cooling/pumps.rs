use super::*;

type Pump = Component;

pub struct PumpList{
    left: Pump,
    right: Pump,
}

impl  PumpList{
    fn new() -> Self{
        let init_seq_fan = [];
        PumpList{
            left: Pump::new(&init_seq_fan),
            right: Pump::new(&init_seq_fan),
        }
    }
    
}
