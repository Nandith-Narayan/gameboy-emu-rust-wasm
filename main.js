import init, {
    greet
}
from "./pkg/gameboy_emu_wasm.js";

await init("./pkg/gameboy_emu_wasm_bg.wasm");

const runWasm = async() => {

   greet();

};