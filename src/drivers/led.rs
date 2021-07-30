//==============================================================================
// Notes
//==============================================================================
// drivers/led.rs
// User Input and Output Checking

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::{gpio, input};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const LED1: gpio::PinConfig = gpio::PinConfig {
	port: config::LED_BLUE_PORT,
	pin: config::LED_BLUE_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const LED2: gpio::PinConfig = gpio::PinConfig {
	port: config::LED_GREEN_PORT,
	pin: config::LED_GREEN_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const BUTTON1: input::Input = input::Input {
	port: config::BUTTON_1_PORT,
	pin: config::BUTTON_1_PIN,
	pull: gpio::PinPull::PullUp,
	edge: input::EdgeTrigger::Falling,
	callback: button_1_handler
};
const BUTTON2: input::Input = input::Input {
	port: config::BUTTON_2_PORT,
	pin: config::BUTTON_2_PIN,
	pull: gpio::PinPull::PullUp,
	edge: input::EdgeTrigger::Falling,
	callback: button_2_handler
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	gpio::pin_setup(&LED1);
	gpio::pin_setup(&LED2);
	input::configure(&BUTTON1);
	input::configure(&BUTTON2);
}

//==============================================================================
// Private Functions
//==============================================================================
fn button_1_handler() {
	static mut LAST_STATE:gpio::PinState = gpio::PinState::PinLow; 
	unsafe { 
		LAST_STATE = if LAST_STATE == gpio::PinState::PinLow {
			gpio::PinState::PinHigh
		}
		else {
			gpio::PinState::PinLow
		};
		gpio::set_pin_state(LED1.port, LED1.pin, LAST_STATE);
	}
}

fn button_2_handler() {
	static mut LAST_STATE:gpio::PinState = gpio::PinState::PinLow; 
	unsafe {
		LAST_STATE = if LAST_STATE == gpio::PinState::PinLow {
			gpio::PinState::PinHigh
		}
		else {
			gpio::PinState::PinLow
		};
		gpio::set_pin_state(LED2.port, LED2.pin, LAST_STATE);
	}
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){
	
}