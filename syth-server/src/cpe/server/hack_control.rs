use syth_config::Config;

pub fn hack_control_packet(config: &Config) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(7);
    buf.push(0x20);
    buf.push(config.cpe.hack_control_flying);
    buf.push(config.cpe.hack_control_noclip);
    buf.push(config.cpe.hack_control_speeding);
    buf.push(config.cpe.hack_control_spawn_control);
    buf.push(config.cpe.hack_control_third_person_view);
    buf.push(config.cpe.hack_control_jump_height.try_into().unwrap());
    buf
}
