use c4board::{bitboard::Bitboard, board::Board};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// The `alert` browser API
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/board.js")]
extern "C" {
    fn set_checker_color(row: usize, col: usize, color: &str);
}

#[wasm_bindgen]
pub fn show_board() {
    let document = web_sys::window().unwrap().document().unwrap();

    let par = document.create_element("p").unwrap();
    par.set_text_content(Some("This is the value of the paragraph. It has been inserted into the DOM through webassembly."));

    document
        .get_element_by_id("board")
        .unwrap()
        .replace_children_with_node_1(&par);

    // let button = document.get_element_by_id("boring-button").unwrap();

    let board = Board::from_c4e("y;;yyryry;yyrryr;ryyrrr;;").unwrap();

    for (row_idx, row_mask) in Bitboard::NTH_ROW.iter().enumerate() {
        for (col_idx, col_mask) in Bitboard::NTH_COLUMN.iter().enumerate() {
            let mask = *col_mask & *row_mask;

            let cell_is_yellow = !(board.yellow() & mask).is_empty();
            let cell_is_red = !(board.red() & mask).is_empty();

            let color_string = match (cell_is_yellow, cell_is_red) {
                (false, false) | (true, true) => "",
                (true, false) => "yellow",
                (false, true) => "red",
            };

            set_checker_color(row_idx, col_idx, color_string);
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name.to_uppercase()));
}
