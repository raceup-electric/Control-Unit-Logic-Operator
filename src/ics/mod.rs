use bw_r_drivers_tc37x::can::Node;
use bw_r_drivers_tc37x::pac::can0::{Can0, N as Can0Node};
use bw_r_drivers_tc37x::can::*;
use integrity_check_system::ics::{ICS,ICSTemplate};
use integrity_check_system::err_map::bst::Bst;
use integrity_check_system::ics_trait::internal::InternalCheck;
use integrity_check_system::ics_trait::external::ICSDep;


const ERRVECSIZE : usize = 7;

pub type CanNode = Node<Can0Node,Can0,Node0,Configured>;
type TID = u16;

pub struct IcsCan<'a>{
    ics_base: ICS<'a,Bst,ERRVECSIZE,TID>,
    can_node: &'a CanNode,
}

impl<'a> IcsCan<'a> {
    pub fn new(id: u16,can_node: &'a CanNode) -> Self {
        let ics = ICS::new(id).unwrap();
        Self{ics_base:ics,can_node}
    }

    pub fn add_dependency(&mut self,dep: ICSDep<'a,ERRVECSIZE,TID>,err_index: usize)
        -> Result<(), (usize, &str)> {
        self.ics_base.add_external_check(dep, err_index)
    }

    pub fn add_check(&mut self, check: InternalCheck<'a> ,err_index: usize) 
        -> Result<(), (usize, &str)>{
        self.ics_base.add_internal_check(check, err_index)
    }

    pub fn run_check(&mut self){
        self.ics_base.internal_check();
        let mexs = self.ics_base.create_ics_messages::<u8>();

        for mex in mexs.iter(){
            let mut payload = [0_u8;8];
            let id = mex.id();
            let id = MessageId{ data: id.into(), length: msg::MessageIdLength::Standard  };
            payload[0] = mex.part();
            let mut i =1;
            for err in mex.iter_data(){
                payload[i] = *err;
                i+=1;
            }
            let frame = Frame { id, data: &payload};
            let _ = self.can_node.transmit(&frame);
        }
    }
}
