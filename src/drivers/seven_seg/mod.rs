//==============================================================================
// Notes
//==============================================================================
// drivers::seven_seg/mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
type NumericValues = [bool; 7];

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
const COM_LINES: [gpio::PinConfig; 5] = [
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM0_PORT,
		pin: config::SEVEN_SEG_COM0_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM1_PORT,
		pin: config::SEVEN_SEG_COM1_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM2_PORT,
		pin: config::SEVEN_SEG_COM2_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM3_PORT,
		pin: config::SEVEN_SEG_COM3_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM4_PORT,
		pin: config::SEVEN_SEG_COM4_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
];

#[allow(dead_code)]
const SEG_LINES: [gpio::PinConfig; 7] = [
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGA_PORT,
		pin: config::SEVEN_SEG_SEGA_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGB_PORT,
		pin: config::SEVEN_SEG_SEGB_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGC_PORT,
		pin: config::SEVEN_SEG_SEGC_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGD_PORT,
		pin: config::SEVEN_SEG_SEGD_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGE_PORT,
		pin: config::SEVEN_SEG_SEGE_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGF_PORT,
		pin: config::SEVEN_SEG_SEGF_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGG_PORT,
		pin: config::SEVEN_SEG_SEGG_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
];

const NUMERIC_VALUES: [NumericValues; 11] = [
	[ false, false, false, false, false, false, true ],		// 0
	[ true, false, false, true, true, true, true ],			// 1
	[ false, false, true, false, false, true, false ],		// 2
	[ false, false, false, false, true, true, false ],		// 3
	[ true, false, false, true, true, false, false ],		// 4
	[ false, true, false, false, true, false, false ],		// 5
	[ false, true, false, false, false, false, false ],		// 6
	[ false, false, false, true, true, true, true ],		// 7
	[ false, false, false, false, false, false, false ],	// 8
	[ false, false, false, false, true, false, false ],		// 9
	[ true, true, true, true, true, true, true ],			// " "
];

static mut DISPLAY_VALUE: [u8; 4] = [ 0, 0, 0, 0];

//==============================================================================
// Public Functions
//==============================================================================\
pub fn init(){
	for &ref com in COM_LINES.iter() {
		gpio::set_pin_function_select(com, 0);
		gpio::pin_setup(com);
	}

	for &ref seg in SEG_LINES.iter() {
		gpio::set_pin_function_select(seg, 0);
		gpio::pin_setup(seg);
	}
}

#[allow(dead_code)]
pub fn set_value(value: [u8; 4]) {
	unsafe {
		for i in 0..DISPLAY_VALUE.len() {
			DISPLAY_VALUE[i] = value[i];
		}
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn set_segments(mut value: usize) {
	// Watch for unhandled values that will cause fault on array bounds
	if value > 9 {
		value = 10; // Will display a blank
	}

	for s in 0..7 {
		if NUMERIC_VALUES[value][s] {
			gpio::set_pin_state(&SEG_LINES[s], gpio::PinState::PinHigh);
		}
		else {
			gpio::set_pin_state(&SEG_LINES[s], gpio::PinState::PinLow);
		}
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){
	static mut ACTIVE_COM: usize = 3;

	unsafe {
		// Set the last line high (off)
		gpio::set_pin_state(&COM_LINES[ACTIVE_COM], gpio::PinState::PinLow);
		ACTIVE_COM = if ACTIVE_COM == 3 { 0 } else { ACTIVE_COM + 1 };

		// Update the segments
		set_segments(DISPLAY_VALUE[ACTIVE_COM].into());

		// Set the next line low (active)
		gpio::set_pin_state(&COM_LINES[ACTIVE_COM], gpio::PinState::PinHigh);
	}
}
