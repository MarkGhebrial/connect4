#[test]
fn is_hanging_test() {
    use crate::bitboard::Bitboard;

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

#[test]
fn has_four_in_a_row_test() {
    use crate::bitboard::Bitboard;

    assert!(!Bitboard::EMPTY.has_four_in_a_row());
    assert!(Bitboard::FULL.has_four_in_a_row());
    assert!(Bitboard::TOP_ROW.has_four_in_a_row());
    assert!(Bitboard::BOTTOM_ROW.has_four_in_a_row());
    assert!(Bitboard::RIGHTMOST_COLUMN.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 1, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 0, 0],
    ]);
    assert!(bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(!bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1, 0, 1],
        [0, 1, 0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1, 0, 1],
        [1, 0, 1, 0, 1, 0, 1],
    ]);
    assert!(!bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [1, 1, 1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 1, 0],
        [0, 1, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 1, 1, 1],
    ]);
    assert!(!bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
    ]);
    assert!(bb.has_four_in_a_row());

    let bb = Bitboard::from_array([
        [0, 1, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ]);
    assert!(bb.has_four_in_a_row());
}

#[test]
fn is_top_cell_filled_test() {
    use crate::bitboard::Bitboard;
    use crate::bitboard::NUM_COLUMNS;

    for i in 0..NUM_COLUMNS {
        println!("Checking column {i}");
        assert!(
            !Bitboard::EMPTY.is_top_cell_filled(i),
            "Empty board should have no filled top cells."
        );
        assert!(
            Bitboard::FULL.is_top_cell_filled(i),
            "Full board should have no empty top cells."
        );
    }

    // TODO: Add some more test cases
}
