//==============================================================================
// Notes
//==============================================================================
// drivers/speedometer.rs
// The means for determing speed

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::{app, config};
use crate::mcu::{counter, systick};

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
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(_binfo: &mut app::Info){
	static mut LAST_TIME: u32 = 0;
	unsafe { 
		if systick::get_diff(LAST_TIME) > UPDATE_FREQUENCY {
			LAST_TIME = systick::get_time();
			counter::get_count(&COUNTER);
		}
	}
}