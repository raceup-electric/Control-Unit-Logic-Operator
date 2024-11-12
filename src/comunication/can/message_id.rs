use bw_r_drivers_tc37x as drivers;
use drivers::can::msg;

#[derive(Copy,Clone)]
pub struct MessageId{
    id: u16,
}

impl Into<drivers::can::MessageId> for MessageId{
    fn into(self) -> drivers::can::MessageId {
        drivers::can::MessageId{ 
            data: self.id.into(),
            length: msg::MessageIdLength::Standard,
        }
    }
}

impl MessageId{
    pub fn new(id: u16) -> Self {
        Self{id}
    }
}
