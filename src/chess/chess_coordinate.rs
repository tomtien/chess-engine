use crate::chess::InvalidNotationError;

pub fn notation_to_idx(notation: &str) -> Result<u16, InvalidNotationError> {
    let parts = notation.as_bytes();
    if parts.len() < 2 || parts.len() > 2 {
        return Err(InvalidNotationError);
    }

    let numeric_part = parts[1] as char;
    let letter_part = parts[0] as char;

    let x = match letter_part {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(InvalidNotationError),
    };
    let y = match numeric_part {
        '1' => 7,
        '2' => 6,
        '3' => 5,
        '4' => 4,
        '5' => 3,
        '6' => 2,
        '7' => 1,
        '8' => 0,
        _ => return Err(InvalidNotationError),
    };

    let sq_idx = y * 8 + x;
    Ok(sq_idx)
}
pub fn idx_to_notation(idx: u16) -> String {
    let x = idx % 8;
    let y = idx / 8;

    let letter_part = match x {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => ' ',
    };

    let numeric_part = match y {
        0 => '8',
        1 => '7',
        2 => '6',
        3 => '5',
        4 => '4',
        5 => '3',
        6 => '2',
        7 => '1',
        _ => ' ',
    };

    let mut notation = String::from(letter_part);
    notation.push(numeric_part);
    notation
}
