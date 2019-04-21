
import * as rust from './pkg/wasm_demo.js';

async function run() {
    await rust.default('./pkg/wasm_demo_bg.wasm');

    testSlice();

    testString();

    testJsValue();

    //rust.run();
}

function testSlice() {
    const a = new Float32Array([1.1, 2.2]);
    const count = rust.numberSlice(a);
    console.log('count: ', count === a.length);

    const val = rust.updateNumSlice(a, 0, 3.3);
    console.log(`val: ${val}`, Math.abs(val-3.3)<1e-7);
}

function testString() {
    const name = 'Jeff';
    const hello = rust.passString(name);
    console.log(hello);
}

function testJsValue() {
    const obj = {
        name: 'Jeff',
        props: {
            'hello': 1
        }
    };
    rust.receiveJs(obj);

    const obj2 = rust.sendRustStruct();
    console.log('From Rust Serde: ', obj2);
}


// run demo
run();