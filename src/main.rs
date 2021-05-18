const DELTA_TIME: f64 = 10.0;

pub struct Battery {
    current_voltage: f64,
    current_charge: f64, // in Ah
    capacity: f64,
    capacity_amp_hrs: f64,
    current_source_amps: f64,
    current_sink_amps: f64,
    exponential_voltage: f64, // Volts
    nominal: f64, // Volts
    is_charging: bool
}

impl Battery {
    // Updating
    fn update_voltage(&mut self, time: f64) {
        // Linear voltage calculation
        // curr_sink_amps - Current flowing into the battery (recharging)
        // curr_source_amps - Current flowing out of the battery (discharging)
        /*if self.current_charge > 0.0 || self.is_charging == true {
            self.current_charge = self.current_charge - (self.current_source_amps - self.current_sink_amps) * DELTA_TIME / 3600.0; // Current charge = Current charge - (current outgoing amps - incoming amps); if outgoing amps > incoming amps its losing charge
            self.current_voltage = self.capacity * (self.current_charge / self.capacity_amp_hrs);
        } else { self.current_charge = 0.0; self.current_voltage = 0.0; }*/

        // More complex version
        // y=mx+b
        // y = slope * rate + b
        // y = slope * time * y intercept
        // slope = y2 - y1
        //        ---------
        //         x2 - x1

        // Equations are based off of this battery graph:
        // https://cdn.discordapp.com/attachments/718057425751769179/843970087009910814/unknown.png
        if time >= 0.0 && time < 1.1 {
            println!("First linear equation.");
            self.current_voltage = ((self.exponential_voltage-self.capacity)/1.0-0.0)*time+self.capacity;
        } else if time > 1.0 && time <= 5.0 {
            println!("Second linear equation.");
            self.current_voltage = ((self.nominal-self.exponential_voltage)/5.0-1.0)*time+self.exponential_voltage;
        } else {
            println!("Third linear equation.");
            self.current_voltage = ((1.0-self.nominal)/5.0-5.3)*time+self.nominal;
        }
    }
    fn update_load(&mut self, load: f64) { self.current_source_amps = load; }

    fn charge(&mut self) {
        if self.current_charge < self.capacity {
            self.is_charging = true;
            self.current_sink_amps = 2.0;
        } else { self.is_charging = false; self.current_sink_amps = 0.0; }
    }

    // Receiving
    fn get_soc(&self) -> f64 {
        return ((self.current_charge/self.capacity) * 100.0).round();
    }
}

fn main() {
    let bat_1_load: f64 = 0.05;
    let bat_2_load: f64 = 10.0;

    let mut bat_1 = Battery{ current_voltage: 0.0, current_charge: 6.5, capacity: 6.5, capacity_amp_hrs: 6.25, current_source_amps: 0.0, current_sink_amps: 0.0, exponential_voltage: 1.28, nominal: 1.18,is_charging: false };
    let mut bat_2 = Battery{ current_voltage: 0.0, current_charge: 6.5, capacity: 6.5, capacity_amp_hrs: 6.15, current_source_amps: 0.0, current_sink_amps: 0.0, exponential_voltage: 1.28, nominal: 1.18,is_charging: false };

    let mut t: f64 = 0.0;
    loop { std::thread::sleep(std::time::Duration::from_millis(DELTA_TIME as u64)); // Simulating 1 frame every x ms
        let t_hrs: f64 = t/36000000.0; // Time in ms / 3.6e+6
        println!("{} Hours", t_hrs);
        // Battery 1 calculations:
        println!("----BAT1----");
        bat_1.update_voltage(t_hrs);
        bat_1.update_load(bat_1_load);
        println!("VOLTAGE: {}",bat_1.current_voltage);
        println!("LOAD: {} Amps", bat_1.current_source_amps);
        println!("CHARGE: {}% (CHARGING: {} @ {}, Amps)", bat_1.get_soc(), bat_1.is_charging, bat_1.current_sink_amps);

        // Battery 2 calculations
        println!("----BAT2----");
        bat_2.update_voltage(t_hrs);
        bat_2.update_load(bat_2_load);
        println!("VOLTAGE: {}",bat_2.current_voltage);
        println!("LOAD: {} Amps", bat_2.current_source_amps);
        println!("CHARGE: {}% (CHARGING: {} @ {}, Amps)", bat_2.get_soc(), bat_2.is_charging, bat_2.current_sink_amps);

        print!("{}[2J", 27 as char); // Clear console for debug purposes.

        t = t + 10.0; // Time + 10 ms
    }
}
