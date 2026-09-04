// It puzzles me that this is the correct way to include a css file in a webpack
// project. One would think that it should be added in index.html or webpack.config.js
import "./style.css";

// Include things from the rust wasm package
import { show_board } from './pkg';

// TODO: Consider replacing this with wasm_bindgen::start (or whatever it's actually called)
show_board();
