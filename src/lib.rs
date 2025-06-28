#![no_std]
//! # super_simple_st7789driver
//!
//! A simple async Rust driver for the ST7789 TFT display, based on embedded-hal traits.
//!
//! ## Features
//! - Async SPI communication
//! - Basic display initialization
//! - Set row/column address
//! - Write memory and data
//!
//! ## Example
//! ```rust
//! use st7789driver::{St7789, Timer_};
//! use embedded_hal::digital::OutputPin;
//! use embedded_hal_async::spi::SpiBus;
//! 
//! struct MyTimer;
//! impl Timer_ for MyTimer {
//!     fn delay_ms(&self, ms: u64) -> impl core::future::Future<Output = ()> {
//!         async move {
//!             // Implement your async delay here
//!         }
//!     }
//! }
//! 
//! // Assume you have implementations for spi, cs, and dc
//! let mut lcd = St7789::new(spi, cs, dc, MyTimer);
//! 
//! // Initialization and basic operations
//! lcd.init().await.unwrap();
//! lcd.set_row(0, 319).await.unwrap();
//! lcd.set_col(0, 239).await.unwrap();
//! lcd.write_memory().await.unwrap();// send write data command
//! lcd.write_data(&[0x00]).await.unwrap();// send real data
//! ```
/// ST7789 command definitions
pub mod commands;
use defmt::info;

use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

use crate::commands::st7789_cmd;

/// Timer trait for async delay
///
/// Implement this trait to provide async millisecond delays for the driver.
pub trait Timer_ {
    /// Delay for the given milliseconds asynchronously.
    fn delay_ms(&self, ms: u64) -> impl core::future::Future<Output = ()>;
}

/// ST7789 display driver
///
/// # Type Parameters
/// - `SPI`: Async SPI bus implementing [`SpiBus`]
/// - `OUTPUT`: Output pin implementing [`OutputPin`]
/// - `TIMER`: Timer implementing [`Timer_`]
pub struct St7789<SPI: SpiBus, OUTPUT: OutputPin, TIMER: Timer_> {
    /// SPI bus
    spi: SPI,
    /// Delay in milliseconds between operations
    delay_ms: u64,
    /// Chip select pin
    cs: OUTPUT,
    /// Data/command pin
    dc: OUTPUT,
    /// Whether the display is initialized
    is_initiated: bool,
    /// Timer for delays
    timer: TIMER,
}

impl<T: SpiBus, OUTPUT: OutputPin, TIMER: Timer_> St7789<T, OUTPUT, TIMER> {
    /// Create a new ST7789 driver instance.
    ///
    /// # Arguments
    /// * `spi` - SPI bus
    /// * `cs` - Chip select pin
    /// * `dc` - Data/command pin
    /// * `timer` - Timer for delays
    pub fn new(spi: T, mut cs: OUTPUT, mut dc: OUTPUT, timer: TIMER) -> Self {
        cs.set_high().unwrap();
        dc.set_high().unwrap();
        Self {
            spi,
            delay_ms: 1,
            cs,
            dc,
            is_initiated: false,
            timer,
        }
    }

    /// Initialize the display with basic settings.
    ///
    /// This function must be called before any other display operations.
    pub async fn init(&mut self) -> Result<(), T::Error> {
        self.is_initiated = true;
        self.timer.delay_ms(self.delay_ms).await;
        self.cs.set_low().unwrap();
        self.timer.delay_ms(self.delay_ms).await;
        self.write_command(&[st7789_cmd::RESET]).await?;

        self.write_command(&[
            st7789_cmd::SLEEP_OUT,
            st7789_cmd::DISPLAY_ON,
            st7789_cmd::DISPLAY_INVERSION_ON,
        ])
        .await?;

        self.write_command(&[st7789_cmd::COL_MODE]).await?;
        self.write_data(&[0x55_u8]).await?;

        Ok(())
    }

    /// Set the row address window.
    ///
    /// # Arguments
    /// * `start` - Start row (0..=319)
    /// * `end` - End row (0..=319)
    pub async fn set_row(&mut self, start: u16, end: u16) -> Result<(), T::Error> {
        let start_hight = (start >> 8) as u8;
        let start_low = (start & 0x00ff) as u8;
        let end_hight = (end >> 8) as u8;
        let end_low = (end & 0x00ff) as u8;
        info!("{:?}", &[start_hight, start_low, end_hight, end_low]);

        self.write_command(&[st7789_cmd::ROW_ADDRESS_SET]).await?;
        self.write_data(&[start_hight, start_low, end_hight, end_low])
            .await?;
        Ok(())
    }

    /// Set the column address window.
    ///
    /// # Arguments
    /// * `start` - Start column (0..=239)
    /// * `end` - End column (0..=239)
    pub async fn set_col(&mut self, start: u16, end: u16) -> Result<(), T::Error> {
        let start_hight = (start >> 8) as u8;
        let start_low = (start & 0x00ff) as u8;
        let end_hight = (end >> 8) as u8;
        let end_low = (end & 0x00ff) as u8;
        info!("{:?}", &[start_hight, start_low, end_hight, end_low]);
        self.write_command(&[st7789_cmd::COLUMN_ADDRESS_SET])
            .await?;
        self.write_data(&[start_hight, start_low, end_hight, end_low])
            .await?;
        Ok(())
    }

    /// Send the MEMORY_WRITE command to start writing pixel data.
    pub async fn write_memory(&mut self) -> Result<(), T::Error> {
        self.write_command(&[st7789_cmd::MEMORY_WRITE]).await?;
        Ok(())
    }

    /// Write data to the display (after setting address window).
    ///
    /// # Arguments
    /// * `data` - Data bytes to send
    pub async fn write_data(&mut self, data: &[u8]) -> Result<(), T::Error> {
        if !self.is_initiated {
            panic!("init first!");
        }
        self.dc.set_high().unwrap();
        self.spi.write(data).await?;
        Ok(())
    }

    /// Write a command to the display.
    ///
    /// # Arguments
    /// * `data` - Command bytes to send
    async fn write_command(&mut self, data: &[u8]) -> Result<(), T::Error> {
        if !self.is_initiated {
            panic!("init first!");
        }
        self.dc.set_low().unwrap();
        self.timer.delay_ms(self.delay_ms).await;
        self.spi.write(data).await?;
        self.timer.delay_ms(self.delay_ms).await;
        Ok(())
    }
}
