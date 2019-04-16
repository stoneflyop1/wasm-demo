/// <reference path="./pkg/wasm_demo.d.ts"></reference>

window.addEventListener('load', () => {
    window.wasm_bindgen('./pkg/wasm_demo_bg.wasm').then(()=>{
        const a = new Float32Array([1.1, 2.2]);
        var count = window.wasm_bindgen.numberSlice(a);
        console.log('count: ', count == a.length);
        var val = window.wasm_bindgen.updateNumSlice(a, 0, 3.3);
        console.log(`val: ${val}`, Math.abs(val-3.3)<1e-7);
    });
});