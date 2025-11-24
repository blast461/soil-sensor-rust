//! ESP32 Soil Humidity Sensor - Rust Reference Implementation
//!
//! This is a reference implementation showing Rust patterns and idioms
//! for an ESP32 soil moisture sensor built against ESP-IDF.
//! For the production-ready C++ version, see: ../soil-sensor-cpp/

use anyhow::Result;
use esp_idf_svc::log::EspLogger;
use log::{error, info};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::time::{Duration, Instant};

// Sensor configuration constants
const DRY_SOIL: u16 = 3000; // Sensor reading in completely dry soil (higher = drier)
const WET_SOIL: u16 = 1200; // Sensor reading in very wet soil (lower = wetter)
const MOISTURE_LOW: u8 = 25; // Below 25% - very dry
const MOISTURE_HIGH: u8 = 75; // Above 75% - very wet
const READING_INTERVAL_MS: u64 = 2000; // Read every 2 seconds
const CALIBRATION_MODE: bool = false; // Set to true for calibration

/// Convert raw ADC reading to moisture percentage
fn raw_to_moisture_percent(raw_value: u16) -> u8 {
    // Higher analog value = drier soil = lower moisture percentage
    let percentage = if raw_value >= DRY_SOIL {
        0
    } else if raw_value <= WET_SOIL {
        100
    } else {
        // Linear mapping: map(raw_value, DRY_SOIL, WET_SOIL, 0, 100)
        let range = DRY_SOIL - WET_SOIL;
        let offset = DRY_SOIL - raw_value;
        ((offset as u32 * 100) / range as u32) as u8
    };
    percentage.min(100)
}

/// Get soil condition description and LED state
fn get_soil_condition(moisture_percent: u8) -> (&'static str, bool) {
    if moisture_percent < MOISTURE_LOW {
        ("DRY - Need Water!", true) // LED on for dry soil
    } else if moisture_percent > MOISTURE_HIGH {
        ("WET - Too Much Water!", false) // LED off for wet soil
    } else {
        ("OPTIMAL", false) // LED off for optimal conditions
    }
}

/// Simulated soil moisture sensor for demonstration
struct MockSoilSensor {
    // Simulate sensor drift over time
    base_value: u16,
    last_reading: Instant,
}

impl MockSoilSensor {
    fn new() -> Self {
        Self {
            base_value: 2400, // Simulated sensor baseline
            last_reading: Instant::now(),
        }
    }

    /// Simulate reading from ADC with realistic sensor behavior
    fn read_averaged(&mut self, _samples: usize) -> Result<u16> {
        // Simulate time-based sensor variations
        let elapsed = self.last_reading.elapsed().as_secs();
        let mut hasher = DefaultHasher::new();
        elapsed.hash(&mut hasher);

        // Add some realistic noise and drift
        let noise = (elapsed as u16 % 200).wrapping_sub(100); // +/-100 noise
        let reading = self.base_value.wrapping_add(noise);

        self.last_reading = Instant::now();
        Ok(reading)
    }

    /// Simulate different soil conditions
    fn set_soil_condition(&mut self, condition: &str) {
        self.base_value = match condition {
            "dry" => 2800,     // Dry soil simulation
            "optimal" => 2000, // Optimal moisture
            "wet" => 1400,     // Wet soil simulation
            _ => 2400,         // Default
        };
    }
}

fn main() -> Result<()> {
    // Ensure the ESP-IDF patches and logging are set up before anything else
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    info!("========================================");
    info!("ESP32 Soil Humidity Sensor (Rust Reference)");
    info!("Board: AITRIP ESP-WROOM-32 (Simulated)");
    info!("========================================");
    info!("");
    info!("Sensor Pin: GPIO 36 (ADC1_CH0) - Simulated");
    info!("LED Pin: GPIO 2 - Simulated");
    info!("Pump Relay Pin: GPIO 4 - Simulated");
    info!("");

    // Initialize mock sensor
    let mut sensor = MockSoilSensor::new();

    // Startup sequence simulation
    info!("Performing startup sequence...");
    for i in 0..3 {
        info!("LED ON (blink {})", i + 1);
        std::thread::sleep(Duration::from_millis(200));
        info!("LED OFF");
        std::thread::sleep(Duration::from_millis(200));
    }

    info!("System ready! Starting measurements...");

    if CALIBRATION_MODE {
        info!("=== CALIBRATION MODE ACTIVE ===");
        info!("Place sensor in DRY soil and note the reading");
        info!("Then place in WET soil and note the reading");
        info!("Update DRY_SOIL and WET_SOIL constants accordingly");
        info!("");
    }

    info!("Raw Value | Moisture % | Status");
    info!("----------|------------|--------");

    // Simulate different soil conditions over time
    let conditions = ["dry", "optimal", "wet", "optimal"];
    let mut condition_index = 0;
    let mut readings_count = 0;

    // Main sensor reading loop (limited for demonstration)
    for _ in 0..20 {
        // Change conditions every 5 readings
        if readings_count % 5 == 0 {
            let condition = conditions[condition_index % conditions.len()];
            sensor.set_soil_condition(condition);
            condition_index += 1;
        }

        // Read soil moisture sensor (averaged for stability)
        match sensor.read_averaged(5) {
            Ok(sensor_value) => {
                // Convert to moisture percentage
                let moisture_percent = raw_to_moisture_percent(sensor_value);

                // Determine soil condition and LED state
                let (soil_condition, led_state) = get_soil_condition(moisture_percent);

                // Simulate LED control
                let led_status = if led_state { "ON" } else { "OFF" };

                // Log readings
                info!(
                    "{:9} | {:8}% | {} (LED: {})",
                    sensor_value, moisture_percent, soil_condition, led_status
                );

                // Simulate pump control logic
                if moisture_percent < MOISTURE_LOW {
                    info!("     -> Pump: WOULD ACTIVATE (soil too dry)");
                } else if moisture_percent > MOISTURE_HIGH {
                    info!("     -> Pump: WOULD DEACTIVATE (soil too wet)");
                }
            }
            Err(e) => {
                error!("Failed to read sensor: {:?}", e);
            }
        }

        readings_count += 1;

        // Wait before next reading
        std::thread::sleep(Duration::from_millis(READING_INTERVAL_MS));
    }

    info!("========================================");
    info!("Demonstration complete!");
    info!("For real ESP32 hardware, use: ../soil-sensor-cpp/");
    info!("========================================");

    Ok(())
}
