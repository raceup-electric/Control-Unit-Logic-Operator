use super::engine::{self, AmkEngine};
use crate::comunication::can::{can_obj::{self, CanObj}, frame::Frame, message_id::MessageId};

const NUMOFENGINE: usize = 4;

pub enum InverterEngineStatus{
    HvOff,
    HvOn,
    RfOk,
}

pub enum Engines {
    FrontRight,
    FrontLeft,
    RearRight,
    RearLeft,
}

pub struct InverterAmk<'a>{
    engine_fl: AmkEngine,
    engine_fr: AmkEngine,
    engine_rl: AmkEngine,
    engine_rr: AmkEngine,
    status: InverterEngineStatus,
    can_node: &'a CanObj
}

impl<'a> InverterAmk<'a>
{
    //TODO: open air to ensure that the HV is off
    //TODO: assign the correct can id to each engine
    pub fn new(can_node: &'a CanObj) -> Self {
       let res=  Self{
            engine_fl: AmkEngine::new(14), 
            engine_fr: AmkEngine::new(15), 
            engine_rl: AmkEngine::new(16), 
            engine_rr: AmkEngine::new(17), 
            status: InverterEngineStatus::HvOff,
            can_node
        };

       res
    }

    //TODO: define the new logic to check the status of the HV,RF
    pub fn check_status(&self) -> InverterEngineStatus{
        todo!()
    }

    pub fn update_data_engine(&mut self, mex: engine::InternalValues, engine: Engines){
        let engine = match engine {
            Engines::FrontRight => &mut self.engine_fr,
            Engines::FrontLeft => &mut self.engine_fl,
            Engines::RearRight => &mut self.engine_rr,
            Engines::RearLeft => &mut self.engine_rl,
        };
       engine.recv_mex_inverter(mex);
    }

    pub fn send_update_engine_mex(&self, mex: &engine::Update, engine: Engines) -> 
        Result<(), can_obj::ErrorTransmit>
    {
        let engine =match engine{
            Engines::FrontRight => &self.engine_fr,
            Engines::FrontLeft => &self.engine_fl,
            Engines::RearRight => &self.engine_rr,
            Engines::RearLeft => &self.engine_rl,
        };
        let data = engine.create_update_mex(mex);
        let id = engine.id();

        let frame = Frame::new(id, &data);

        self.can_node.transmit(&frame)
    }

}
