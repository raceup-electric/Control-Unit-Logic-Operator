/*
 * Author: Alberto Damo
 * Date: 14/11/2024
 *
 * This module contains the implementation of the cooling system.
 * It controls the pumps and tha fans
 *
 * It uses a general component to easily implement pumps and fans
 */

pub mod fans;
pub mod pumps;

type Power = u8;
type TimeStep = u8;

struct Component<'a>{
    active: bool,
    power_on_curve: &'a [(Power,TimeStep)], //[0,100]
}

impl<'a> Component<'a>{
    fn new(init_seq: &'a[(Power,TimeStep)]) -> Self {
        Self{active: false,power_on_curve:init_seq}
    }

    fn start(&mut self)
    {
        todo!()
    }

    fn stop(&mut self)
    {
        todo!()
    }
}

