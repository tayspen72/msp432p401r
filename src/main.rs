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
// use cortex_m_semihosting::hprintln;
// use panic_halt as _; // Breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _;

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
	
	loop {
		task_handler();
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
fn task_handler() {
	mcu::task_handler();
	drivers::task_handler();
}
