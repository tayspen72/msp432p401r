//==============================================================================
// Notes
//==============================================================================
// main.rs

//==============================================================================
// Crates and Mods
//==============================================================================
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

mod app;
mod config;
mod drivers;
mod mcu;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Main
//==============================================================================
#[entry]
fn main() -> ! {
	init();
	
	let mut info = app::Info::take().unwrap();

	loop {
		task_handler(&mut info);
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn init() {
	mcu::init();
	drivers::init();
}

//==============================================================================
// Task Handler
//==============================================================================
fn task_handler(info: &mut app::Info) {
	mcu::task_handler();
	drivers::task_handler(info);
	app::task_handler(info);
}
