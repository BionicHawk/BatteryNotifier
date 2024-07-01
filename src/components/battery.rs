use systemstat::Platform;


pub struct BatteryStatus {
    pub percent: f32,
    pub is_charging: bool
}

pub fn get_battery_status() -> Option<BatteryStatus> {
    let sys = systemstat::System::new();
    let b_life_res = sys.battery_life();
    let on_ac_res = sys.on_ac_power();

    if let Ok(b_life) = b_life_res {
        if let Ok(is_ac) = on_ac_res {

            return Some(BatteryStatus {
                 percent: b_life.remaining_capacity * 100.0,
                 is_charging: is_ac 
                })

        }
    }

    return None;

}