use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::*;
use esp_idf_sys as _; // Link the ESP-IDF

fn main() -> Result<(), anyhow::Error> {
    // Initialize the ESP32 peripherals
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // Configure GPIO pins for LEDs
    let mut red_led = PinDriver::output(pins.gpio2)?;
    let mut yellow_led = PinDriver::output(pins.gpio4)?;
    let mut green_led = PinDriver::output(pins.gpio16)?;

    loop {
        // Red LED on, others off
        red_led.set_high()?;
        yellow_led.set_low()?;
        green_led.set_low()?;
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Yellow LED on, others off
        red_led.set_low()?;
        yellow_led.set_high()?;
        green_led.set_low()?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Green LED on, others off
        red_led.set_low()?;
        yellow_led.set_low()?;
        green_led.set_high()?;
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Yellow LED on, others off (transition to red)
        red_led.set_low()?;
        yellow_led.set_high()?;
        green_led.set_low()?;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
