pub enum TempCategory{
    Motor,
    Inverter,
    Igbt
}

pub struct AmkTemperatures{
    amk_temp_motor: u16,
    amk_temp_inverter: u16,
    amk_temp_igbt: u16,
}

impl AmkTemperatures {
    fn new() -> Self{
        Self{ 
            amk_temp_motor: 0, 
            amk_temp_inverter: 0, 
            amk_temp_igbt: 0, 
        }
    }

    fn update_temp(&mut self,temp_category: TempCategory,temp: u16){
        match temp_category{
            TempCategory::Motor => self.amk_temp_motor = temp,
            TempCategory::Inverter => self.amk_temp_inverter = temp,
            TempCategory::Igbt => self.amk_temp_igbt = temp,
        }
    }

    fn fetch_temp(&self,temp_category: TempCategory) -> u16{
        match temp_category{
            TempCategory::Motor => self.amk_temp_motor,
            TempCategory::Inverter => self.amk_temp_inverter,
            TempCategory::Igbt => self.amk_temp_igbt,
        }
    }
}