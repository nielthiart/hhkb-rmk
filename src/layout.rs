// Generated from the matrix transform in hhkb-zmk's hhkb_pro2.overlay.
// Index with the electrical [row][col] the scanner selects; the value is the
// logical (row, col) used by the keymap and vial.json.
#[rustfmt::skip]
pub const MATRIX_TO_LAYOUT: [[Option<(u8, u8)>; 8]; 8] = [
    [Some((0, 2)), Some((1, 1)), Some((1, 2)), Some((2, 2)), Some((2, 1)), Some((3, 1)), Some((3, 2)), Some((3, 3))],
    [Some((0, 3)), Some((0, 4)), Some((1, 4)), Some((1, 3)), Some((2, 3)), Some((2, 4)), Some((3, 4)), Some((3, 5))],
    [Some((0, 5)), Some((0, 6)), Some((1, 6)), Some((1, 5)), Some((2, 5)), Some((2, 6)), Some((3, 6)), None],
    [Some((0, 1)), Some((0, 0)), Some((1, 0)), Some((2, 0)), Some((3, 0)), Some((4, 0)), Some((4, 1)), Some((4, 2))],
    [Some((0, 7)), Some((0, 8)), Some((1, 7)), Some((1, 8)), Some((2, 8)), Some((2, 7)), Some((3, 7)), None],
    [Some((0, 13)), Some((0, 14)), Some((1, 13)), Some((2, 12)), Some((3, 12)), Some((3, 11)), Some((4, 4)), Some((4, 3))],
    [Some((0, 9)), Some((0, 10)), Some((1, 9)), Some((1, 10)), Some((2, 10)), Some((2, 9)), Some((3, 8)), None],
    [Some((0, 11)), Some((0, 12)), Some((1, 12)), Some((1, 11)), Some((2, 11)), Some((3, 10)), Some((3, 9)), None],
];
