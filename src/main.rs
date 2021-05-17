pub struct Battery {
    current_voltage: f64,
    current_charge: f64, // in Ah
    capacity: f64,
    capacity_amp_hrs: f64,
    current_source_amps: f64,
    current_sink_amps: f64,
    is_charging: bool
}

impl Battery {
    // Updating
    fn update_voltage(&mut self) {
        // Linear voltage calculation
        // curr_sink_amps - Current flowing into the battery (recharging)
        // curr_source_amps - Current flowing out of the battery (discharging)
        if(self.current_charge > 0.0) {
            self.current_charge = self.current_charge - (self.current_source_amps - self.current_sink_amps); // Current charge = Current charge - (current outgoing amps - incoming amps); if outgoing amps > incoming amps its losing charge
            self.current_voltage = self.capacity * (self.current_charge / self.capacity_amp_hrs);
        } else { self.current_charge = 0.0; self.current_voltage = 0.0; }
    }

    fn update_load(&mut self, load: f64) { self.current_source_amps = load; }

    fn charge(&mut self) {
        if self.current_charge <  self.capacity {
            self.is_charging = true;
            self.current_sink_amps = 1.0;
        } else { self.is_charging = false; println!("Battery does not need to be charged at this moment. "); }
    }

    // Receiving
    fn get_soc(&self) -> f64 {
        return ((self.current_charge/self.capacity) * 100.0).round();
    }
}

fn main() {
    let bat_1_load: f64 = 0.0;
    let bat_2_load: f64 = 1.0;

    let mut bat_1 = Battery{current_voltage: 0.0, current_charge: 6.5, capacity: 6.5, capacity_amp_hrs: 6.25, current_source_amps: 0.0, current_sink_amps: 0.0, is_charging: false};
    let mut bat_2 = Battery{current_voltage: 0.0, current_charge: 6.5, capacity: 6.5, capacity_amp_hrs: 6.15, current_source_amps: 0.0, current_sink_amps: 0.0, is_charging: false};


    loop { std::thread::sleep(std::time::Duration::from_millis(1000)); // Simulating 1 frame every x ms
        // Battery 1 calculations:
        println!("----BAT1----");
        bat_1.update_voltage();
        bat_1.update_load(bat_1_load);
        println!("VOLTAGE: {}", bat_1.current_voltage);
        println!("LOAD: {} Amps", bat_1.current_source_amps);
        println!("CHARRGE: {}%", bat_1.get_soc());

        // Battery 2 calculations
        bat_2.update_voltage();
        bat_2.update_load(bat_2_load);
        println!("----BAT2----");
        println!("VOLTAGE: {}",bat_2.current_voltage);
        println!("LOAD: {} Amps", bat_2.current_source_amps);
        println!("CHARRGE: {}% (CHARGING: {})", bat_2.get_soc(), bat_2.current_sink_amps);


        print!("{}[2J", 27 as char); // Clear console for debug purposes.

    }
}
