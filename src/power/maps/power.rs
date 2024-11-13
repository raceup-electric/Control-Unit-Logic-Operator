use super::PresetMap;

pub struct PowerPreset{
    max_wat: usize,
    pos_torque: u8,
}

pub type PowerMaps<const N:usize> = super::PresetMap<PowerPreset,N>;
