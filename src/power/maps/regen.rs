use super::PresetMap;

pub struct RegenPreset{
    max_a: usize,
    neg_torque: u8,
}

pub type RegenMaps<const N:usize> = super::PresetMap<RegenPreset,N>;
