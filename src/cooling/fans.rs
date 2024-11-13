use super::*;

type Fan = Component;

pub struct FanList{
    left: Fan,
    right: Fan,
}

impl  FanList{
    fn new() -> Self{
        let init_seq_fan = [];
        FanList{
            left: Fan::new(&init_seq_fan),
            right: Fan::new(&init_seq_fan),
        }
    }
    
}
