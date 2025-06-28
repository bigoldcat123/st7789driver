# st7789driver

`st7789driver` 是一个基于 Rust 的异步驱动库，用于控制 ST7789 显示屏，适用于嵌入式 no_std 环境。该库基于 [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) 和 [`embedded-hal-async`](https://github.com/embassy-rs/embedded-hal-async) traits，支持 SPI 通信和异步延时。

## 特性

- 支持 ST7789 屏幕初始化
- 行/列地址设置
- 显存写入命令
- 异步 SPI 通信
- 可自定义延时实现

## 用法

### 依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
st7789driver = { path = "你的路径/st7789driver" }
embedded-hal = "1"
embedded-hal-async = "1"
defmt = "0.3"
```

### 示例

```rust
use st7789driver::{St7789, Timer_};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

struct MyTimer;
impl Timer_ for MyTimer {
    fn delay_ms(&self, ms: u64) -> impl core::future::Future<Output = ()> {
        async move {
            // 实现你的异步延时
        }
    }
}

// 假设你有 spi, cs, dc 的实现
let mut lcd = St7789::new(spi, cs, dc, MyTimer);

// 初始化和基本操作
lcd.init().await.unwrap();
lcd.set_row(0, 319).await.unwrap();
lcd.set_col(0, 239).await.unwrap();
lcd.write_memory().await.unwrap();// 发送写数据指令
lcd.write_data(&[0x00]).await.unwrap();// 发送数据
```

## API 说明

- `init()`：初始化屏幕
- `set_row(start, end)`：设置行地址
- `set_col(start, end)`：设置列地址
- `write_memory()`：写入显存
- `write_data(data)`：写入数据

## 依赖

- [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)
- [`embedded-hal-async`](https://github.com/embassy-rs/embedded-hal-async)
- [`defmt`](https://github.com/knurling-rs/defmt)

## 许可证

MIT License
