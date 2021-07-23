//==============================================================================
// Notes
//==============================================================================
// mcu::rtc.rs
// RTC Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use crate::mcu::gpio;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static RTC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::RTC_C>>> = 
	Mutex::new(RefCell::new(None));

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

	unsafe {
		INITIALIZED = true;
	}
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
