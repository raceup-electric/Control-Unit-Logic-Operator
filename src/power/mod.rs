mod engines;
mod maps;
mod settings;

pub struct PowerControl;

impl PowerControl {
    pub fn init_ready_to_drive(&mut self);

    fn check_hv(&self) -> bool;
    fn check_rf(&self) -> bool;
}
