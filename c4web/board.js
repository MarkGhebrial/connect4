/// Reach into the gross, sticky DOM to set the color of one of the cells on the
/// board.
export function set_checker_color(row, col, color) {
    let board = document.getElementsByClassName("container")[0];
    let column = board.children[6 - col];
    let cell = column.children[5 - row];
    
    if (color === "yellow") {
        cell.classList.add("yellow");
    }
    else if (color === "red") {
        cell.classList.add("red");
    }
    else {
        cell.classList.remove("yellow");
        cell.classList.remove("red");
    }
}