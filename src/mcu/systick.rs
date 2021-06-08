//==============================================================================
// Notes
//==============================================================================
// mcu::systick.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::exception;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const SYSTICK_FREQUENCY: u32 = 1;	//Hz
static mut INITIALIZED: bool = false;
static SYSTICK_HANDLE: Mutex<RefCell<Option<cortex_m::peripheral::SYST>>> = 
	Mutex::new(RefCell::new(None));
static SYSTICK_TIME: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(mut systick: cortex_m::peripheral::SYST) {
	unsafe {
		if INITIALIZED {
			return;
		}
	}

	configure(&mut systick);

	free(|cs| SYSTICK_HANDLE.borrow(cs).replace(Some(systick)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_ticks() -> u32 {
	free(|cs| SYSTICK_TIME.borrow(cs).get())
}

#[allow(dead_code)]
pub fn get_diff(diff: u32) -> u32 {
	let current = get_ticks();
	if diff > current {
		0
	}
	else {
		current - diff
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure (systick: &mut cortex_m::peripheral::SYST) {
	// Set the systick clock source
	systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
	
	// Core should be running at 24MHz - fire every second
	systick.set_reload(24_000_000);
	
	systick.clear_current();
	systick.enable_counter();
	systick.enable_interrupt();	
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[exception]
fn SysTick() {
	static mut COUNT: u32 = 0;
	*COUNT = *COUNT + 1;
	if *COUNT == SYSTICK_FREQUENCY {
		* COUNT = 0;
		free(|cs| SYSTICK_TIME.borrow(cs).set(SYSTICK_TIME.borrow(cs).get() + 1));
	}
}

//==============================================================================
// Task Handler
//==============================================================================

