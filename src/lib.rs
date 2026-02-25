pub mod assembler;
pub mod cpu;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub mod app;

pub use assembler::{AssemblyError, AssemblyOutput, assemble};
pub use cpu::{Cpu, CpuError};

#[cfg(target_arch = "wasm32")]
pub use wasm::{RegisterState, WasmCpu};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    // Set up console error panic hook for better error messages
    console_error_panic_hook::set_once();

    // Mount the Yew app
    yew::Renderer::<app::App>::new().render();
}
