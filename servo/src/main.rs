use rppal::gpio::{Gpio};

use std::error::Error;
use std::time::{Duration};
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    // initialize the pin for the servo.
    let mut servo_pin = Gpio::new()?.get(18)?.into_output();
    
    for n in 1..5 {
        // setup the servo at 0°.
        // 0° = 500 µs
        servo_pin.set_pwm(Duration::from_millis(20), Duration::from_micros(500));
        // wait 500 ms.
        thread::sleep(Duration::from_millis(500));
        // setup the servo at 180°.
        // 180° = 2300 µs
        servo_pin.set_pwm(Duration::from_millis(20), Duration::from_micros(2300));
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
