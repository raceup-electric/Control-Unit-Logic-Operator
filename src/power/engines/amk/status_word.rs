/*
 * struct motorValues1 {
 *    bool AMK_bSystemReady;      //System ready(SBM)
 *    bool AMK_bError;            //Error
 *    bool AMK_bWarn;             //Warning
 *    bool AMK_bQuitDcOn;         //HVactivation acknowledgment
 *    bool AMK_bDcOn;             //HVactivation level
 *    bool AMK_bQuitInverterOn;   // RF Controller enable acknowledgment
 *    bool AMK_bInverterOn;       //Controller enable level
 *    bool AMK_bDerating;         //Derating (torque limitation active)
 * 
 */

#[allow(non_camel_case_types)]
pub enum WorkFields {
    AMK_bSystemReady,  
    AMK_bError,            
    AMK_bWarn,         
    AMK_bQuitDcOn,     
    AMK_bDcOn,         
    AMK_bQuitInverterOn,
    AMK_bInverterOn,      
    AMK_bDerating,     
}

pub struct AmkStatusWord{
    word: u8,
}

#[allow(non_snake_case)]
impl AmkStatusWord {
    fn new() -> Self {
        Self{word: 0}
    }

    fn from_word(word: u8) -> Self{
        Self{word}
    }

    fn get_word(&self) -> u8{
        self.word
    }

    fn update_status(&mut self, field: WorkFields,  val: bool){
        match field{
            WorkFields::AMK_bSystemReady => self.update_bit(0, val),
            WorkFields::AMK_bError => self.update_bit(1, val),
            WorkFields::AMK_bWarn => self.update_bit(2, val),
            WorkFields::AMK_bQuitDcOn =>self.update_bit(3, val) ,
            WorkFields::AMK_bDcOn => self.update_bit(4, val),
            WorkFields::AMK_bQuitInverterOn => self.update_bit(5, val),
            WorkFields::AMK_bInverterOn => self.update_bit(6, val),
            WorkFields::AMK_bDerating => self.update_bit(7, val),
        }
    }

    fn check_status(&self, field: WorkFields) -> bool{
        match field{
            WorkFields::AMK_bSystemReady => self.check_bit(0),
            WorkFields::AMK_bError => self.check_bit(1),
            WorkFields::AMK_bWarn => self.check_bit(2),
            WorkFields::AMK_bQuitDcOn => self.check_bit(3),
            WorkFields::AMK_bDcOn => self.check_bit(4),
            WorkFields::AMK_bQuitInverterOn =>self.check_bit(5), 
            WorkFields::AMK_bInverterOn => self.check_bit(6),
            WorkFields::AMK_bDerating => self.check_bit(7),
        }
    }

    //private
    fn update_bit(&mut self,bit : u8, active: bool){
        let bit = match active{
            true => 1 << bit,
            false => !(1 << bit),
        };

        match active{
            true => self.word |= bit,
            false => self.word &= bit,
        }
    }

    fn check_bit(&self,bit: usize) -> bool{
        let bit = 1 << bit;
        (self.word & bit) == 1
    }
}
