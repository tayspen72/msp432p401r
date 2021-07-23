//==============================================================================
// Notes
//==============================================================================
// mcu::adc.rs
// ADC Driver

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
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct Adc{

}

//==============================================================================
// Variables
//==============================================================================
static ADC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::ADC14>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(adc: msp432p401r_pac::ADC14) {
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| ADC_HANDLE.borrow(cs).replace(Some(adc)));

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