#[test]
fn is_hanging_test() {
    use crate::connect4::bitboard::Bitboard;

    assert!(!Bitboard::EMPTY.is_hanging());
    assert!(!Bitboard::FULL.is_hanging());
    assert!(!Bitboard::BOTTOM_ROW.is_hanging());
    assert!(!Bitboard::RIGHTMOST_COLUMN.is_hanging());
    assert!(Bitboard::NTH_ROW[1].is_hanging());

    let bb = Bitboard::from_array([
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 1],
        [0, 1, 0, 1, 0, 0, 1],
        [0, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 0, 0, 1],
    ]);
    assert!(!bb.is_hanging());

    let bb = Bitboard::from_array([
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 0],
    ]);
    assert!(bb.is_hanging());

    let bb = Bitboard::from_array([
        [0, 1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
    ]);
    assert!(bb.is_hanging());
}
