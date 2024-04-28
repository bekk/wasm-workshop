# Task 7 - Snake in Rust/WASM

This task is about implementing Snake in Javascript and Rust running in WASM. 

This task is open-ended and intended to let you explore Rust in WASM using a familiar game. Focus on which ever features you'd like :)

Some code is provided to get you started: 
- A HTML canvas is used to render a grid
- A simple one-cell snake is implemented and can be moved around using the arrow keys
- A game loop is implemented in JavaScript, responsible for updating the game state and drawing the game on the canvas.
- The game state is located in Rust, along with functions for updating the game state. 

Some things that are not implemented:
- Food
- The ability for the snake to grow beyond one cell

## Running the code

1. Install Rust: If you haven't already, install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).
2. Install `wasm-pack` by running `cargo install wasm-pack`. This installs the `wasm-pack` tool globally, which helps compile the Rust code to Webassembly.
4. Ensure that your working directory is inside the folder `tasks/7` before proceeding
3. Compile the Rust code to a WASM module by running `wasm-pack build --target bundler rust-snake`
4. Install npm dependencies with `npm i`
5. Serve the web application using `npm run serve`. 
6. Open localhost:8080 in your browser and you should be good to go :) Try controlling the snake with your arrow keys.

## Notes

- The JavaScript part of the application resides in the `index.js` file
- The Rust part of the application resides in the `rust-snake/src/lib.rs` file.
- Webpack will hot reload changes in the `index.js` and `index.html` files, but not changes in the Rust code. This requires recompiling the code (see step 3 above).