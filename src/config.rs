//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::mcu;
use crate::mcu::{adc, counter, eusci};

//==============================================================================
// ADC
//==============================================================================


//==============================================================================
// Clock
//==============================================================================
#[allow(dead_code)] pub const LFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_IN_PIN: u8 = 0;
#[allow(dead_code)] pub const LFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_OUT_PIN: u8 = 1;
#[allow(dead_code)] pub const HFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_IN_PIN: u8 = 2;
#[allow(dead_code)] pub const HFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_OUT_PIN: u8 = 3;

//==============================================================================
// Counter
//==============================================================================
#[allow(dead_code)] pub const COUNTER_TACLK_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const COUNTER_TACLK_PIN: u8 = 2;
#[allow(dead_code)] pub const COUNTER_TACLK: counter::TaClk = counter::TaClk::A2;
#[allow(dead_code)] pub const COUNTER_FUNCTION_SELECT: u8 = 0b10;

//==============================================================================
// Debug
//==============================================================================


//==============================================================================
// Flash
//==============================================================================


//==============================================================================
// Fuel ADC
//==============================================================================
#[allow(dead_code)] pub const FUEL_ADC_PORT: mcu::Port = mcu::Port::Port5;
#[allow(dead_code)] pub const FUEL_ADC_PIN: u8 = 5;
#[allow(dead_code)] pub const FUEL_ADC_CHANNEL: adc::Channel = adc::Channel::A0;
#[allow(dead_code)] pub const FUEL_ADC_FUNCTION_SELECT: u8 = 0b11;

//==============================================================================
// I2C
//==============================================================================
#[allow(dead_code)] pub const I2C_SDA_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SDA_PIN: u8 = 6;
#[allow(dead_code)] pub const I2C_SCL_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SCL_PIN: u8 = 7;
#[allow(dead_code)] pub const I2C_EUSCI: eusci::EUSCI = eusci::EUSCI::B0;
#[allow(dead_code)] pub const I2C_ADDRESS: u8 = 0x70;
#[allow(dead_code)] pub const I2C_SPEED: u32 = 400_000;
#[allow(dead_code)] pub const I2C_FUNCTION_SELECT: u8 = 0b01;

//==============================================================================
// LCD
//==============================================================================


//==============================================================================
// Push Button
//==============================================================================
// pub const PUSH_BUTTON_IN_PIN: u8 	= 13;
// pub const PUSH_BUTTON_OUT_PIN: u8 	= 15;

//==============================================================================
// RTC
//==============================================================================


//==============================================================================
// Seven Segment
//==============================================================================
#[allow(dead_code)] pub const SEVEN_SEG_COM0_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const SEVEN_SEG_COM0_PIN: u8 = 4;
#[allow(dead_code)] pub const SEVEN_SEG_COM1_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const SEVEN_SEG_COM1_PIN: u8 = 5;
#[allow(dead_code)] pub const SEVEN_SEG_COM2_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const SEVEN_SEG_COM2_PIN: u8 = 6;
#[allow(dead_code)] pub const SEVEN_SEG_COM3_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const SEVEN_SEG_COM3_PIN: u8 = 7;
#[allow(dead_code)] pub const SEVEN_SEG_COM4_PORT: mcu::Port = mcu::Port::Port2;
#[allow(dead_code)] pub const SEVEN_SEG_COM4_PIN: u8 = 3;

#[allow(dead_code)] pub const SEVEN_SEG_SEGA_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGA_PIN: u8 = 0;
#[allow(dead_code)] pub const SEVEN_SEG_SEGB_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGB_PIN: u8 = 1;
#[allow(dead_code)] pub const SEVEN_SEG_SEGC_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGC_PIN: u8 = 2;
#[allow(dead_code)] pub const SEVEN_SEG_SEGD_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGD_PIN: u8 = 3;
#[allow(dead_code)] pub const SEVEN_SEG_SEGE_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGE_PIN: u8 = 4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGF_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGF_PIN: u8 = 5;
#[allow(dead_code)] pub const SEVEN_SEG_SEGG_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const SEVEN_SEG_SEGG_PIN: u8 = 6;

//==============================================================================
// SPI
//==============================================================================


//==============================================================================
// Temperature Sensor
//==============================================================================
#[allow(dead_code)] pub const TEMPERATURE_ADC_PORT: mcu::Port = mcu::Port::PortDisabled;
#[allow(dead_code)] pub const TEMPERATURE_ADC_PIN: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_CHANNEL: adc::Channel = adc::Channel::Temperature;
#[allow(dead_code)] pub const TEMPERATURE_ADC_SIGNAL: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_FUNCTION_SELECT: u8 = 0b11;

//==============================================================================
// Uart
//==============================================================================
