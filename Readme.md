# st7789driver

> 📖 This README is in English. [点击这里查看中文文档 (README_ZH.md)](./README_ZH.md)

`st7789driver` is an async Rust driver library for controlling the ST7789 display, designed for embedded no_std environments. It is based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) and [`embedded-hal-async`](https://github.com/embassy-rs/embedded-hal-async) traits, supporting SPI communication and async delays.

## Features

- ST7789 display initialization
- Row/column address setting
- Video memory write commands
- Async SPI communication
- Customizable delay implementation

## Usage

### Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
st7789driver = { path = "your_path/st7789driver" }
embedded-hal = "1"
embedded-hal-async = "1"
defmt = "0.3"
```

### Example

```rust
use st7789driver::{St7789, Timer_};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

struct MyTimer;
impl Timer_ for MyTimer {
    fn delay_ms(&self, ms: u64) -> impl core::future::Future<Output = ()> {
        async move {
            // Implement your async delay here
        }
    }
}

// Assume you have implementations for spi, cs, and dc
let mut lcd = St7789::new(spi, cs, dc, MyTimer);

// Initialization and basic operations
lcd.init().await.unwrap();
lcd.set_row(0, 319).await.unwrap();
lcd.set_col(0, 239).await.unwrap();
lcd.write_memory().await.unwrap();// send write data command
lcd.write_data(&[0x00]).await.unwrap();// send real data
```

## API Overview

- `init()`: Initialize the display
- `set_row(start, end)`: Set row address
- `set_col(start, end)`: Set column address
- `write_memory()`: Write to video memory
- `write_data(data)`: Write data

## Dependencies

- [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
- [`embedded-hal-async`](https://github.com/embassy-rs/embedded-hal-async)
- [`defmt`](https://github.com/knurling-rs/defmt)

## License

MIT License

