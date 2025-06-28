#![no_std]
pub mod commands;
use defmt::info;

use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

use crate::commands::st7789_cmd;

pub trait Timer_ {
    fn delay_ms(&self, ms: u64) -> impl Future<Output = ()>;
}

pub struct St7789<SPI: SpiBus, OUTPUT: OutputPin, TIMER: Timer_> {
    spi: SPI,
    delay_ms: u64,
    cs: OUTPUT,
    dc: OUTPUT,
    is_initiated: bool,
    timer: TIMER,
}

impl<T: SpiBus, OUTPUT: OutputPin, TIMER: Timer_> St7789<T, OUTPUT, TIMER> {
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
    pub async fn init(&mut self) -> Result<(), T::Error> {
        self.is_initiated = true;
        self.timer.delay_ms(self.delay_ms).await;
        // Timer::after_millis(self.delay_ms).await;
        self.cs.set_low().unwrap();
        self.timer.delay_ms(self.delay_ms).await;
        // Timer::after_millis(self.delay_ms).await;
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
    /// 0..=319
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
    /// 0..=239
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
    pub async fn write_memory(&mut self) -> Result<(), T::Error> {
        self.write_command(&[st7789_cmd::MEMORY_WRITE]).await?;
        // self.write_data(data).await?;
        Ok(())
    }
    pub async fn write_data(&mut self, data: &[u8]) -> Result<(), T::Error> {
        if !self.is_initiated {
            panic!("init first!");
        }
        self.dc.set_high().unwrap();
        // Timer::after_millis(self.delay_ms).await;
        self.spi.write(data).await?;
        Ok(())
    }
    async fn write_command(&mut self, data: &[u8]) -> Result<(), T::Error> {
        if !self.is_initiated {
            panic!("init first!");
        }
        self.dc.set_low().unwrap();
        self.timer.delay_ms(self.delay_ms).await;

        // Timer::after_millis(self.delay_ms).await;
        self.spi.write(data).await?;
        self.timer.delay_ms(self.delay_ms).await;

        // Timer::after_millis(self.delay_ms).await;
        Ok(())
    }
}
