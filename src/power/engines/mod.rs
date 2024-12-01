/*
 * Author: Alberto Damo
 * Date: 14/11/2024
 *
 * This is a general interface to interact with the engines and the inverter in the car
 * If you want to change the engines create a new module and implement the trait below
 * (PowerControl).
 *
 * If you are still using the AMK engines (I hope not) use that as an example
 */

pub mod amk;

use crate::comunication::can::can_obj::{CanObj,ErrorTransmit};

pub enum InverterEngineStatus{
    HvOff,
    HvOn,
    StartPrecharge,
    CompletePrecharge,
    RfOk,
}

pub enum Engines {
    FrontRight,
    FrontLeft,
    RearRight,
    RearLeft,
}

pub trait PowerControl<'a> {
    type EngineInternalState;
    type EngineUpdateMex;

    fn new(can_obj: &'a CanObj) -> Self;

    //TODO: define the new logic to check the status of the HV,RF,Precharge,RF 
    //and the switches between these states
    fn check_status(&self) -> InverterEngineStatus;
    /*
     * throttle: gas pedal [0-100] in %
     */
    fn accelerate(&self, throttle: u8);

    /*
     * regen_brake: gas pedal [0-100] in %
     */
    fn regen_brake(&self, regen_brake: u8);

    fn update_data_engine(&mut self, mex: &Self::EngineInternalState, engine: Engines);

    fn send_update_engine_mex(&self, mex: &Self::EngineUpdateMex, engine: Engines) -> 
        Result<(), ErrorTransmit>;
}
