pub enum DriverInputPedals{
    Throttle,
    Brake,
    Paddle,
}

type WordPedals = u8;
type WordSteeringWheel = i16;

pub struct DriverInput{
    throttle: WordPedals,                   //[0,100]
    brake: WordPedals,                      //[0,100]
    paddle: WordPedals,                     //[0,100]
    steering_wheel: WordSteeringWheel,      //[-180,180]
}

impl DriverInput{
    pub fn new() -> Self {
        Self{ 
            throttle: 0,
            brake: 0,
            paddle: 0,
            steering_wheel: 0,
        }
    }

    pub fn fetch_driver_input_depals(&self, input: DriverInputPedals) -> WordPedals{
        match input{
            DriverInputPedals::Throttle => self.throttle,
            DriverInputPedals::Brake => self.brake,
            DriverInputPedals::Paddle => self.paddle,
        }
    }

    pub fn update_driver_input_depals(&mut self, input: DriverInputPedals, val: WordPedals) -> Result<(),()>{
        if val > 180{
            return Err(());
        }

        match input{
            DriverInputPedals::Throttle => self.throttle = val,
            DriverInputPedals::Brake => self.brake = val,
            DriverInputPedals::Paddle => self.paddle = val,
        };
        Ok(())
    }

    pub fn fetch_driver_input_steering_wheel(&self) -> WordSteeringWheel{
        self.steering_wheel
    }

    pub fn update_driver_input_steering_wheel(&mut self, val: WordSteeringWheel) -> Result<(),()>{
        if val < 180 || val > 180{
            return Err(());
        }
        self.steering_wheel = val;
        Ok(())
    }
}
