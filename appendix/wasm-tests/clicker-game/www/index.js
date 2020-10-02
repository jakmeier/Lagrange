import * as wasm from "clicker-game";
import "./styles.css";

// window.buy = function() {
//     wasm.buy();
// }

// wasm.init();

window.buy = function() {
    wasm.safe_buy();
}

wasm.safe_init();