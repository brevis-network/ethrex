//! Pico Ethrex stateless validator guest program.

#![no_main]

use ere_platform_pico::{PicoPlatform, pico_sdk};
use ethrex_stateless_validator::platform::entrypoint;

pico_sdk::entrypoint!(main);

fn main() {
    entrypoint::<PicoPlatform>();
}
