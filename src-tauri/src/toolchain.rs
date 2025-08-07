use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Toolchain {
    Native,
    MinGW,
    AndroidNDK,
    Emscripten,
    WasmPack,
    Custom(String),
}

impl Toolchain {
    pub fn command_prefix(&self) -> Vec<String> {
        match self {
            Toolchain::Native => vec![],
            Toolchain::MinGW => vec!["x86_64-w64-mingw32-".to_string()],
            Toolchain::AndroidNDK => vec!["$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/".to_string()],
            Toolchain::Emscripten => vec!["emcc".to_string()],
            Toolchain::WasmPack => vec!["wasm-pack".to_string()],
            Toolchain::Custom(prefix) => vec![prefix.clone()],
        }
    }
}
