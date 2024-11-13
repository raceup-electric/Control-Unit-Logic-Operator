pub struct PowerSettings{
    pub power_limit: u8,
    pub max_speed: u8,
    pub max_pos_torque: u8,
    pub max_neg_torque: u8,
    pub front_motor_repartition: u8,
    pub rear_motor_repartition: u8,
    pub regen_current_scale: u8,
    pub max_regen_current: u8,
    pub torque_vectoring: bool,
}
