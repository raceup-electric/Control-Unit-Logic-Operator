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


pub struct AmkEngine{
    status_word: status_word::AmkStatusWord,
    temps: temperatures::AmkTemperatures,
    amk_error_info_diagnostic_number: u16,
    amk_actual_velocity: u16,
    amk_torque_current: u16,
    amk_magnetizing_current: u16,
}

impl AmkEngine {
    pub fn new() -> Self {
        Self{
            status_word: todo!(),
            temps: todo!(),
            amk_error_info_diagnostic_number: todo!(),
            amk_actual_velocity: todo!(),
            amk_torque_current: todo!(),
            amk_magnetizing_current: todo!(),
        }
    }
    
}
