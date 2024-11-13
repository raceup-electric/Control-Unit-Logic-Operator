pub mod power;
pub mod regen;
pub mod reparitions;

struct PresetMap<P,const N: usize> {
    presets: [Option<P>;N],
}

impl<P,const N:usize> PresetMap<P,N> {
    pub fn get_preset(&self, i:usize) -> Result<Option<&P>,()> {
        if i < N{
            return match &self.presets[i]{
                Some(p) => Ok(Some(p)),
                None => Ok(None)
            }
        }
        Err(())
    }

    pub fn set_preset(&mut self, preset: P, preset_num: usize) -> Result<(),()>{
        if preset_num < N{
            self.presets[preset_num] = Some(preset);
            return Ok(());
        }
        Err(())
    }
    
}
