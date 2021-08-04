//==============================================================================
// Notes
//==============================================================================
// drivers::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod fuel;
pub mod lcd;
pub mod led;
pub mod odometer;
pub mod quadalpha;
pub mod speedometer;

use crate::app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	fuel::init();
	lcd::init();
	led::init();
	odometer::init();
	quadalpha::init();
	speedometer::init();
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(info: &mut app::Info) {
	fuel::task_handler(info);
	lcd::task_handler();
	led::task_handler();
	odometer::task_handler(info);
	quadalpha::task_handler(info);
	speedometer::task_handler(info);
}