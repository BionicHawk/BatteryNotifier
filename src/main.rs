use std::thread::sleep;

use components::{battery::get_battery_status, notifications::{show_almost_charged, show_fully_charged, show_initialized_service}};
use systemstat::Duration;

mod components;

static MS_SLEEP: u64 = 300;

fn main() {
    let mut shown_almost = false;
    let mut shown_full = false;

    show_initialized_service();

    loop {
        sleep(Duration::from_millis(MS_SLEEP));
        let battery_status_option = get_battery_status();

        if let Some(battery_status) = battery_status_option {
            
            print!("\rworking...");

            if battery_status.is_charging {

                print!("\rCharging... battery = {}%", battery_status.percent);

                if battery_status.percent >= 95.0 && !shown_almost {
                    show_almost_charged();
                    shown_almost = true;
                }

                if battery_status.percent >= 100.0 && !shown_full {
                    show_fully_charged();
                    shown_full = true;
                }

            }

            continue;
        }

        shown_almost = false;
        shown_full = false;

    }
    
}
