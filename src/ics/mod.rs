use integrity_check_system::ics::{ICS,ICSTemplate};
use integrity_check_system::err_map::bst::Bst;
use integrity_check_system::ics_trait::internal::InternalCheck;
use integrity_check_system::ics_trait::external::ICSDep;
use crate::comunication::can::*;


const ERRVECSIZE : usize = 7;

type TID = u16;

pub struct IcsCan<'a>{
    ics_base: ICS<'a,Bst,ERRVECSIZE,TID>,
    can_node: &'a can_obj::CanObj,
}

#[allow(unused)]
impl<'a> IcsCan<'a> {
    pub fn with_capacity(can_node: &'a can_obj::CanObj,int_err_cap: usize, ext_err_cap: usize, error_cap: usize, id: TID) -> Self
    {
        Self{
            ics_base: ICS::with_capacity(int_err_cap, ext_err_cap, error_cap, id),
            can_node
        }
    }

    pub fn new(id: u16,can_node: &'a can_obj::CanObj) -> Self {
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
            payload[0] = mex.part();
            let mut i =1;
            for err in mex.iter_data(){
                payload[i] = *err;
                i+=1;
            }
            let frame = frame::Frame::new(id, &payload);
            let _ = self.can_node.transmit(&frame);
        }
    }
}
