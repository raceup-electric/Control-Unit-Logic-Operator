use super::*;

const INIT_SEQ_FAN : [(Power,TimeStep);2] = [(0,0),(0,0)];

type Fan<'a> = Component<'a>;

pub struct FanList<'a>{
    left: Fan<'a>,
    right: Fan<'a>,
}

impl<'a> FanList<'a>{
    fn new() -> Self{
        FanList{
            left: Fan::new(&INIT_SEQ_FAN),
            right: Fan::new(&INIT_SEQ_FAN),
        }
    }
    
}
