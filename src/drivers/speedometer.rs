//==============================================================================
// Notes
//==============================================================================
// drivers/speedometer.rs
// The means for determing speed

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu;
use crate::mcu::{counter, gpio, systick};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const COUNTER: counter::Counter = counter::Counter {
	taclk_port: config::COUNTER_TACLK_PORT,
	taclk_pin: config::COUNTER_TACLK_PIN,
	taclk: config::COUNTER_TACLK,
	function_select: config::COUNTER_FUNCTION_SELECT
};

const UPDATE_FREQUENCY: u32 = 25;	// Update (25/100)x per second

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	counter::setup(&COUNTER);
	counter::start(&COUNTER, true);
	
	gpio::pin_setup(&gpio::PinConfig {
		port: mcu::Port::Port2,
		pin: 5,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh
	});
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info){
	static mut LAST_TIME: u32 = 0;
	
	// Clear any previously set flags
	if info.change_flags.speed {
		info.change_flags.speed = false;
	}
	
	unsafe { 
		if systick::get_diff(LAST_TIME) > UPDATE_FREQUENCY {
			LAST_TIME = systick::get_time();
			let speed = counter::get_count(&COUNTER);
			
			if info.speed != speed {
				info.change_flags.speed = true;
				info.speed = speed;
			}
		}
	}
	
	static mut LAST: bool = true;
	static mut TOGGLE_TIME: u32 = 0;
	
	unsafe { 
		if systick::get_diff(TOGGLE_TIME) < 1 {
			return;
		}
		
		TOGGLE_TIME = systick::get_time();
	}
	
	unsafe { 
		if LAST {
			gpio::set_pin_state(
				&gpio::PinConfig {
					port: mcu::Port::Port2,
					pin: 5,
					direction: gpio::PinDirection::Output,
					pull: gpio::PinPull::PullDisabled,
					state: gpio::PinState::PinHigh,
				},
				gpio::PinState::PinLow
			);
			LAST = false;
		}
		else {
			gpio::set_pin_state(
				&gpio::PinConfig {
					port: mcu::Port::Port2,
					pin: 5,
					direction: gpio::PinDirection::Output,
					pull: gpio::PinPull::PullDisabled,
					state: gpio::PinState::PinHigh,
				},
				gpio::PinState::PinHigh
			);
			LAST = true;
		}
	}
}