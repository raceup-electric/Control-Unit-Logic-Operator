pub mod fans;
pub mod pumps;

type Power = u8;
type TimeStep = u8;

struct Component{
    active: bool,
    power_on_curve: &[(Power,TimeStep)], //[0,100]
}

impl Component{
    fn new(init_seq: &[u8]) -> Self {
        Self{active: false,power_on_curve:init_seq}
    }

    fn start(&mut self);
    fn stop(&mut self);
}

