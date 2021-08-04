//==============================================================================
// Notes
//==============================================================================
// drivers/lcd.rs
// 2x16 LCD driver

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::{gpio, timer};
use gpio::PinState as PinState;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
enum Command {
	ClearDisplay =	0x01,
	ReturnHome =	0x02,
	EntryModeSet =	0x04,
	DisplayOnOff =	0x08,
	CursorShift =	0x10,
	FunctionSet =	0x20,
	SetCgram =		0x40,
	SetDdram =		0x80,
}

#[allow(dead_code)]
enum DisplayModeBlink {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum DisplayModeCursor {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum DisplayModeDisplay {
	Off = 0,
	On = 1,	
}

#[allow(dead_code)]
enum EntryModeIncrement {
	Decrement = 0,
	Increment = 1,	
}

#[allow(dead_code)]
enum EntryModeShift {
	Right = 0,
	Left = 1,	
}

#[allow(dead_code)]
enum FunctionSetDataLength {
	Bits4 = 0,
	Bits8 = 1
}

#[allow(dead_code)]
enum FunctionSetDisplayLines {
	Lines1 = 0,
	Lines2 = 1
}

#[allow(dead_code)]
enum FunctionSetFontType {
	Type5x8,
	Type5x11
}

//==============================================================================
// Variables
//==============================================================================
const RS: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_RS_PORT,
	pin: config::LCD_RS_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const RW: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_RW_PORT,
	pin: config::LCD_RW_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const EN: gpio::PinConfig = gpio::PinConfig {
	port: config::LCD_EN_PORT,
	pin: config::LCD_EN_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const D: [gpio::PinConfig; 8] = [
	gpio::PinConfig {
		port: config::LCD_D0_PORT,
		pin: config::LCD_D0_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D1_PORT,
		pin: config::LCD_D1_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D2_PORT,
		pin: config::LCD_D2_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D3_PORT,
		pin: config::LCD_D3_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D4_PORT,
		pin: config::LCD_D4_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D5_PORT,
		pin: config::LCD_D5_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D6_PORT,
		pin: config::LCD_D6_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
	gpio::PinConfig {
		port: config::LCD_D7_PORT,
		pin: config::LCD_D7_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow,
	},
];

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	gpio::pin_setup(&RS);
	gpio::pin_setup(&RW);
	gpio::pin_setup(&EN);
	for d in D.iter() {
		gpio::pin_setup(&d);
	}
	
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x30);
	timer::delay(1);
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x30);
	timer::delay(1);
	write_command(gpio::PinState::PinLow, gpio::PinState::PinLow, 0x30);
	timer::delay(1);
	clear_display();
	entry_mode_set(EntryModeIncrement::Increment, EntryModeShift::Right);
	function_set(FunctionSetDataLength::Bits8, FunctionSetDisplayLines::Lines2, FunctionSetFontType::Type5x11);
	display_on_off_set(DisplayModeDisplay::On, DisplayModeCursor::Off, DisplayModeBlink::Off);
	set_ddram_address(0x00);
	
	// Write this to start off
	return_home();
	write_address(0x00);
	write_data('A' as u8);
	write_data('B' as u8);
	write_data('C' as u8);
	write_data('D' as u8);
	write_data('E' as u8);
	write_data('F' as u8);
	write_data('G' as u8);
	write_data('H' as u8);
	write_data('I' as u8);
	write_data('J' as u8);
	write_data('K' as u8);
	write_data('L' as u8);
	write_data('M' as u8);
	write_data('N' as u8);
	write_data('O' as u8);
	write_data('P' as u8);
	
	write_data('0' as u8);
	write_data('1' as u8);
	write_data('2' as u8);
	write_data('3' as u8);
	write_data('4' as u8);
	write_data('5' as u8);
	write_data('6' as u8);
	write_data('7' as u8);
	write_data('8' as u8);
	write_data('9' as u8);
	write_data(0xF6);
	write_data(0xF7);
	write_data(0xC7);
	write_data(0xC8);
	write_data(0x7E);
	write_data(0x7F);
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn clear_display() {
	write_command(PinState::PinLow, PinState::PinLow, Command::ClearDisplay as u8);
}

#[allow(dead_code)]
fn display_on_off_set(display: DisplayModeDisplay, cursor: DisplayModeCursor, blink: DisplayModeBlink) {
	let mut val = Command::DisplayOnOff as u8;
	if let DisplayModeDisplay::On = display {
		val |= 0x4;
	}
	if let DisplayModeCursor::On = cursor {
		val |= 0x2;
	}
	if let DisplayModeBlink::On = blink {
		val |= 0x1;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn entry_mode_set(increment: EntryModeIncrement, shift: EntryModeShift) {
	let mut val = Command::EntryModeSet as u8;
	if let EntryModeIncrement::Increment = increment {
		val |= 0x2;
	}
	if let EntryModeShift::Left = shift {
		val |= 0x1;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn function_set(length: FunctionSetDataLength, lines: FunctionSetDisplayLines, font: FunctionSetFontType) {
	let mut val = Command::FunctionSet as u8;
	if let FunctionSetDataLength::Bits8 = length {
		val |= 0x10;
	}
	if let FunctionSetDisplayLines::Lines2 = lines {
		val |= 0x8;
	}
	if let FunctionSetFontType::Type5x11 = font {
		val |= 0x4;
	}
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn return_home() {
	write_command(PinState::PinLow, PinState::PinLow, Command::ReturnHome as u8);
}

#[allow(dead_code)]
fn set_cgram_address(address: u8) {
	let val = (address & 0x3F) | Command::SetCgram as u8;
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn set_ddram_address(address: u8) {
	let val = (address & 0x7F) | Command::SetDdram as u8;
	write_command(PinState::PinLow, PinState::PinLow, val);
}

#[allow(dead_code)]
fn write_address(address: u8) {
	write_command(PinState::PinHigh, PinState::PinHigh, address);
}

#[allow(dead_code)]
fn write_data(data: u8) {
	write_command(PinState::PinHigh, PinState::PinLow, data);
}

#[allow(dead_code)]
fn write_command(rs_state: gpio::PinState, rw_state: gpio::PinState, val: u8) {
	gpio::set_pin_state(RS.port, RS.pin, rs_state);
	gpio::set_pin_state(RS.port, RW.pin, rw_state);
		
	for i in 0..8 {
		let state = if val & (1 << i) > 0 { PinState::PinHigh } else { PinState::PinLow };
		gpio::set_pin_state(D[i].port, D[i].pin, state);
	}
	
	gpio::set_pin_state(EN.port, EN.pin, PinState::PinHigh);

	timer::delay(1);
	
	gpio::set_pin_state(EN.port, EN.pin, PinState::PinLow);
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){
	
}