//==============================================================================
// Notes
//==============================================================================
// mcu::rtc.rs
// RTC Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
// use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
// use crate::mcu;
// use crate::mcu::gpio;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static RTC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::RTC_C>>> = 
	Mutex::new(RefCell::new(None));

static CURRENT_TIME: Mutex<Cell<Option<u32>>> = Mutex::new(Cell::new(None));
static CURRENT_TIME_MS: Mutex<Cell<Option<u32>>> = Mutex::new(Cell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(rtc: msp432p401r_pac::RTC_C) {
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| RTC_HANDLE.borrow(cs).replace(Some(rtc)));
	free(|cs| CURRENT_TIME.borrow(cs).replace(Some(0)));
	free(|cs| CURRENT_TIME_MS.borrow(cs).replace(Some(0)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_diff(time: u32) -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	let current = free(|cs| CURRENT_TIME.borrow(cs).get() ).unwrap();
	if time < current {
		0
	}
	else {
		current - time
	}
}

#[allow(dead_code)]
pub fn get_diff_ms(time: u32, time_ms: u32) -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	let current = free(|cs| CURRENT_TIME.borrow(cs).get() ).unwrap();
	let current_ms = free(|cs| CURRENT_TIME_MS.borrow(cs).get() ).unwrap();
	let current_actual = current + current_ms;
	let time_actual = time + time_ms;

	if time_actual < current_actual {
		0
	}
	else {
		current_actual - time_actual
	}
}

#[allow(dead_code)]
pub fn get_time() -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	free(|cs| CURRENT_TIME.borrow(cs).get() ).unwrap()
}

#[allow(dead_code)]
pub fn get_time_ms() -> u32 {
	unsafe { if !INITIALIZED {
		return 0;
	}}

	free(|cs| CURRENT_TIME_MS.borrow(cs).get() ).unwrap()
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
