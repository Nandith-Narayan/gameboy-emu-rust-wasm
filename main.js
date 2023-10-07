import init, {
    run_until_frame_end,
    initialize_rom
}
from "./pkg/gameboy_emu_wasm.js";

let data = []

await init("./pkg/gameboy_emu_wasm_bg.wasm");

const runWasm = async() => {

   run_until_frame_end();

};


const fileSelector = document.getElementById('rom-select');
fileSelector.addEventListener('change', (event) => {
    let rom = event.target.files[0];
    let reader = new FileReader();
    reader.onload = function (e) {
        // binary data
        data = new Uint8Array(e.target.result);
        initialize_rom(data);
        runWasm();
    };
    reader.onerror = function (e) {
        // error occurred
        console.log('Error : ' + e.type);
    };
    reader.readAsArrayBuffer(rom);
});