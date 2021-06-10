//==============================================================================
// Notes
//==============================================================================
// drivers::seven_seg/mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu;
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


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
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM1_PORT,
		pin: config::SEVEN_SEG_COM1_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
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
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_COM4_PORT,
		pin: config::SEVEN_SEG_COM4_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
];

#[allow(dead_code)]
const SEG_LINES: [gpio::PinConfig; 7] = [
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGA_PORT,
		pin: config::SEVEN_SEG_SEGA_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGB_PORT,
		pin: config::SEVEN_SEG_SEGB_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGC_PORT,
		pin: config::SEVEN_SEG_SEGC_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGD_PORT,
		pin: config::SEVEN_SEG_SEGD_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGE_PORT,
		pin: config::SEVEN_SEG_SEGE_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGF_PORT,
		pin: config::SEVEN_SEG_SEGF_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
	gpio::PinConfig {
		port: config::SEVEN_SEG_SEGG_PORT,
		pin: config::SEVEN_SEG_SEGG_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinHigh,
	},
];

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
	gpio::print_dir(mcu::Port::Port2);
	gpio::print_out(mcu::Port::Port2);
	gpio::print_dir(mcu::Port::Port4);
	gpio::print_out(mcu::Port::Port4);
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
pub fn task_handler(){
	static mut ACTIVE_COM: usize = 0;

	unsafe { 
		// Set the last line high (off)
		gpio::set_pin_state(&COM_LINES[ACTIVE_COM], gpio::PinState::PinLow);
		ACTIVE_COM = if ACTIVE_COM == 3 { 0 } else { ACTIVE_COM + 1 };

		// Set the next line low (active)
		gpio::set_pin_state(&COM_LINES[ACTIVE_COM], gpio::PinState::PinHigh);
	}
}
