/*
 * Author: Alberto Damo
 * Date: 14/11/2024
 *
 * This module is based on the datasheet of AMK engines. Check for more info.
 * Is is the pdf in this directory (./amk_datasheet.pdf). 
 * The info about the Can messages starts at page: 60
 */

use crate::utils::bit_manipulation::BitOps;

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

#[derive(Copy,Clone)]
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
        let word = &mut self.word;
        match field{
            WorkFields::AMK_bSystemReady => *word = word.update_bit(0, val).unwrap(),
            WorkFields::AMK_bError => *word = word.update_bit(1, val).unwrap(),
            WorkFields::AMK_bWarn => *word = word.update_bit(2, val).unwrap(),
            WorkFields::AMK_bQuitDcOn => *word = word.update_bit(3, val).unwrap(),
            WorkFields::AMK_bDcOn => *word = word.update_bit(4, val).unwrap(),
            WorkFields::AMK_bQuitInverterOn => *word = word.update_bit(5, val).unwrap(),
            WorkFields::AMK_bInverterOn => *word = word.update_bit(6, val).unwrap(),
            WorkFields::AMK_bDerating =>*word = word.update_bit(7, val).unwrap(),
        }
    }

    fn check_status(&self, field: WorkFields) -> bool{
        let word = &self.word;
        match field{
            WorkFields::AMK_bSystemReady => word.check_bit(0).unwrap(),
            WorkFields::AMK_bError =>word.check_bit(1).unwrap(),
            WorkFields::AMK_bWarn => word.check_bit(2).unwrap(),
            WorkFields::AMK_bQuitDcOn =>word.check_bit(3).unwrap(),
            WorkFields::AMK_bDcOn => word.check_bit(4).unwrap(),
            WorkFields::AMK_bQuitInverterOn =>word.check_bit(5).unwrap(), 
            WorkFields::AMK_bInverterOn => word.check_bit(6).unwrap(),
            WorkFields::AMK_bDerating => word.check_bit(7).unwrap(),
        }
    }
}
