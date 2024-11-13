use super::PresetMap;

pub struct RepartitionPreset{
    tv: bool,
    front: u8,
    rear: u8,
}

pub type RepartitionMaps<const N:usize> = super::PresetMap<RepartitionPreset,N>;
