//==============================================================================
// Notes
//==============================================================================
// mcu::counter.rs
// A GPIO Pin Counter Using the Internal Timer_A Peripheral

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
// use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct Counter{
	pub sda_port: mcu::Port,
	pub sda_pin: u8,
	pub scl_port: mcu::Port,
	pub scl_pin: u8,
	pub function_select: u8,
	pub address: u8,
	pub speed: u32
}

//==============================================================================
// Variables
//==============================================================================
static TIMER_A0_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A0>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A1_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A1>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A2_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A2>>> = 
	Mutex::new(RefCell::new(None));
static TIMER_A3_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::TIMER_A3>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(
	timer_a0: msp432p401r_pac::TIMER_A0,
	timer_a1: msp432p401r_pac::TIMER_A1,
	timer_a2: msp432p401r_pac::TIMER_A2,
	timer_a3: msp432p401r_pac::TIMER_A3,
){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| TIMER_A0_HANDLE.borrow(cs).replace(Some(timer_a0)));
	free(|cs| TIMER_A1_HANDLE.borrow(cs).replace(Some(timer_a1)));
	free(|cs| TIMER_A2_HANDLE.borrow(cs).replace(Some(timer_a2)));
	free(|cs| TIMER_A3_HANDLE.borrow(cs).replace(Some(timer_a3)));

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
