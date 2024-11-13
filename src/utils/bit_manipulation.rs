use bw_r_drivers_tc37x::gpio::Output;

pub trait BitOps
where Self: 
{
    fn update_bit(&self ,bit : Self, active: bool) -> Result<Self,()> where Self: Sized;
    fn check_bit(&self,bit: Self) -> Result<bool,()>;
}

impl BitOps for u8{
    fn update_bit(&self ,bit : Self, active: bool) -> Result<Self,()> 
        where Self: Sized
    {
        if *self >= 8{
           return Err(())
        }

        let bit = match active{
            true => 1 << bit,
            false => (1 << bit) ^ 0xFF,
        };
        Ok(match active{
            true => *self | bit,
            false => *self & bit,
        })
    }

    fn check_bit(&self,bit: Self) -> Result<bool,()>
    {
        if *self >= 8{
            return Err(());
        }
        let bit : Self = 1 << bit;
        Ok((*self & bit) == 1)

    }
}
