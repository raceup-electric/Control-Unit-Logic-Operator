use super::message_id::MessageId;


#[derive(Copy,Clone)]
pub struct Frame<'a>{
    id: MessageId,
    data: &'a[u8],
}

impl<'a> Frame<'a> {
    pub fn new (id: u16,data: &'a[u8]) -> Self {
        Self{
            id: MessageId::new(id),
            data
        }
    }
    
}

impl<'a> Into<bw_r_drivers_tc37x::can::Frame<'a>> for Frame<'a>{
    fn into(self) -> bw_r_drivers_tc37x::can::Frame<'a> {
        bw_r_drivers_tc37x::can::Frame{ 
            id: self.id.into(), 
            data: self.data
        }
    }

    // add code here
}
