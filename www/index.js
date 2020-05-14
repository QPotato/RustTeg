import * as wasm from "rusteg";

const pre = document.getElementById("rusteg-canvas");

const render = () => {
    const game = wasm.start();
    console.log(game);
    console.log(game.get_map())
    pre.textContent = JSON.stringify(game.get_map());

};

render()