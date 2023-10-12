import init, {
    run_until_frame_end,
    initialize_rom,
    get_background_tile_data
}
from "./pkg/gameboy_emu_wasm.js";

let data = []

await init("./pkg/gameboy_emu_wasm_bg.wasm");

let ctx = document.getElementById("canvas").getContext("2d");
let ctx_bg = document.getElementById("background-map-canvas").getContext("2d");
const runWasm = async() => {
    // Force 60 fps, even if monitor renders at a higher fps
    setTimeout(function () {

        requestAnimationFrame(runWasm);


        let frame_data = run_until_frame_end();

        for (let y = 0; y < 144; y++) {
            for (let x = 0; x < 160; x++) {

                let base_addr = (y * 160 + x) * 3
                let r = frame_data[base_addr];
                let g = frame_data[base_addr + 1];
                let b = frame_data[base_addr + 2];

                ctx.fillStyle = "rgb(" + r + ", " + g + ", " + b + ")";
                ctx.fillRect(x * 4, y * 4, 4, 4);

            }
        }

        let debug_frame_data = get_background_tile_data();

        for (let y = 0; y < 256; y++) {
            for (let x = 0; x < 256; x++) {

                let base_addr = (y * 256 + x)
                let r = debug_frame_data[base_addr];

                ctx_bg.fillStyle = "rgb(" + r + ", " + r + ", " + r + ")";
                ctx_bg.fillRect(x, y, 1, 1);

            }
        }

    }, 16);
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
