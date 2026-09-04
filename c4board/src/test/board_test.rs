#[test]
fn is_valid_test() {
    todo!("Test for Board::is_valid() is not implemented yet");
}

#[test]
fn iter_moves_test() {
    todo!("Test for Board::iter_moves() is not implemented yet");
}

#[test]
fn from_c4e_test() {
    use crate::board::Board;

    let board = Board::from_c4e("y;;yyryry;yyrryr;ryyrrr;;").unwrap();
    assert!(board.is_valid());
}
