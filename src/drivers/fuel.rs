//==============================================================================
// Notes
//==============================================================================
// drivers/fuel.rs
// Fuel Level

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu::{adc, systick};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum FuelLevel {
	LevelUnknown,
	LevelEmpty,
	Level1,
	Level2,
	Level3,
	LevelFull,
}

const FUEL_ADC: adc::Adc = adc::Adc {
	port: config::FUEL_ADC_PORT,
	pin: config::FUEL_ADC_PIN,
	channel: config::FUEL_ADC_CHANNEL,
	function_select: config::FUEL_ADC_FUNCTION_SELECT,
	resolution: adc::Resolution::B14
};

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
const FUEL_UPDATE_TIME: u32 = 5;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	adc::configure(&FUEL_ADC);
}

#[allow(dead_code)]
pub fn get_fuel_level() -> FuelLevel {
	// TODO: Finish this
	let adc = adc::read_ref(&FUEL_ADC, 2.5);

	let r = 1.0 / ((3.3 / adc ) - 1.0);

	match r {
		i if i < 100.0 => FuelLevel::LevelEmpty,
		i if i < 200.0 => FuelLevel::Level1,
		i if i < 300.0 => FuelLevel::Level2,
		i if i < 400.0 => FuelLevel::Level3,
		i if i < 500.0 => FuelLevel::LevelFull,
		_ => FuelLevel::LevelUnknown
	}
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info){
	static mut LAST_TIME: u32 = 0;

	if info.change_flags.speed {
		info.change_flags.speed = false;
	}
	
	unsafe { 
		// if rtc::get_diff(LAST_TIME) > FUEL_UPDATE_TIME {
			// LAST_TIME = rtc::get_time();
		if systick::get_diff(LAST_TIME) >= 20 {
			LAST_TIME = systick::get_time();

			let speed = info.speed;
			info.speed = adc::read(&FUEL_ADC);
			if info.speed != speed {
				info.change_flags.speed = true;
			}
		}
	}
}