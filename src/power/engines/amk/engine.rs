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
    amk_magnetizing_current: u16,
    amk_actual_velocity: u16,
    amk_torque_current: u16,
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

    pub fn recv_mex_inverter(&mut self,mex: InternalValues) {
        match mex{
            InternalValues::AmkActualValues1(m) => self.mex_1 = m,
            InternalValues::AmkActualValues2(m) => self.mex_2 = m,
        }
    }

    pub fn id(&self) -> u16 {
        self.can_id
    }

    pub fn create_update_mex(&self,update: &Update ) -> [u8;8]{
        let control_word_reserved_byte = 0_u8;
        let mut control_word = 0_u8;
        control_word = control_word.update_bit(0, update.AMK_bInverterOn).unwrap();
        control_word = control_word.update_bit(1, update.AMK_bDcOn).unwrap();
        control_word = control_word.update_bit(2, update.AMK_bEnable).unwrap();
        control_word = control_word.update_bit(3, update.AMK_bErrorReset).unwrap();

        let (target_velocity_0,target_velocity_1) = split_i16_into_u8(update.target_velocity);
        let (pos_torque_lim_0,pos_torque_lim_1) = split_i16_into_u8(update.pos_torque);
        let (neg_torque_lim_0,neg_torque_lim_1) = split_i16_into_u8(update.neg_torque);

        [
            control_word_reserved_byte,control_word,
            target_velocity_0,target_velocity_1,
            pos_torque_lim_0,pos_torque_lim_1,
            neg_torque_lim_0,neg_torque_lim_1
        ]
    }
}

fn split_i16_into_u8(num: i16) -> (u8,u8){
    let num_bytes = num.to_le_bytes();
    let part_1 = u8::from_le_bytes([num_bytes[0]]);
    let part_2 = u8::from_le_bytes([num_bytes[1]]);
    (part_1,part_2)
}
