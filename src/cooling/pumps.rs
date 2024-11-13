use super::*;

const INIT_SEQ_PUMP : [(Power,TimeStep);2] = [(0,0),(0,0)];

type Pump<'a> = Component<'a>;

pub struct PumpList<'a>{
    left: Pump<'a>,
    right: Pump<'a>,
}

impl<'a> PumpList<'a>{
    fn new() -> Self{
        PumpList{
            left: Pump::new(&INIT_SEQ_PUMP),
            right: Pump::new(&INIT_SEQ_PUMP),
        }
    }
    
}
