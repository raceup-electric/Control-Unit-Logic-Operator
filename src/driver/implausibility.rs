use crate::utils::bit_manipulation::BitOps;

pub enum ImplausibilityType {
    ThrottleBrake,
    ThrottlePaddle,
    ThrottlePotentiometer,
}

type Word = u8;

pub struct DriverImplausibility{
    imp_buffer: Word,
}

impl DriverImplausibility {
    pub fn new() -> Self {
        Self{imp_buffer: 0}
    }

    pub fn word(&self) -> Word{
        self.imp_buffer
    }

    pub fn check_imp(&self, imp: ImplausibilityType) -> bool{
        match imp{
            ImplausibilityType::ThrottleBrake => self.imp_buffer.check_bit(0),
            ImplausibilityType::ThrottlePaddle => self.imp_buffer.check_bit(1),
            ImplausibilityType::ThrottlePotentiometer => self.imp_buffer.check_bit(2),
        }
    }

    pub fn update_imp(&self, imp: ImplausibilityType, val: bool) -> bool{
        match imp{
            ImplausibilityType::ThrottleBrake => self.imp_buffer.update_bit(bit, val),
            ImplausibilityType::ThrottlePaddle => self.imp_buffer.update_bit(1,val),
            ImplausibilityType::ThrottlePotentiometer => self.imp_buffer.update_bit(2,val),
        }
    }
    
}
