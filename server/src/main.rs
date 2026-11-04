#![no_std]
#![no_main]

//! 串口持有者与控制台服务的薄入口。
//! 唯一入口是 `uart_server::serve`；本 bin 只做公共 `mold`/`execution` 胶水。

#[mold::entry]
fn main() -> Result<(), driver_shared::fail::Fail> {
    uart_server::serve()
}
