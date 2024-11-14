use crate::utils::bit_manipulation::BitOps;

use super::status_word;
use super::temperatures;

/*
 *    float AMK_ActualVelocity;       //Signed - Unit: rpm - Actual speed value
 *    float AMK_TorqueCurrent;        //Signed - Raw data for calculating 'actual torque current'Iq See 'Units'on page 61
 *    Uint16 AMK_Voltage;   //unSigned - Raw data for calculating 'actual dc_bus voltage
 *    float AMK_Current;  // see PDK
 *} ;
 * 
 * struct motorValues2 {
 *    float AMK_TempMotor;                //Signed - Unit: 0.1 �C - Motor temperature
 *    float AMK_TempInverter;             //Signed - Unit: 0.1 �C - Cold plate temperature
 *    float AMK_TempIGBT;                 //Signed - Unit: 0.1 �C - IGBTtemperature
 *    unsigned int AMK_ErrorInfo;         //Unsigned - Diagnostic number
 *};
 */

pub enum InternalValues {
    AmkActualValues1(AmkActualValues1),
    AmkActualValues2(AmkActualValues2),
}

#[allow(non_snake_case)]
pub struct Update{
    AMK_bInverterOn: bool,
    AMK_bDcOn:bool,
    AMK_bEnable:bool,
    AMK_bErrorReset:bool,
    target_velocity: i16,
    pos_torque: i16,
    neg_torque: i16,

}

#[derive(Copy,Clone)]
pub struct AmkActualValues1{
    status_word: status_word::AmkStatusWord,
    amk_actual_velocity: i16, //rpm
    amk_torque_current: i16,
    amk_magnetizing_current: i16,
    timestamp: u16,
}

#[derive(Copy,Clone)]
pub struct AmkActualValues2{
    temps: temperatures::AmkTemperatures,
    amk_error_info_diagnostic_number: u16,
    timestamp: u16,
}

pub struct AmkEngine{
    mex_1: AmkActualValues1,
    mex_2: AmkActualValues2,
    can_id: u16,
}

impl AmkEngine {
    pub fn new(can_id: u16) -> Self {
        Self{
            mex_1: AmkActualValues1{
                status_word: todo!(),
                amk_actual_velocity: todo!(),
                amk_torque_current: todo!(),
                amk_magnetizing_current: todo!(),
                timestamp: 0,
            },
            mex_2: AmkActualValues2{
                temps: todo!(),
                amk_error_info_diagnostic_number: todo!(),
                timestamp: 0,
            },
            can_id
        }
    }

    pub fn recv_mex_inverter(&mut self,mex: &InternalValues) {
        match mex{
            InternalValues::AmkActualValues1(m) => self.mex_1 = *m,
            InternalValues::AmkActualValues2(m) => self.mex_2 = *m,
        }
    }

    pub fn id(&self) -> u16 {
        self.can_id
    }

    pub fn create_update_mex(&self,update: &Update ) -> [u8;8]{
        let mut control_word = 0_u16;
        control_word = control_word.update_bit(8, update.AMK_bInverterOn).unwrap();
        control_word = control_word.update_bit(9, update.AMK_bDcOn).unwrap();
        control_word = control_word.update_bit(10, update.AMK_bEnable).unwrap();
        control_word = control_word.update_bit(12, update.AMK_bErrorReset).unwrap();

        let cw = control_word.to_le_bytes();
        let tv = update.target_velocity.to_le_bytes();
        let pt = update.pos_torque.to_le_bytes();
        let nt = update.neg_torque.to_le_bytes();

        let mut res = [0;8];
        let mut i =0;
        for byte in cw.into_iter().chain(tv).chain(pt).chain(nt){
            res[i] = byte;
        }

        res
    }
}
